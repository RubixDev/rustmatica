use std::{borrow::Cow, ops::RangeInclusive};

use fastnbt::LongArray;

use crate::{
    schema,
    util::{UVec3, Vec3},
    BlockState, Entity, Litematic, TileEntity,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Region<'l> {
    pub name: Cow<'l, str>,
    pub position: UVec3,
    pub size: Vec3,
    pub tile_entities: Vec<TileEntity<'l>>,
    pub entities: Vec<Entity<'l>>,
    palette: Vec<BlockState<'l>>,
    blocks: Vec<usize>,
}

impl<'l> Region<'l> {
    pub fn new(name: Cow<'l, str>, position: UVec3, size: Vec3) -> Self {
        Self {
            name,
            position,
            size,
            tile_entities: vec![],
            entities: vec![],
            palette: vec![block!()],
            blocks: vec![0; size.volume()],
        }
    }

    pub fn from_raw(raw: Cow<'l, schema::Region<'_>>, name: Cow<'l, str>) -> Self {
        let mut new = Self::new(name, raw.position, raw.size);
        new.palette = raw.block_state_palette.to_owned();
        new.tile_entities = raw.tile_entities.to_owned();
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

    pub fn to_raw(&self) -> schema::Region<'_> {
        let mut new = schema::Region {
            position: self.position,
            size: self.size,
            block_state_palette: self.palette.to_owned(),
            tile_entities: self.tile_entities.to_owned(),
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
        pos.x + pos.y * size.z * size.x + pos.z * size.z
    }

    pub fn get_block(&'l self, pos: UVec3) -> &'l BlockState<'_> {
        &self.palette[self.blocks[self.pos_to_index(pos)]]
    }

    pub fn set_block(&mut self, pos: UVec3, block: BlockState<'l>) {
        let id = if let Some(pos) = self.palette.iter().position(|b| b == &block) {
            pos
        } else {
            self.palette.push(block);
            self.palette.len() - 1
        };
        let pos = self.pos_to_index(pos);
        self.blocks[pos] = id;
    }

    pub fn get_tile_entity(&'l self, pos: UVec3) -> Option<&'l TileEntity<'_>> {
        self.tile_entities.iter().find(|e| e.pos == pos)
    }

    pub fn set_tile_entity(&mut self, tile_entity: TileEntity<'l>) {
        if let Some(index) = self
            .tile_entities
            .iter()
            .position(|e| e.pos == tile_entity.pos)
        {
            self.tile_entities[index] = tile_entity;
        } else {
            self.tile_entities.push(tile_entity);
        }
    }

    pub fn remove_tile_entity(&mut self, pos: UVec3) {
        if let Some(index) = self.tile_entities.iter().position(|e| e.pos == pos) {
            self.tile_entities.remove(index);
        }
    }

    pub fn min_global_x(&self) -> usize {
        (self.position.x as i32).min(self.position.x as i32 + self.size.x + 1) as usize
    }
    pub fn max_global_x(&self) -> usize {
        (self.position.x as i32).max(self.position.x as i32 + self.size.x - 1) as usize
    }
    pub fn min_global_y(&self) -> usize {
        (self.position.y as i32).min(self.position.y as i32 + self.size.y + 1) as usize
    }
    pub fn max_global_y(&self) -> usize {
        (self.position.y as i32).max(self.position.y as i32 + self.size.y - 1) as usize
    }
    pub fn min_global_z(&self) -> usize {
        (self.position.z as i32).min(self.position.z as i32 + self.size.z + 1) as usize
    }
    pub fn max_global_z(&self) -> usize {
        (self.position.z as i32).max(self.position.z as i32 + self.size.z - 1) as usize
    }

    pub fn min_x(&self) -> usize {
        0
    }
    pub fn max_x(&self) -> usize {
        0.max(self.size.abs().x - 1)
    }
    pub fn min_y(&self) -> usize {
        0
    }
    pub fn max_y(&self) -> usize {
        0.max(self.size.abs().y - 1)
    }
    pub fn min_z(&self) -> usize {
        0
    }
    pub fn max_z(&self) -> usize {
        0.max(self.size.abs().z - 1)
    }

    pub fn x_range(&self) -> RangeInclusive<usize> {
        self.min_x()..=self.max_x()
    }
    pub fn y_range(&self) -> RangeInclusive<usize> {
        self.min_y()..=self.max_y()
    }
    pub fn z_range(&self) -> RangeInclusive<usize> {
        self.min_z()..=self.max_z()
    }

    pub fn total_blocks(&self) -> usize {
        self.blocks.iter().filter(|b| b != &&0).count()
    }

    pub fn blocks(&'l self) -> Blocks<'l> {
        Blocks {
            palette: &self.palette,
            blocks: &self.blocks,
            index: 0,
            size: self.size.abs(),
        }
    }

    pub fn as_litematic(self, description: Cow<'l, str>, author: Cow<'l, str>) -> Litematic<'l> {
        let mut l = Litematic::new(self.name.clone(), description, author);
        l.regions.push(self);
        l
    }
}

pub struct Blocks<'b> {
    palette: &'b Vec<BlockState<'b>>,
    blocks: &'b Vec<usize>,
    index: usize,
    size: UVec3,
}

impl<'b> Iterator for Blocks<'b> {
    type Item = (UVec3, &'b BlockState<'b>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.volume() {
            return None;
        }
        let block = self.palette.get(*self.blocks.get(self.index)?)?;
        let x = self.index % self.size.x;
        let y = self.index / self.size.z / self.size.y % self.size.y;
        let z = self.index / self.size.z % self.size.z;
        self.index += 1;
        Some((UVec3::new(x, y, z), block))
    }
}
