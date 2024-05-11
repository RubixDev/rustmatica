//! This example creates a preview for a schematic similar to `layer_view.rs` and sets it as
//! the schematics preview image.

use image::{imageops::FilterType, DynamicImage, Rgba, RgbaImage};
use mcdata::{latest::MapColor, util::BlockPos};

// define a type alias so we don't have to repeat the generics everywhere
type Litematic = rustmatica::Litematic<mcdata::latest::BlockState>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1);
    let path = path
        .as_deref()
        .unwrap_or("test_files/tmc_catalogue/4gt_azalea_94.5k.litematic");

    let mut schem = Litematic::read_file(path)?;
    let enclosing = schem.enclosing_box();

    let min_y = std::env::args()
        .nth(2)
        .map(|s| s.parse::<i32>().expect("min_y must be a valid i32"))
        .unwrap_or(enclosing.y_range().start);
    let max_y = std::env::args()
        .nth(3)
        .map(|s| s.parse::<i32>().expect("max_y must be a valid i32"))
        .unwrap_or(enclosing.y_range().end);

    let mut pixels = get_pixels_for_y(&schem, min_y);
    for y in min_y + 1..max_y {
        let layer_pixels = get_pixels_for_y(&schem, y);
        for (prev, new) in pixels
            .iter_mut()
            .flatten()
            .zip(layer_pixels.into_iter().flatten())
        {
            if new != MapColor::None {
                *prev = new;
            }
        }
    }

    // construct the image
    let img = RgbaImage::from_fn(
        enclosing.size.x.unsigned_abs(),
        enclosing.size.z.unsigned_abs(),
        |x, z| Rgba(pixels[z as usize][x as usize].rgba()),
    );
    // resize manually so we can specify the filter type
    let img = DynamicImage::from(img).resize(140, 140, FilterType::Nearest);
    schem.metadata.set_preview_image(Some(img));
    schem.write_file(path)?;

    Ok(())
}

fn get_pixels_for_y(schem: &Litematic, y: i32) -> Vec<Vec<MapColor>> {
    let mut pixels = vec![];
    let enclosing = schem.enclosing_box();
    for z in enclosing.z_range() {
        let mut row = vec![];
        for x in enclosing.x_range() {
            row.push(
                schem
                    .regions
                    .iter()
                    .filter_map(|region| region.get_block_global_opt(BlockPos::new(x, y, z)))
                    .next()
                    .map_or(MapColor::None, |b| b.map_color()),
            );
        }
        pixels.push(row);
    }
    pixels
}
