use std::borrow::Cow;

use bitvec::{view::BitView, order::Lsb0, prelude::BitVec};
use fastnbt::LongArray;

use crate::{schema::{BlockState, self}, util::{Vec3, UVec3}};

#[derive(Debug)]
pub struct Region<'l> {
    pub name: Cow<'l, str>,
    pub position: Vec3,
    pub size: Vec3,
    palette: Vec<BlockState<'l>>,
    blocks: Vec<usize>,
}

impl <'l> Region<'l> {
    pub fn new(name: Cow<'l, str>, position: Vec3, size: Vec3) -> Self {
        return Self {
            name,
            position,
            size,
            palette: vec![BlockState {
                name: Cow::Borrowed("minecraft:air"),
                properties: None,
            }],
            blocks: vec![0; size.volume()],
        };
    }

    pub fn from_raw(raw: Cow<'l, schema::Region<'l>>, name: Cow<'l, str>) -> Self {
        let mut new = Self::new(name, raw.position, raw.size);
        new.palette = raw.block_state_palette.clone();

        let num_bits = new.num_bits();
        new.blocks = raw.block_states.iter()
            .map(|block| (*block as u64).view_bits::<Lsb0>().to_bitvec())
            .flatten()
            .collect::<BitVec>()
            .chunks(num_bits)
            .map(|slice| slice.iter().rev().fold(0, |acc, bit| acc << 1 | *bit as usize))
            .collect::<Vec<usize>>();

        return new;
    }

    pub fn to_raw(&self) -> schema::Region {
        let mut new = schema::Region {
            position: self.position,
            size: self.size,
            block_state_palette: self.palette.to_owned(),
            tile_entities: vec![],
            entities: vec![],
            pending_fluid_ticks: vec![],
            pending_block_ticks: vec![],
            block_states: LongArray::new(vec![]),
        };

        let num_bits = self.num_bits();
        new.block_states = LongArray::new(self.blocks.iter()
            .map(|id| (&id.view_bits::<Lsb0>()[..num_bits]).iter().collect::<BitVec>())
            .flatten()
            .collect::<BitVec>()
            .chunks(64)
            .map(|bits| bits.iter().rev().fold(0, |acc, bit| acc << 1 | *bit as i64))
            .collect());

        return new;
    }

    fn num_bits(&self) -> usize {
        let mut num_bits = 2;
        while 1 << num_bits < self.palette.len() {
            num_bits += 1;
        }
        return num_bits;
    }

    fn pos_to_index(&self, pos: UVec3) -> usize {
        let size = self.size.abs();
        return pos.x
             + pos.y * size.y
             + pos.z * size.y * size.x;
    }

    pub fn get_block(&'l self, pos: UVec3) -> &'l BlockState<'l> {
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

    pub fn min_global_x(&self) -> i32 { self.position.x.min(self.position.x + self.size.x + 1) }
    pub fn max_global_x(&self) -> i32 { self.position.x.max(self.position.x + self.size.x - 1) }
    pub fn min_global_y(&self) -> i32 { self.position.y.min(self.position.y + self.size.y + 1) }
    pub fn max_global_y(&self) -> i32 { self.position.y.max(self.position.y + self.size.y - 1) }
    pub fn min_global_z(&self) -> i32 { self.position.z.min(self.position.z + self.size.z + 1) }
    pub fn max_global_z(&self) -> i32 { self.position.z.max(self.position.z + self.size.z - 1) }

    pub fn min_x(&self) -> i32 { 0.min(self.size.x + 1) }
    pub fn max_x(&self) -> i32 { 0.max(self.size.x - 1) }
    pub fn min_y(&self) -> i32 { 0.min(self.size.y + 1) }
    pub fn max_y(&self) -> i32 { 0.max(self.size.y - 1) }
    pub fn min_z(&self) -> i32 { 0.min(self.size.z + 1) }
    pub fn max_z(&self) -> i32 { 0.max(self.size.z - 1) }

    pub fn total_blocks(&self) -> usize { self.blocks.iter().filter(|b| b != &&0).count() }

    // TODO: blocks() -> Iterator over all blocks (maybe with position)
}
