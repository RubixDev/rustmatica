use std::{borrow::Cow, collections::HashMap};
use fastnbt::{Value, LongArray};
use serde::{Serialize, Deserialize};

use crate::{util::Vec3, BlockState, TileEntity};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Litematic<'a> {
    pub regions: HashMap<Cow<'a, str>, Region<'a>>,
    pub minecraft_data_version: u32,
    pub version: u32,
    pub metadata: Metadata<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Region<'a> {
    pub position: Vec3,
    pub size: Vec3,
    pub block_state_palette: Vec<BlockState<'a>>,
    pub tile_entities: Vec<TileEntity<'a>>,
    pub entities: Vec<Value>,
    pub pending_fluid_ticks: Vec<Value>,
    pub pending_block_ticks: Vec<Value>,
    pub block_states: LongArray,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata<'a> {
    pub total_blocks: u64,
    pub name: Cow<'a, str>,
    pub time_modified: i64,
    pub time_created: i64,
    pub region_count: u32,
    pub enclosing_size: Vec3,
    pub total_volume: u32,
    pub description: Cow<'a, str>,
    pub author: Cow<'a, str>,
    // TODO: PreviewImageData: IntArray,
}
