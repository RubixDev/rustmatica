use std::ops::RangeInclusive;

use fastnbt::LongArray;
use mcdata::{
    util::{BlockPos, UVec3},
    GenericBlockEntity, GenericBlockState, GenericEntity,
};
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
            blocks: vec![0; size.volume()],
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

    fn pos_to_index(&self, pos: UVec3) -> usize {
        let size = self.size.abs();
        (pos.x + pos.y * size.z * size.x + pos.z * size.z) as usize
    }

    /// Get the block state at the given position within this region.
    pub fn get_block(&self, pos: UVec3) -> &BlockState {
        &self.palette[self.blocks[self.pos_to_index(pos)]]
    }

    /// Set the block state at the given position within this region.
    pub fn set_block(&mut self, pos: UVec3, block: BlockState) {
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
        self.position.x.min(self.position.x + self.size.x)
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

    /// Calculate the minimum x coordinate of this region in the regions's coordinate system.
    pub fn min_x(&self) -> u32 {
        0
    }

    /// Calculate the maximum x coordinate of this region in the regions's coordinate system.
    pub fn max_x(&self) -> u32 {
        0.max(self.size.abs().x - 1)
    }

    /// Calculate the minimum y coordinate of this region in the regions's coordinate system.
    pub fn min_y(&self) -> u32 {
        0
    }

    /// Calculate the maximum y coordinate of this region in the regions's coordinate system.
    pub fn max_y(&self) -> u32 {
        0.max(self.size.abs().y - 1)
    }

    /// Calculate the minimum z coordinate of this region in the regions's coordinate system.
    pub fn min_z(&self) -> u32 {
        0
    }

    /// Calculate the maximum z coordinate of this region in the regions's coordinate system.
    pub fn max_z(&self) -> u32 {
        0.max(self.size.abs().z - 1)
    }

    /// Get the range of possible x coordinates within this region.
    pub fn x_range(&self) -> RangeInclusive<u32> {
        self.min_x()..=self.max_x()
    }

    /// Get the range of possible y coordinates within this region.
    pub fn y_range(&self) -> RangeInclusive<u32> {
        self.min_y()..=self.max_y()
    }

    /// Get the range of possible z coordinates within this region.
    pub fn z_range(&self) -> RangeInclusive<u32> {
        self.min_z()..=self.max_z()
    }

    /// Calculate the total count of non-air blocks in this region.
    pub fn total_blocks(&self) -> usize {
        self.blocks.iter().filter(|b| b != &&0).count()
    }

    /// Create an iterator over all blocks in this region.
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
#[derive(Debug)]
pub struct Blocks<'b, BlockState>
where
    BlockState: mcdata::BlockState,
{
    palette: &'b Vec<BlockState>,
    blocks: &'b Vec<usize>,
    index: usize,
    size: UVec3,
}

impl<'b, BlockState> Iterator for Blocks<'b, BlockState>
where
    BlockState: mcdata::BlockState,
{
    type Item = (UVec3, &'b BlockState);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.volume() {
            return None;
        }
        let block = self.palette.get(*self.blocks.get(self.index)?)?;
        let x = self.index as u32 % self.size.x;
        let y = self.index as u32 / self.size.z / self.size.y % self.size.y;
        let z = self.index as u32 / self.size.z % self.size.z;
        self.index += 1;
        Some((UVec3::new(x, y, z), block))
    }
}
