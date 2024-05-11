use std::ops::RangeInclusive;

use fastnbt::LongArray;
use mcdata::{util::BlockPos, GenericBlockEntity, GenericBlockState, GenericEntity};
use serde::{de::DeserializeOwned, Serialize};

use crate::{schema, Litematic, PendingBlockTick, PendingFluidTick};

type CowStr = std::borrow::Cow<'static, str>;

/// A single region of a litematica schematic.
///
/// Just like [`Litematic`], the type has three generic type parameters for the types of block
/// states, entities, and block entities. These types must implement the corresponding [`mcdata`]
/// trait and default to [`mcdata`]s "generic" types.
///
/// A region has a [name](Self::name), [position](Self::position), and [size](Self::size), as well
/// as a list of [entities](Self::entities), a list of [block entities](Self::block_entities), and
/// an internal representation for storing block states of all contained positions. To interact
/// with the block states, see the [`get_block`](Self::get_block), [`set_block`](Self::set_block),
/// and [`blocks`](Self::blocks) functions.
#[derive(Debug, Clone, PartialEq)]
pub struct Region<
    BlockState = GenericBlockState,
    Entity = GenericEntity,
    BlockEntity = GenericBlockEntity,
> where
    BlockState: mcdata::BlockState + Serialize + DeserializeOwned,
    Entity: mcdata::Entity + Serialize + DeserializeOwned,
    BlockEntity: mcdata::BlockEntity + Serialize + DeserializeOwned,
{
    /// The name of this region.
    pub name: CowStr,

    /// The position of this region within the schematic.
    pub position: BlockPos,

    /// The size of this region.
    ///
    /// Note that this size is allowed to be negative. The actual size is always the absolute value
    /// of this, but a negative size can indicate that the region grows in the negative direction
    /// from [`Self::position`]. Local coordinates in regions aren't directly relative to
    /// [`Self::position`], but to the minimum position in the region, effectively to
    /// `BlockPos::new(region.min_global_x(), region.min_global_y(), region.min_global_z())`.
    pub size: BlockPos,

    /// The list of block entities in this region.
    pub block_entities: Vec<BlockEntity>,

    /// The list of entities in this region.
    pub entities: Vec<Entity>,

    /// Pending block ticks in this region.
    pub pending_block_ticks: Vec<PendingBlockTick>,

    /// Pending fluid ticks in this region.
    pub pending_fluid_ticks: Vec<PendingFluidTick>,

    palette: Vec<BlockState>,
    blocks: Vec<usize>,
}

impl<BlockState, Entity, BlockEntity> Region<BlockState, Entity, BlockEntity>
where
    BlockState: mcdata::BlockState + Serialize + DeserializeOwned,
    Entity: mcdata::Entity + Serialize + DeserializeOwned,
    BlockEntity: mcdata::BlockEntity + Serialize + DeserializeOwned,
{
    /// Create a new, empty region with the given name, position, and size.
    pub fn new(name: impl Into<CowStr>, position: BlockPos, size: BlockPos) -> Self {
        Self {
            name: name.into(),
            position,
            size,
            block_entities: vec![],
            entities: vec![],
            pending_block_ticks: vec![],
            pending_fluid_ticks: vec![],
            palette: vec![BlockState::air()],
            blocks: vec![0; size.volume() as usize],
        }
    }

    /// Construct a [`Region`] from a [raw NBT region](schema::Region) with the given name.
    pub(crate) fn from_raw(
        raw: schema::Region<BlockState, Entity, BlockEntity>,
        name: impl Into<CowStr>,
    ) -> Self {
        fn inner<
            B: mcdata::BlockState + Serialize + DeserializeOwned,
            E: mcdata::Entity + Serialize + DeserializeOwned,
            T: mcdata::BlockEntity + Serialize + DeserializeOwned,
        >(
            raw: schema::Region<B, E, T>,
            name: CowStr,
        ) -> Region<B, E, T> {
            let mut new = Region::new(name, raw.position, raw.size);
            new.palette = raw.block_state_palette.to_owned();
            new.block_entities = raw.tile_entities.to_owned();
            new.entities = raw.entities.to_owned();
            new.pending_block_ticks = raw.pending_block_ticks.clone();
            new.pending_fluid_ticks = raw.pending_fluid_ticks.clone();

            let num_bits = new.num_bits();
            new.blocks = raw
                .block_states
                .iter()
                .flat_map(|block| (0..64).map(move |bit| block >> bit & 1))
                .collect::<Vec<i64>>()
                .chunks(num_bits)
                .map(|slice| {
                    slice
                        .iter()
                        .rev()
                        .fold(0, |acc, bit| acc << 1 | *bit as usize)
                })
                .collect::<Vec<usize>>();

            new
        }
        inner(raw, name.into())
    }

    /// Create a new [raw NBT region](schema::Region) from this [`Region`].
    pub(crate) fn to_raw(&self) -> schema::Region<BlockState, Entity, BlockEntity> {
        let mut new = schema::Region {
            position: self.position,
            size: self.size,
            block_state_palette: self.palette.to_owned(),
            tile_entities: self.block_entities.to_owned(),
            entities: self.entities.to_owned(),
            pending_block_ticks: self.pending_block_ticks.clone(),
            pending_fluid_ticks: self.pending_fluid_ticks.clone(),
            block_states: LongArray::new(vec![]),
        };

        let num_bits = self.num_bits();
        new.block_states = LongArray::new(
            self.blocks
                .iter()
                .flat_map(|id| (0..num_bits).map(move |bit| id >> bit & 1))
                .collect::<Vec<usize>>()
                .chunks(64)
                .map(|bits| bits.iter().rev().fold(0, |acc, bit| acc << 1 | *bit as i64))
                .collect(),
        );

        new
    }

    fn num_bits(&self) -> usize {
        let mut num_bits = 2;
        while 1 << num_bits < self.palette.len() {
            num_bits += 1;
        }
        num_bits
    }

    fn pos_to_index(&self, pos: BlockPos) -> usize {
        let size = self.size.abs();
        let pos = pos.abs();
        (pos.x + pos.y * size.x * size.z + pos.z * size.x) as usize
    }

    fn assert_bounds(&self, pos: BlockPos) {
        assert!(
            self.x_range().contains(&pos.x),
            "x position '{}' out of bounds for {:?}",
            pos.x,
            self.x_range(),
        );
        assert!(
            self.y_range().contains(&pos.y),
            "y position '{}' out of bounds for {:?}",
            pos.y,
            self.y_range(),
        );
        assert!(
            self.z_range().contains(&pos.z),
            "z position '{}' out of bounds for {:?}",
            pos.z,
            self.z_range(),
        );
    }

    /// Returns whether the local position is inside this region.
    ///
    /// For global coordinates, use [`Self::is_in_global_bounds`] instead.
    pub fn is_in_bounds(&self, pos: BlockPos) -> bool {
        self.x_range().contains(&pos.x)
            && self.y_range().contains(&pos.y)
            && self.z_range().contains(&pos.z)
    }

    /// Get the block state at the given position within this region.
    ///
    /// Panics if the position is not inside this region.
    /// See [`Self::get_block_opt`] for a non-panicking version.
    ///
    /// For global coordinates, use [`Self::get_block_global`] instead.
    pub fn get_block(&self, pos: BlockPos) -> &BlockState {
        self.assert_bounds(pos);
        &self.palette[self.blocks[self.pos_to_index(pos)]]
    }

    /// Set the block state at the given position within this region.
    ///
    /// Panics if the position is not inside this region.
    /// See [`Self::set_block_opt`] for a non-panicking version.
    ///
    /// For global coordinates, use [`Self::set_block_global`] instead.
    pub fn set_block(&mut self, pos: BlockPos, block: BlockState) {
        self.assert_bounds(pos);
        let block = block;
        let id = if let Some(pos) = self.palette.iter().position(|b| b == &block) {
            pos
        } else {
            self.palette.push(block);
            self.palette.len() - 1
        };
        let pos = self.pos_to_index(pos);
        self.blocks[pos] = id;
    }

    /// Get the block state at the given position within this region.
    ///
    /// Returns `None` if the position is not inside this region.
    /// See [`Self::get_block`] for a panicking version.
    ///
    /// For global coordinates, use [`Self::get_block_global_opt`] instead.
    pub fn get_block_opt(&self, pos: BlockPos) -> Option<&BlockState> {
        self.is_in_bounds(pos)
            .then(|| &self.palette[self.blocks[self.pos_to_index(pos)]])
    }

    /// Set the block state at the given position within this region.
    ///
    /// Returns `false` and does nothing if the position is not inside this region.
    /// See [`Self::set_block`] for a panicking version.
    ///
    /// For global coordinates, use [`Self::set_block_global_opt`] instead.
    pub fn set_block_opt(&mut self, pos: BlockPos, block: BlockState) -> bool {
        self.is_in_bounds(pos)
            .then(|| self.set_block(pos, block))
            .is_some()
    }

    /// Converts a position from the region-local to the global coordinate system.
    ///
    /// Panics if the position is not inside this region.
    pub fn pos_to_global(&self, pos: BlockPos) -> BlockPos {
        self.assert_bounds(pos);
        BlockPos {
            x: pos.x + self.min_global_x(),
            y: pos.y + self.min_global_y(),
            z: pos.z + self.min_global_z(),
        }
    }

    /// Converts a position from the global to the region-local coordinate system.
    ///
    /// Panics if the position is not inside this region.
    pub fn pos_from_global(&self, pos: BlockPos) -> BlockPos {
        self.assert_global_bounds(pos);
        BlockPos {
            x: pos.x - self.min_global_x(),
            y: pos.y - self.min_global_y(),
            z: pos.z - self.min_global_z(),
        }
    }

    fn assert_global_bounds(&self, pos: BlockPos) {
        assert!(
            self.global_x_range().contains(&pos.x),
            "x position '{}' out of bounds for {:?}",
            pos.x,
            self.global_x_range(),
        );
        assert!(
            self.global_y_range().contains(&pos.y),
            "y position '{}' out of bounds for {:?}",
            pos.y,
            self.global_y_range(),
        );
        assert!(
            self.global_z_range().contains(&pos.z),
            "z position '{}' out of bounds for {:?}",
            pos.z,
            self.global_z_range(),
        );
    }

    /// Returns whether the global position is inside this region.
    ///
    /// For local coordinates, use [`Self::is_in_bounds`] instead.
    pub fn is_in_global_bounds(&self, pos: BlockPos) -> bool {
        self.global_x_range().contains(&pos.x)
            && self.global_y_range().contains(&pos.y)
            && self.global_z_range().contains(&pos.z)
    }

    /// Get the block state at the given global position.
    ///
    /// Panics if the position is not inside this region.
    /// See [`Self::get_block_global_opt`] for a non-panicking version.
    ///
    /// For local coordinates, use [`Self::get_block`] instead.
    pub fn get_block_global(&self, pos: BlockPos) -> &BlockState {
        self.assert_global_bounds(pos);
        &self.palette[self.blocks[self.pos_to_index(self.pos_from_global(pos))]]
    }

    /// Set the block state at the given global position.
    ///
    /// Panics if the position is not inside this region.
    /// See [`Self::set_block_global_opt`] for a non-panicking version.
    ///
    /// For local coordinates, use [`Self::set_block`] instead.
    pub fn set_block_global(&mut self, pos: BlockPos, block: BlockState) {
        self.assert_global_bounds(pos);
        self.set_block(self.pos_from_global(pos), block);
    }

    /// Get the block state at the given global position.
    ///
    /// Returns `None` if the position is not inside this region.
    /// See [`Self::get_block_global`] for a panicking version.
    ///
    /// For local coordinates, use [`Self::get_block_opt`] instead.
    pub fn get_block_global_opt(&self, pos: BlockPos) -> Option<&BlockState> {
        self.is_in_global_bounds(pos)
            .then(|| &self.palette[self.blocks[self.pos_to_index(self.pos_from_global(pos))]])
    }

    /// Set the block state at the given global position.
    ///
    /// Returns `false` and does nothing if the position is not inside this region.
    /// See [`Self::set_block_global`] for a panicking version.
    ///
    /// For local coordinates, use [`Self::set_block_opt`] instead.
    pub fn set_block_global_opt(&mut self, pos: BlockPos, block: BlockState) -> bool {
        self.is_in_global_bounds(pos)
            .then(|| self.set_block(self.pos_from_global(pos), block))
            .is_some()
    }

    /// Get a reference to the current block state palette.
    pub fn block_palette(&self) -> &[BlockState] {
        &self.palette
    }

    /// Find a block entity by its position.
    pub fn get_block_entity(&self, pos: BlockPos) -> Option<&BlockEntity> {
        self.block_entities.iter().find(|e| e.position() == pos)
    }

    /// Replace or add a block entity.
    ///
    /// Returns the previous block entity if there already was one at the same position.
    pub fn set_block_entity(&mut self, block_entity: BlockEntity) -> Option<BlockEntity> {
        if let Some(prev) = self
            .block_entities
            .iter_mut()
            .find(|e| e.position() == block_entity.position())
        {
            Some(std::mem::replace(prev, block_entity))
        } else {
            self.block_entities.push(block_entity);
            None
        }
    }

    /// Removes the block entity at the given position.
    ///
    /// Returns the removed block entity if there was one.
    pub fn remove_block_entity(&mut self, pos: BlockPos) -> Option<BlockEntity> {
        self.block_entities
            .iter()
            .position(|e| e.position() == pos)
            .map(|idx| self.block_entities.swap_remove(idx))
    }

    /// Calculate the minimum x coordinate of this region in the schematic's coordinate system.
    pub fn min_global_x(&self) -> i32 {
        self.position.x.min(self.position.x + self.size.x + 1)
    }

    /// Calculate the maximum x coordinate of this region in the schematic's coordinate system.
    pub fn max_global_x(&self) -> i32 {
        self.position.x.max(self.position.x + self.size.x - 1)
    }

    /// Calculate the minimum y coordinate of this region in the schematic's coordinate system.
    pub fn min_global_y(&self) -> i32 {
        self.position.y.min(self.position.y + self.size.y + 1)
    }

    /// Calculate the maximum y coordinate of this region in the schematic's coordinate system.
    pub fn max_global_y(&self) -> i32 {
        self.position.y.max(self.position.y + self.size.y - 1)
    }

    /// Calculate the minimum z coordinate of this region in the schematic's coordinate system.
    pub fn min_global_z(&self) -> i32 {
        self.position.z.min(self.position.z + self.size.z + 1)
    }

    /// Calculate the maximum z coordinate of this region in the schematic's coordinate system.
    pub fn max_global_z(&self) -> i32 {
        self.position.z.max(self.position.z + self.size.z - 1)
    }

    /// Get the range of possible global x coordinates in this region.
    pub fn global_x_range(&self) -> RangeInclusive<i32> {
        self.min_global_x()..=self.max_global_x()
    }

    /// Get the range of possible global y coordinates in this region.
    pub fn global_y_range(&self) -> RangeInclusive<i32> {
        self.min_global_y()..=self.max_global_y()
    }

    /// Get the range of possible global z coordinates in this region.
    pub fn global_z_range(&self) -> RangeInclusive<i32> {
        self.min_global_z()..=self.max_global_z()
    }

    /// Calculate the minimum x coordinate of this region in the regions's coordinate system.
    pub fn min_x(&self) -> i32 {
        0
    }

    /// Calculate the maximum x coordinate of this region in the regions's coordinate system.
    pub fn max_x(&self) -> i32 {
        0.max(self.size.x.abs() - 1)
    }

    /// Calculate the minimum y coordinate of this region in the regions's coordinate system.
    pub fn min_y(&self) -> i32 {
        0
    }

    /// Calculate the maximum y coordinate of this region in the regions's coordinate system.
    pub fn max_y(&self) -> i32 {
        0.max(self.size.y.abs() - 1)
    }

    /// Calculate the minimum z coordinate of this region in the regions's coordinate system.
    pub fn min_z(&self) -> i32 {
        0
    }

    /// Calculate the maximum z coordinate of this region in the regions's coordinate system.
    pub fn max_z(&self) -> i32 {
        0.max(self.size.z.abs() - 1)
    }

    /// Get the range of possible x coordinates within this region.
    pub fn x_range(&self) -> RangeInclusive<i32> {
        self.min_x()..=self.max_x()
    }

    /// Get the range of possible y coordinates within this region.
    pub fn y_range(&self) -> RangeInclusive<i32> {
        self.min_y()..=self.max_y()
    }

    /// Get the range of possible z coordinates within this region.
    pub fn z_range(&self) -> RangeInclusive<i32> {
        self.min_z()..=self.max_z()
    }

    /// Count the number of non-air blocks in this region.
    pub fn total_blocks(&self) -> usize {
        self.blocks.iter().filter(|b| b != &&0).count()
    }

    /// Create an iterator over all blocks in this region.
    ///
    /// Each item will be a tuple of the local coordinates of this block, and the block state
    /// itself.
    pub fn blocks(&self) -> Blocks<'_, BlockState> {
        Blocks {
            palette: &self.palette,
            blocks: &self.blocks,
            index: 0,
            size: self.size.abs(),
        }
    }

    /// Create a new [`Litematic`] from this [`Region`] with a given description and author.
    ///
    /// The created schematic will have the same name as this region and will include this region
    /// as its only region.
    pub fn as_litematic(
        self,
        description: impl Into<CowStr>,
        author: impl Into<CowStr>,
    ) -> Litematic<BlockState, Entity, BlockEntity> {
        let mut l = Litematic::new(self.name.clone(), description, author);
        l.regions.push(self);
        l
    }
}

/// An iterator over all blocks in a [`Region`].
///
/// Each item will be a tuple of the local coordinates of this block, and the block state itself.
#[derive(Debug)]
pub struct Blocks<'b, BlockState>
where
    BlockState: mcdata::BlockState,
{
    palette: &'b Vec<BlockState>,
    blocks: &'b Vec<usize>,
    index: usize,
    size: BlockPos,
}

impl<'b, BlockState> Iterator for Blocks<'b, BlockState>
where
    BlockState: mcdata::BlockState,
{
    type Item = (BlockPos, &'b BlockState);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.volume() as usize {
            return None;
        }
        let block = self.palette.get(*self.blocks.get(self.index)?)?;
        let x = self.index as i32 % self.size.x;
        let y = self.index as i32 / self.size.x / self.size.z % self.size.y;
        let z = self.index as i32 / self.size.x % self.size.z;
        self.index += 1;
        Some((BlockPos::new(x, y, z), block))
    }
}
