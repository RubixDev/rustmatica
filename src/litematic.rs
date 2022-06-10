use std::{
    borrow::Cow,
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

#[cfg(feature = "chrono")]
use chrono::{DateTime, TimeZone, Utc};
use fastnbt::{from_bytes, to_bytes};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};

use crate::{
    error::Result,
    schema,
    util::{current_time, UVec3},
};

use super::Region;

#[derive(Debug)]
pub struct Litematic<'l> {
    pub regions: Vec<Region<'l>>,
    pub name: Cow<'l, str>,
    pub description: Cow<'l, str>,
    pub author: Cow<'l, str>,
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

impl<'l> Litematic<'l> {
    pub fn new(name: Cow<'l, str>, description: Cow<'l, str>, author: Cow<'l, str>) -> Self {
        let now = current_time();
        Self {
            regions: vec![],
            name,
            description,
            author,
            version: 6,
            minecraft_data_version: 2975,
            time_created: now,
            time_modified: now,
        }
    }

    pub fn from_raw(raw: Cow<'l, schema::Litematic<'l>>) -> Self {
        return Self {
            regions: raw
                .regions
                .iter()
                .map(|(name, region)| {
                    Region::from_raw(Cow::Owned(region.to_owned()), name.to_owned())
                })
                .collect(),
            name: raw.metadata.name.to_owned(),
            description: raw.metadata.description.to_owned(),
            author: raw.metadata.author.to_owned(),
            version: raw.version,
            minecraft_data_version: raw.minecraft_data_version,

            #[cfg(feature = "chrono")]
            time_created: Utc.timestamp_millis(raw.metadata.time_created),
            #[cfg(not(feature = "chrono"))]
            time_created: raw.metadata.time_created,
            #[cfg(feature = "chrono")]
            time_modified: Utc.timestamp_millis(raw.metadata.time_modified),
            #[cfg(not(feature = "chrono"))]
            time_modified: raw.metadata.time_modified,
        };
    }

    pub fn to_raw(&self) -> schema::Litematic {
        return schema::Litematic {
            regions: {
                let mut map = HashMap::new();
                for region in self.regions.iter() {
                    map.insert(region.name.to_owned(), region.to_raw());
                }
                map
            },
            version: self.version,
            minecraft_data_version: self.minecraft_data_version,
            metadata: schema::Metadata {
                name: self.name.to_owned(),
                description: self.description.to_owned(),
                author: self.author.to_owned(),
                region_count: self.regions.len() as u32,
                total_blocks: self.total_blocks(),
                total_volume: self.total_volume(),
                enclosing_size: self.enclosing_size(),

                #[cfg(feature = "chrono")]
                time_created: self.time_created.timestamp_millis(),
                #[cfg(not(feature = "chrono"))]
                time_created: self.time_created,
                #[cfg(feature = "chrono")]
                time_modified: current_time().timestamp_millis(),
                #[cfg(not(feature = "chrono"))]
                time_modified: current_time(),
            },
        };
    }

    pub fn from_uncompressed_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self::from_raw(Cow::Owned(from_bytes(bytes)?)))
    }

    pub fn to_uncompressed_bytes(&self) -> Result<Vec<u8>> {
        Ok(to_bytes(&self.to_raw())?)
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
            x: bounds[1] - bounds[0] + 1,
            y: bounds[3] - bounds[2] + 1,
            z: bounds[5] - bounds[4] + 1,
        }
    }
}
