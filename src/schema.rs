use fastnbt::{IntArray, LongArray};
use mcdata::{
    util::{BlockPos, UVec3},
    GenericBlockEntity, GenericBlockState, GenericEntity,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type CowStr = std::borrow::Cow<'static, str>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Litematic<
    BlockState = GenericBlockState,
    Entity = GenericEntity,
    BlockEntity = GenericBlockEntity,
> where
    BlockState: mcdata::BlockState,
    Entity: mcdata::Entity,
    BlockEntity: mcdata::BlockEntity,
{
    pub regions: HashMap<String, Region<BlockState, Entity, BlockEntity>>,
    pub minecraft_data_version: u32,
    pub version: u32,
    pub sub_version: Option<u32>,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Region<
    BlockState = GenericBlockState,
    Entity = GenericEntity,
    BlockEntity = GenericBlockEntity,
> where
    BlockState: mcdata::BlockState,
    Entity: mcdata::Entity,
    BlockEntity: mcdata::BlockEntity,
{
    pub position: BlockPos,
    pub size: BlockPos,
    pub block_state_palette: Vec<BlockState>,
    pub block_states: LongArray,
    pub tile_entities: Vec<BlockEntity>,
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<Entity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pending_block_ticks: Vec<PendingBlockTick>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pending_fluid_ticks: Vec<PendingFluidTick>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    pub name: String,
    pub author: String,
    pub description: String,
    pub region_count: u32,
    pub total_volume: u32,
    pub total_blocks: u64,
    pub time_created: i64,
    pub time_modified: i64,
    pub enclosing_size: UVec3,
    pub preview_image_data: Option<IntArray>,
}

/// A pending block tick.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[allow(missing_docs)]
pub struct PendingBlockTick {
    pub block: CowStr,
    pub priority: i32,
    pub sub_tick: i64,
    pub time: i32,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "z")]
    pub z: i32,
}

/// A pending fluid tick.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
#[allow(missing_docs)]
pub struct PendingFluidTick {
    pub fluid: CowStr,
    pub priority: i32,
    pub sub_tick: i64,
    pub time: i32,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "z")]
    pub z: i32,
}
