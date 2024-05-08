use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

#[cfg(feature = "chrono")]
use chrono::{DateTime, TimeZone, Utc};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use mcdata::{util::UVec3, GenericBlockEntity, GenericBlockState, GenericEntity};
use serde::{de::DeserializeOwned, Serialize};

use crate::{error::Result, schema, util};

use super::Region;

// TODO: require version 6 when reading?
// TODO: support multiple versions?
const SCHEMATIC_VERSION: u32 = 6;

type CowStr = std::borrow::Cow<'static, str>;

#[derive(Debug)]
pub struct Litematic<
    BlockState = GenericBlockState,
    Entity = GenericEntity,
    BlockEntity = GenericBlockEntity,
> where
    BlockState: mcdata::BlockState + Serialize + DeserializeOwned,
    Entity: mcdata::Entity + Serialize + DeserializeOwned,
    BlockEntity: mcdata::BlockEntity + Serialize + DeserializeOwned,
{
    pub regions: Vec<Region<BlockState, Entity, BlockEntity>>,
    pub name: CowStr,
    pub description: CowStr,
    pub author: CowStr,
    pub version: u32,
    pub minecraft_data_version: u32,

    #[cfg(feature = "chrono")]
    pub time_created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub time_created: i64,
    #[cfg(feature = "chrono")]
    pub time_modified: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub time_modified: i64,
}

impl<BlockState, Entity, BlockEntity> Litematic<BlockState, Entity, BlockEntity>
where
    BlockState: mcdata::BlockState + Serialize + DeserializeOwned,
    Entity: mcdata::Entity + Serialize + DeserializeOwned,
    BlockEntity: mcdata::BlockEntity + Serialize + DeserializeOwned,
{
    pub fn new(
        name: impl Into<CowStr>,
        description: impl Into<CowStr>,
        author: impl Into<CowStr>,
    ) -> Self {
        let now = util::current_time();
        Self {
            regions: vec![],
            name: name.into(),
            description: description.into(),
            author: author.into(),
            version: SCHEMATIC_VERSION,
            minecraft_data_version: 2975,
            time_created: now,
            time_modified: now,
        }
    }

    pub fn from_raw(raw: schema::Litematic<BlockState, Entity, BlockEntity>) -> Self {
        return Self {
            regions: raw
                .regions
                .iter()
                .map(|(name, region)| Region::from_raw(region.to_owned(), name.clone()))
                .collect(),
            name: CowStr::Owned(raw.metadata.name.clone()),
            description: CowStr::Owned(raw.metadata.description.clone()),
            author: CowStr::Owned(raw.metadata.author.clone()),
            version: raw.version,
            minecraft_data_version: raw.minecraft_data_version,

            #[cfg(feature = "chrono")]
            time_created: Utc.timestamp_millis_opt(raw.metadata.time_created).unwrap(),
            #[cfg(not(feature = "chrono"))]
            time_created: raw.metadata.time_created,
            #[cfg(feature = "chrono")]
            time_modified: Utc
                .timestamp_millis_opt(raw.metadata.time_modified)
                .unwrap(),
            #[cfg(not(feature = "chrono"))]
            time_modified: raw.metadata.time_modified,
        };
    }

    pub fn to_raw(&self) -> schema::Litematic<BlockState, Entity, BlockEntity> {
        schema::Litematic {
            regions: {
                let mut map = HashMap::new();
                for region in self.regions.iter() {
                    map.insert(region.name.clone().into_owned(), region.to_raw());
                }
                map
            },
            version: self.version,
            minecraft_data_version: self.minecraft_data_version,
            metadata: schema::Metadata {
                name: self.name.clone().into_owned(),
                description: self.description.clone().into_owned(),
                author: self.author.clone().into_owned(),
                region_count: self.regions.len() as u32,
                total_blocks: self.total_blocks(),
                total_volume: self.total_volume(),
                enclosing_size: self.enclosing_size(),

                #[cfg(feature = "chrono")]
                time_created: self.time_created.timestamp_millis(),
                #[cfg(not(feature = "chrono"))]
                time_created: self.time_created,
                #[cfg(feature = "chrono")]
                time_modified: util::current_time().timestamp_millis(),
                #[cfg(not(feature = "chrono"))]
                time_modified: util::current_time(),
            },
        }
    }

    pub fn from_uncompressed_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self::from_raw(fastnbt::from_bytes(bytes)?))
    }

    pub fn to_uncompressed_bytes(&self) -> Result<Vec<u8>> {
        Ok(fastnbt::to_bytes(&self.to_raw())?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut gz = GzDecoder::new(bytes);
        let mut extracted = vec![];
        gz.read_to_end(&mut extracted)?;
        Self::from_uncompressed_bytes(&extracted)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = vec![];
        let mut gz = GzEncoder::new(&mut buf, Compression::default());
        gz.write_all(&self.to_uncompressed_bytes()?)?;
        gz.finish()?;
        Ok(buf)
    }

    pub fn read_file(filename: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        Litematic::from_bytes(&bytes)
    }

    pub fn write_file(&self, filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.to_bytes()?)?;
        Ok(())
    }

    pub fn total_blocks(&self) -> u64 {
        self.regions.iter().map(|r| r.total_blocks() as u64).sum()
    }

    pub fn total_volume(&self) -> u32 {
        self.regions.iter().map(|r| r.size.volume() as u32).sum()
    }

    pub fn enclosing_size(&self) -> UVec3 {
        let mut bounds = [0; 6];
        for region in self.regions.iter() {
            bounds[0] = bounds[0].min(region.min_global_x());
            bounds[1] = bounds[1].max(region.max_global_x());
            bounds[2] = bounds[2].min(region.min_global_y());
            bounds[3] = bounds[3].max(region.max_global_y());
            bounds[4] = bounds[4].min(region.min_global_z());
            bounds[5] = bounds[5].max(region.max_global_z());
        }
        UVec3 {
            x: (bounds[1] - bounds[0] + 1).unsigned_abs(),
            y: (bounds[3] - bounds[2] + 1).unsigned_abs(),
            z: (bounds[5] - bounds[4] + 1).unsigned_abs(),
        }
    }
}
