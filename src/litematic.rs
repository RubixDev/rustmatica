use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use fastnbt::IntArray;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use mcdata::{
    util::{BlockPos, Cuboid},
    GenericBlockEntity, GenericBlockState, GenericEntity,
};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, TimeZone, Utc};

#[cfg(feature = "image")]
use image::{
    imageops::{self, FilterType},
    DynamicImage, RgbaImage,
};

use crate::{error::Result, schema, util};

use super::Region;

// TODO: require version 6 when reading?
// TODO: support multiple versions?
const SCHEMATIC_VERSION: i32 = 6;
const SCHEMATIC_VERSION_SUB: i32 = 1;

type CowStr = std::borrow::Cow<'static, str>;

/// Metadata for a litematica schematic.
///
/// Metadata can be read without reading the schematic contents using for example
/// [`LitematicMetadata::read_file`]. When reading a full litematica schematic you can also re-use
/// metadata that has already been read, using for example [`Litematic::read_file_with_metadata`].
/// This might be useful if you want to choose the generic types based on e.g. the
/// [Minecraft data version](Self::minecraft_data_version).
#[derive(Debug)]
pub struct LitematicMetadata {
    /// The name of this schematic.
    pub name: CowStr,

    /// The description of this schematic.
    pub description: CowStr,

    /// The author of this schematic.
    pub author: CowStr,

    /// The litematica format version this schematic was created with.
    pub version: i32,

    /// An optional litematica format subversion.
    pub sub_version: Option<i32>,

    /// The Minecraft [data version](https://minecraft.wiki/w/Data_version) used for blocks and
    /// entities in this schematic.
    ///
    /// See [`mcdata::data_version`] for a list of some important versions.
    pub minecraft_data_version: i32,

    /// The datetime of when this schematic was created.
    #[cfg(feature = "chrono")]
    pub time_created: DateTime<Utc>,

    /// The unix timestamp of when this schematic was created.
    #[cfg(not(feature = "chrono"))]
    pub time_created: i64,

    /// The datetime of when this schematic was last modified.
    #[cfg(feature = "chrono")]
    pub time_modified: DateTime<Utc>,

    /// The unix timestamp of when this schematic was last modified.
    #[cfg(not(feature = "chrono"))]
    pub time_modified: i64,

    /// An optional preview image.
    #[cfg(feature = "image")]
    preview_image: Option<DynamicImage>,

    /// The raw ARGB preview image data.
    #[cfg(not(feature = "image"))]
    preview_image: Option<Vec<i32>>,
}

/// A litematica schematic.
///
/// The type has three generic type parameters for the types of block states, entities, and block
/// entities. These types must implement the corresponding [`mcdata`] trait and default to
/// [`mcdata`]s "generic" types.
///
/// A litematica schematic consists of some [metadata](LitematicMetadata) and a list of
/// [`Region`]s. The metadata contains information like the name, description, and author, while
/// the [`Region`]s contain the blocks and entities.
///
/// ## Coordinate Systems
///
/// Litematica schematics operate on two different coordinate systems: "global" and "region-local".
/// In this documentation, "global" may sometimes also be referred to as "schematic-wide" and
/// "region-local" may be shortened to just "local", completely omitted, or referred to as "within
/// this region". The difference between the two is that the global coordinates can describe every
/// position in the entire schematic, i.e. every position in [`Self::enclosing_box`], whereas local
/// coordinates are specific to one [`Region`] and relative to the regions position. Local
/// coordinates are also always positive and start at 0.
///
/// Usually, when there is one function that takes local coordinates, there is another function
/// that does the same with global coordinates. For example [`Region::get_block`] and
/// [`Region::get_block_global`]. You can also convert between the two systems with
/// [`Region::pos_to_global`] and [`Region::pos_from_global`].
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
    /// The list of [`Region`]s in this schematic.
    pub regions: Vec<Region<BlockState, Entity, BlockEntity>>,

    /// The metadata of this schematic.
    pub metadata: LitematicMetadata,
}

impl<BlockState, Entity, BlockEntity> Litematic<BlockState, Entity, BlockEntity>
where
    BlockState: mcdata::BlockState + Serialize + DeserializeOwned,
    Entity: mcdata::Entity + Serialize + DeserializeOwned,
    BlockEntity: mcdata::BlockEntity + Serialize + DeserializeOwned,
{
    /// Create a new, empty schematic with the given name, description, and author.
    pub fn new(
        name: impl Into<CowStr>,
        description: impl Into<CowStr>,
        author: impl Into<CowStr>,
    ) -> Self {
        let now = util::current_time();
        Self {
            regions: vec![],
            metadata: LitematicMetadata {
                name: name.into(),
                description: description.into(),
                author: author.into(),
                version: SCHEMATIC_VERSION,
                sub_version: Some(SCHEMATIC_VERSION_SUB),
                minecraft_data_version: 2975,
                time_created: now,
                time_modified: now,
                preview_image: None,
            },
        }
    }

    /// Construct a [`Litematic`] from a [raw NBT litematic regions](schema::LitematicRegions) and
    /// existing metadata.
    pub(crate) fn from_raw(
        raw_regions: schema::LitematicRegions<BlockState, Entity, BlockEntity>,
        metadata: LitematicMetadata,
    ) -> Self {
        Self {
            regions: raw_regions
                .regions
                .into_iter()
                .map(|(name, region)| Region::from_raw(region, name))
                .collect(),
            metadata,
        }
    }

    /// Create a new [raw NBT litematic](schema::Litematic) from this [`Litematic`].
    pub(crate) fn to_raw(&self) -> schema::Litematic<BlockState, Entity, BlockEntity> {
        schema::Litematic {
            regions: {
                let mut map = HashMap::new();
                for region in self.regions.iter() {
                    map.insert(region.name.clone().into_owned(), region.to_raw());
                }
                schema::LitematicRegions { regions: map }
            },
            metadata: schema::LitematicMetadata {
                version: self.metadata.version,
                sub_version: self.metadata.sub_version,
                minecraft_data_version: self.metadata.minecraft_data_version,
                metadata: schema::Metadata {
                    name: self.metadata.name.clone().into_owned(),
                    description: self.metadata.description.clone().into_owned(),
                    author: self.metadata.author.clone().into_owned(),
                    region_count: self.regions.len() as i32,
                    total_blocks: self.total_blocks(),
                    total_volume: self.total_volume(),
                    enclosing_size: self.enclosing_size(),

                    #[cfg(feature = "chrono")]
                    time_created: self.metadata.time_created.timestamp_millis(),
                    #[cfg(not(feature = "chrono"))]
                    time_created: self.metadata.time_created,
                    #[cfg(feature = "chrono")]
                    time_modified: util::current_time().timestamp_millis(),
                    #[cfg(not(feature = "chrono"))]
                    time_modified: util::current_time(),

                    #[cfg(feature = "image")]
                    preview_image_data: self.metadata.preview_image.as_ref().map(|img| {
                        IntArray::new(
                            img.to_rgba8()
                                .into_vec()
                                .chunks_exact(4)
                                .map(|px| {
                                    let mut px = [px[0], px[1], px[2], px[3]];
                                    // convert RGBA to ARGB
                                    px.rotate_right(1);
                                    i32::from_be_bytes(px)
                                })
                                .collect(),
                        )
                    }),
                    #[cfg(not(feature = "image"))]
                    preview_image_data: self.metadata.preview_image.clone().map(IntArray::new),
                },
            },
        }
    }

    /// Load a schematic from uncompressed bytes.
    pub fn from_uncompressed_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self::from_raw(
            fastnbt::from_bytes(bytes)?,
            LitematicMetadata::from_uncompressed_bytes(bytes)?,
        ))
    }

    /// Load a schematic from uncompressed bytes and use the exising metadata.
    pub fn from_uncompressed_bytes_with_metadata(
        bytes: &[u8],
        metadata: LitematicMetadata,
    ) -> Result<Self> {
        Ok(Self::from_raw(fastnbt::from_bytes(bytes)?, metadata))
    }

    /// Write this schematic to uncompressed bytes.
    pub fn to_uncompressed_bytes(&self) -> Result<Vec<u8>> {
        Ok(fastnbt::to_bytes(&self.to_raw())?)
    }

    /// Load a schematic from gzip compressed bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut gz = GzDecoder::new(bytes);
        let mut extracted = vec![];
        gz.read_to_end(&mut extracted)?;
        Self::from_uncompressed_bytes(&extracted)
    }

    /// Load a schematic from gzip compressed bytes and use the existing metadata.
    pub fn from_bytes_with_metadata(bytes: &[u8], metadata: LitematicMetadata) -> Result<Self> {
        let mut gz = GzDecoder::new(bytes);
        let mut extracted = vec![];
        gz.read_to_end(&mut extracted)?;
        Self::from_uncompressed_bytes_with_metadata(&extracted, metadata)
    }

    /// Write this schematic to gzip compressed bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = vec![];
        let mut gz = GzEncoder::new(&mut buf, Compression::default());
        gz.write_all(&self.to_uncompressed_bytes()?)?;
        gz.finish()?;
        Ok(buf)
    }

    /// Load a schematic from a file.
    pub fn read_file(filename: impl AsRef<Path>) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        Litematic::from_bytes(&bytes)
    }

    /// Load a schematic from a file and use the existing metadata.
    pub fn read_file_with_metadata(
        filename: impl AsRef<Path>,
        metadata: LitematicMetadata,
    ) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        Litematic::from_bytes_with_metadata(&bytes, metadata)
    }

    /// Write this schematic to a file.
    pub fn write_file(&self, filename: impl AsRef<Path>) -> Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.to_bytes()?)?;
        Ok(())
    }

    /// The total number of blocks all regions combined contain.
    pub fn total_blocks(&self) -> i64 {
        self.regions.iter().map(|r| r.total_blocks() as i64).sum()
    }

    /// The total volume of all regions combined.
    pub fn total_volume(&self) -> i32 {
        self.regions.iter().map(|r| r.size.volume() as i32).sum()
    }

    /// Calculate the size of the box enclosing all regions.
    #[inline]
    pub fn enclosing_size(&self) -> BlockPos {
        self.enclosing_box().size
    }

    /// Calculate the box enclosing all regions.
    pub fn enclosing_box(&self) -> Cuboid {
        let mut bounds = [0; 6];
        for region in self.regions.iter() {
            bounds[0] = bounds[0].min(region.min_global_x());
            bounds[1] = bounds[1].max(region.max_global_x());
            bounds[2] = bounds[2].min(region.min_global_y());
            bounds[3] = bounds[3].max(region.max_global_y());
            bounds[4] = bounds[4].min(region.min_global_z());
            bounds[5] = bounds[5].max(region.max_global_z());
        }
        Cuboid {
            origin: BlockPos {
                x: bounds[0],
                y: bounds[2],
                z: bounds[4],
            },
            size: BlockPos {
                x: (bounds[1] - bounds[0] + 1).abs(),
                y: (bounds[3] - bounds[2] + 1).abs(),
                z: (bounds[5] - bounds[4] + 1).abs(),
            },
        }
    }
}

impl LitematicMetadata {
    /// Construct [`LitematicMetadata`] from [raw NBT litematic metadata](schema::LitematicMetadata).
    pub(crate) fn from_raw(raw: schema::LitematicMetadata) -> Self {
        Self {
            name: CowStr::Owned(raw.metadata.name),
            description: CowStr::Owned(raw.metadata.description),
            author: CowStr::Owned(raw.metadata.author),
            version: raw.version,
            sub_version: raw.sub_version,
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

            #[cfg(feature = "image")]
            preview_image: raw.metadata.preview_image_data.map(|data| {
                RgbaImage::from_raw(
                    140,
                    140,
                    data.iter()
                        .flat_map(|int| {
                            let mut px = int.to_be_bytes();
                            // data is ARGB, convert it to RGBA
                            px.rotate_left(1);
                            px
                        })
                        .collect(),
                )
                .expect("preview image data should have 140x140 pixels")
                .into()
            }),
            #[cfg(not(feature = "image"))]
            preview_image: raw
                .metadata
                .preview_image_data
                .clone()
                .map(|data| data.into_inner()),
        }
    }

    /// Load schematic metadata from uncompressed bytes.
    pub fn from_uncompressed_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self::from_raw(fastnbt::from_bytes(bytes)?))
    }

    /// Load schematic metadata from gzip compressed bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut gz = GzDecoder::new(bytes);
        let mut extracted = vec![];
        gz.read_to_end(&mut extracted)?;
        Self::from_uncompressed_bytes(&extracted)
    }

    /// Load schematic metadata from a file.
    pub fn read_file(filename: impl AsRef<Path>) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        LitematicMetadata::from_bytes(&bytes)
    }

    /// Get the optional preview image.
    #[cfg(feature = "image")]
    pub fn preview_image(&self) -> Option<&DynamicImage> {
        self.preview_image.as_ref()
    }

    /// Set the preview image.
    ///
    /// Returns the previous image if there was one.
    #[cfg(feature = "image")]
    pub fn set_preview_image(&mut self, image: Option<DynamicImage>) -> Option<DynamicImage> {
        match image {
            Some(img) => {
                let scaled = img.resize(140, 140, FilterType::CatmullRom);
                let mut preview = RgbaImage::new(140, 140);
                imageops::overlay(
                    &mut preview,
                    &scaled,
                    (140 - scaled.width() as i64) / 2,
                    (140 - scaled.height() as i64) / 2,
                );
                self.preview_image.replace(preview.into())
            }
            None => self.preview_image.take(),
        }
    }

    /// Get the optional raw ARGB preview image data.
    #[cfg(not(feature = "image"))]
    pub fn preview_image_data(&self) -> Option<&[i32]> {
        self.preview_image.as_deref()
    }

    /// Set the preview image.
    ///
    /// Returns the previous image data if there was some.
    ///
    /// Panics if the data length is not exactly `140 * 140`.
    #[cfg(not(feature = "image"))]
    pub fn set_preview_image_data(&mut self, data: Option<Vec<i32>>) -> Option<Vec<i32>> {
        match data {
            Some(data) => {
                assert_eq!(
                    data.len(),
                    140 * 140,
                    "preview image must be 140x140 pixels in size"
                );
                self.preview_image.replace(data)
            }
            None => self.preview_image.take(),
        }
    }
}
