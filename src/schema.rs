use fastnbt::{LongArray, Value};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

use crate::{
    util::{UVec3, Vec3},
    BlockState, Entity, TileEntity,
};

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
    pub position: UVec3,
    pub size: Vec3,
    pub block_state_palette: Vec<BlockState<'a>>,
    pub tile_entities: Vec<TileEntity<'a>>,
    pub entities: Vec<Entity<'a>>,
    pub pending_fluid_ticks: Vec<Value>, /* TODO */
    pub pending_block_ticks: Vec<Value>, /* TODO */
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
    pub enclosing_size: UVec3,
    pub total_volume: u32,
    pub description: Cow<'a, str>,
    pub author: Cow<'a, str>,
    // TODO: PreviewImageData: IntArray,
}
