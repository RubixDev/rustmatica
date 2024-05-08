use std::ops::RangeInclusive;

use fastnbt::LongArray;
use mcdata::{
    util::{BlockPos, UVec3},
    GenericBlockEntity, GenericBlockState, GenericEntity,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{schema, Litematic};

type CowStr = std::borrow::Cow<'static, str>;

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
    pub name: CowStr,
    pub position: BlockPos,
    pub size: BlockPos,
    pub block_entities: Vec<BlockEntity>,
    pub entities: Vec<Entity>,
    palette: Vec<BlockState>,
    blocks: Vec<usize>,
}

impl<BlockState, Entity, BlockEntity> Region<BlockState, Entity, BlockEntity>
where
    BlockState: mcdata::BlockState + Serialize + DeserializeOwned,
    Entity: mcdata::Entity + Serialize + DeserializeOwned,
    BlockEntity: mcdata::BlockEntity + Serialize + DeserializeOwned,
{
    pub fn new(name: impl Into<CowStr>, position: BlockPos, size: BlockPos) -> Self {
        Self {
            name: name.into(),
            position,
            size,
            block_entities: vec![],
            entities: vec![],
            palette: vec![BlockState::air()],
            blocks: vec![0; size.volume()],
        }
    }

    pub fn from_raw(
        raw: schema::Region<BlockState, Entity, BlockEntity>,
        name: impl Into<CowStr>,
    ) -> Self {
        let mut new = Self::new(name, raw.position, raw.size);
        new.palette = raw.block_state_palette.to_owned();
        new.block_entities = raw.tile_entities.to_owned();
        new.entities = raw.entities.to_owned();

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

    pub fn to_raw(&self) -> schema::Region<BlockState, Entity, BlockEntity> {
        let mut new = schema::Region {
            position: self.position,
            size: self.size,
            block_state_palette: self.palette.to_owned(),
            tile_entities: self.block_entities.to_owned(),
            entities: self.entities.to_owned(),
            pending_fluid_ticks: vec![],
            pending_block_ticks: vec![],
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

    pub fn get_block(&self, pos: UVec3) -> &BlockState {
        &self.palette[self.blocks[self.pos_to_index(pos)]]
    }

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

    pub fn get_block_entity(&self, pos: BlockPos) -> Option<&BlockEntity> {
        self.block_entities.iter().find(|e| e.position() == pos)
    }

    pub fn set_block_entity(&mut self, block_entity: BlockEntity) {
        if let Some(index) = self
            .block_entities
            .iter()
            .position(|e| e.position() == block_entity.position())
        {
            self.block_entities[index] = block_entity;
        } else {
            self.block_entities.push(block_entity);
        }
    }

    pub fn remove_block_entity(&mut self, pos: BlockPos) {
        if let Some(index) = self.block_entities.iter().position(|e| e.position() == pos) {
            self.block_entities.remove(index);
        }
    }

    pub fn min_global_x(&self) -> i32 {
        self.position.x.min(self.position.x + self.size.x)
    }
    pub fn max_global_x(&self) -> i32 {
        self.position.x.max(self.position.x + self.size.x - 1)
    }
    pub fn min_global_y(&self) -> i32 {
        self.position.y.min(self.position.y + self.size.y + 1)
    }
    pub fn max_global_y(&self) -> i32 {
        self.position.y.max(self.position.y + self.size.y - 1)
    }
    pub fn min_global_z(&self) -> i32 {
        self.position.z.min(self.position.z + self.size.z + 1)
    }
    pub fn max_global_z(&self) -> i32 {
        self.position.z.max(self.position.z + self.size.z - 1)
    }

    pub fn min_x(&self) -> u32 {
        0
    }
    pub fn max_x(&self) -> u32 {
        0.max(self.size.abs().x - 1)
    }
    pub fn min_y(&self) -> u32 {
        0
    }
    pub fn max_y(&self) -> u32 {
        0.max(self.size.abs().y - 1)
    }
    pub fn min_z(&self) -> u32 {
        0
    }
    pub fn max_z(&self) -> u32 {
        0.max(self.size.abs().z - 1)
    }

    pub fn x_range(&self) -> RangeInclusive<u32> {
        self.min_x()..=self.max_x()
    }
    pub fn y_range(&self) -> RangeInclusive<u32> {
        self.min_y()..=self.max_y()
    }
    pub fn z_range(&self) -> RangeInclusive<u32> {
        self.min_z()..=self.max_z()
    }

    pub fn total_blocks(&self) -> usize {
        self.blocks.iter().filter(|b| b != &&0).count()
    }

    pub fn blocks(&self) -> Blocks<'_, BlockState> {
        Blocks {
            palette: &self.palette,
            blocks: &self.blocks,
            index: 0,
            size: self.size.abs(),
        }
    }

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
