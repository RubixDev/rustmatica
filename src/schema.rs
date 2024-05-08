use fastnbt::{LongArray, Value};
use mcdata::{
    util::{BlockPos, UVec3},
    GenericBlockEntity, GenericBlockState, GenericEntity,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub tile_entities: Vec<BlockEntity>,
    pub entities: Vec<Entity>,
    pub pending_fluid_ticks: Vec<Value>, /* TODO */
    pub pending_block_ticks: Vec<Value>, /* TODO */
    pub block_states: LongArray,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    pub total_blocks: u64,
    pub name: String,
    pub time_modified: i64,
    pub time_created: i64,
    pub region_count: u32,
    pub enclosing_size: UVec3,
    pub total_volume: u32,
    pub description: String,
    pub author: String,
    // TODO: PreviewImageData: IntArray,
}
