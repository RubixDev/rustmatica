//! This example creates a top-down preview for a schematic and sets it as the schematics preview
//! image. It takes a path to a `client.jar` as its first argument which will be used to retrieve
//! block models and textures.
//!
//! In order to keep this code relatively short, a bunch of shortcuts have been taken for block
//! model resolving. Because of that, the preview will only show full blocks which inherit the
//! `block/cube` model.

use std::{
    collections::{hash_map::Entry, BTreeMap, HashMap},
    fs::File,
    io::Read,
};

use image::{
    imageops::{self, FilterType},
    DynamicImage, ImageFormat, Rgba, RgbaImage,
};
use mcdata::util::{BlockPos, Cuboid};
use zip::ZipArchive;

// define a type alias instead of import so the generics are "filled out"
type Litematic = rustmatica::Litematic;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // resolve and read client.jar
    let jar_path = std::env::args().nth(1);
    let jar_path = jar_path
        .as_deref()
        .expect("a path to client.jar is required");
    let client_jar = ZipArchive::new(File::open(jar_path)?)?;

    // resolve and read schematic
    let schem_path = std::env::args().nth(2);
    let schem_path = schem_path
        .as_deref()
        .unwrap_or("test_files/tmc_catalogue/4gt_azalea_94.5k.litematic");
    let schem = Litematic::read_file(schem_path)?;
    let enclosing_box = schem.enclosing_box();

    // resolve other CLI args
    let min_y = std::env::args()
        .nth(3)
        .and_then(|s| {
            if s == "_" {
                None
            } else {
                Some(s.parse::<i32>().expect("min_y must be a valid i32"))
            }
        })
        .unwrap_or(enclosing_box.y_range().start);
    let max_y = std::env::args()
        .nth(4)
        .and_then(|s| {
            if s == "_" {
                None
            } else {
                Some(s.parse::<i32>().expect("max_y must be a valid i32"))
            }
        })
        .unwrap_or(enclosing_box.y_range().end);
    let shadow_opacity = std::env::args()
        .nth(5)
        .map(|s| s.parse::<u8>().expect("shadow opacity must be a valid u8"))
        .unwrap_or(0);

    // construct a context
    let mut ctx = Context {
        client_jar,
        schem,
        enclosing_box,
        block_states: HashMap::new(),
        block_models: HashMap::new(),
        img_width: enclosing_box.x_range().len() as u32 * 16,
        img_height: enclosing_box.z_range().len() as u32 * 16,
    };

    // the target image
    let mut img = DynamicImage::from(RgbaImage::new(ctx.img_width, ctx.img_height));
    let shadow_layer = RgbaImage::from_pixel(
        ctx.img_width,
        ctx.img_height,
        Rgba([0, 0, 0, shadow_opacity]),
    );
    for y in min_y..max_y {
        // overlay each layer on the target
        let layer = get_pixels_for_y(&mut ctx, y)?;
        imageops::overlay(&mut img, &layer, 0, 0);
        // overlay a translucent black layer so that lower layers appear darker
        imageops::overlay(&mut img, &shadow_layer, 0, 0);
    }

    // manually resize to use `FilterType::Nearest` instead of `CatmullRom`
    let img = img.resize(140, 140, FilterType::Nearest);
    // set preview image and save
    ctx.schem.metadata.set_preview_image(Some(img));
    ctx.schem.write_file(schem_path)?;

    Ok(())
}

struct Context {
    client_jar: ZipArchive<File>,
    schem: Litematic,
    enclosing_box: Cuboid,
    block_states: HashMap<String, JsonBlockState>,
    block_models: HashMap<String, Model>,
    img_width: u32,
    img_height: u32,
}

impl Context {
    fn get_block_state(&mut self, id: String) -> Result<&JsonBlockState> {
        Ok(match self.block_states.entry(id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let path = format!(
                    "assets/minecraft/blockstates/{}.json",
                    entry
                        .key()
                        .strip_prefix("minecraft:")
                        .unwrap_or(entry.key())
                );
                let model = serde_json::from_reader(
                    self.client_jar
                        .by_name(&path)
                        .map_err(|e| format!("couldn't read '{path}': {e}"))?,
                )
                .map_err(|e| format!("failed to parse '{path}': {e}"))?;
                entry.insert(model)
            }
        })
    }

    fn get_model(&mut self, id: String) -> Result<&Model> {
        Ok(match self.block_models.entry(id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let path = format!(
                    "assets/minecraft/models/{}.json",
                    entry
                        .key()
                        .strip_prefix("minecraft:")
                        .unwrap_or(entry.key())
                );
                let model = serde_json::from_reader(
                    self.client_jar
                        .by_name(&path)
                        .map_err(|e| format!("couldn't read '{path}': {e}"))?,
                )
                .map_err(|e| format!("failed to parse '{path}': {e}"))?;
                entry.insert(model)
            }
        })
    }
}

// the following definitions are incomplete and only contain what's needed for this example

#[derive(Debug, Clone, serde::Deserialize)]
enum JsonBlockState {
    #[serde(rename = "variants")]
    Variants(BTreeMap<String, BlockStateVariants>),
    #[serde(rename = "multipart")]
    #[allow(dead_code)]
    Multipart(serde_json::Value),
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
enum BlockStateVariants {
    Single(BlockStateVariant),
    Multiple(Vec<BlockStateVariant>),
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BlockStateVariant {
    model: String,
    #[serde(default)]
    x: u16,
    #[serde(default)]
    y: u16,
    #[serde(default)]
    uvlock: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Model {
    parent: Option<String>,
    #[serde(default)]
    textures: HashMap<String, String>,
}

fn get_pixels_for_y(ctx: &mut Context, y: i32) -> Result<DynamicImage> {
    // the image for this layer
    let mut img = RgbaImage::new(ctx.img_width, ctx.img_height);
    for z in ctx.enclosing_box.z_range() {
        for x in ctx.enclosing_box.x_range() {
            // get the block state at this position
            let Some(block) = ctx
                .schem
                .regions
                .iter()
                .find_map(|region| region.get_block_global_opt(BlockPos::new(x, y, z)))
                .cloned()
            else {
                continue;
            };
            // get the blockstate json
            let state = ctx.get_block_state(block.name.clone().into_owned())?;
            // resolve the variant to use
            let variant = match state {
                // to keep things simple we don't support multipart models
                JsonBlockState::Multipart(_) => continue,
                // find the first variant matching the block state
                JsonBlockState::Variants(variants) => match variants
                    .iter()
                    .find(|(k, _)| {
                        k.split(',').all(|prop| {
                            prop.is_empty() || {
                                let (k, v) = prop.split_once('=').unwrap();
                                block.properties[k] == v
                            }
                        })
                    })
                    .map(|(_, v)| v)
                {
                    Some(BlockStateVariants::Single(single)) => single,
                    // usually one should be selected at random, respecing their "weight", but to
                    // keep things simple we just take the first one
                    Some(BlockStateVariants::Multiple(multiple)) => &multiple[0],
                    None => continue,
                },
            }
            .clone();

            // collect all parents
            let mut models = vec![];
            let mut curr = variant.model.clone();
            loop {
                let c = ctx.get_model(curr)?.clone();
                models.push(c.clone());
                let Some(parent) = c.parent else { break };
                curr = parent;
            }
            // to keep things simple, we only look for models that have "block/cube" as a parent
            if !models
                .iter()
                .any(|model| model.parent.as_deref() == Some("block/cube"))
            {
                continue;
            }

            // merge all "texture variables"
            let mut textures = HashMap::new();
            for model in models.into_iter().rev() {
                textures.extend(model.textures);
            }
            // north: -Z
            // east: +X
            // south: +Z
            // west: -X
            // TODO: validate this in-game
            let up_face = match variant.x {
                0 => "up",
                90 => "south",
                180 => "down",
                270 => "north",
                angle => panic!("invalid x rotation: {angle}"),
            };
            // fully resolve the texture path
            let Some(mut up) = textures.get(up_face) else {
                continue;
            };
            while let Some(t) = up.strip_prefix('#') {
                up = &textures[t];
            }

            // and we finally know which texture to use
            let path = format!(
                "assets/minecraft/textures/{}.png",
                up.strip_prefix("minecraft:").unwrap_or(up)
            );
            // read the texture file from the jar
            let mut buf = vec![];
            let mut file = ctx
                .client_jar
                .by_name(&path)
                .map_err(|e| format!("couldn't read '{path}': {e}"))?;
            file.read_to_end(&mut buf)
                .map_err(|e| format!("couldn't read '{path}': {e}"))?;
            let tex = image::load_from_memory_with_format(&buf, ImageFormat::Png)?;
            // rotate it accordingly
            let tex = match (variant.uvlock, variant.y) {
                (true, _) | (_, 0) => tex,
                (false, 90) => tex.rotate90(),
                (false, 180) => tex.rotate180(),
                (false, 270) => tex.rotate270(),
                (_, angle) => panic!("invalid y rotation: {angle}"),
            };
            // and add it to the image
            imageops::overlay(&mut img, &tex, x as i64 * 16, z as i64 * 16);
        }
    }
    Ok(img.into())
}
