//! This example displays either a single layer or a top-down view of a schematic in the terminal
//! using Unicode half blocks and ANSI escape codes. Each block is represented by its default map
//! color. It demonstrates how to work with multiple regions and the global coordinate system.

use std::fmt::Write;

use mcdata::{latest::MapColor, util::BlockPos};

// define a type alias so we don't have to repeat the generics everywhere
type Litematic = rustmatica::Litematic<mcdata::latest::BlockState>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1);
    let path = path
        .as_deref()
        .unwrap_or("test_files/tmc_catalogue/4gt_azalea_94.5k.litematic");
    let schem = Litematic::read_file(path)?;

    let pixels = match std::env::args().nth(2).as_deref() {
        Some("all") | None => {
            // if not exactly one y layer was specified, get a top-down view of all layers
            let enclosing = schem.enclosing_box();
            let mut pixels = get_pixels_for_y(&schem, enclosing.origin.y);
            for y in enclosing.y_range() {
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
            pixels
        }
        Some(s) => get_pixels_for_y(&schem, s.parse().expect("layer must be a valid i32")),
    };

    println!("{}", pixels_to_ansi(&pixels));

    Ok(())
}

fn get_pixels_for_y(schem: &Litematic, y: i32) -> Vec<Vec<MapColor>> {
    let mut pixels = vec![];
    let enclosing = schem.enclosing_box();
    for z in enclosing.z_range() {
        let mut row = vec![];
        for x in enclosing.x_range() {
            // find all blocks in all regions at this position
            let blocks = schem
                .regions
                .iter()
                .filter_map(|region| region.get_block_global_opt(BlockPos::new(x, y, z)))
                .collect::<Vec<_>>();
            // log it, if multiple regions overlap here
            if blocks.len() > 1 {
                println!("multiple blocks at ({x}, {y}, {z}) : {blocks:?}");
            }
            // save the map color
            row.push(blocks.first().map_or(MapColor::None, |b| b.map_color()));
        }
        pixels.push(row);
    }
    pixels
}

fn pixels_to_ansi(pixels: &[Vec<MapColor>]) -> String {
    const TOP_HALF: &str = "\u{2580}";
    const BOTTOM_HALF: &str = "\u{2584}";

    let mut out = String::new();
    for line in (0..pixels.len()).step_by(2) {
        for char in 0..pixels[line].len() {
            let top_pix = pixels[line][char];
            let bot_pix = if line + 1 >= pixels.len() {
                MapColor::None
            } else {
                pixels[line + 1][char]
            };
            let top_invis = top_pix == MapColor::None;
            let bot_invis = bot_pix == MapColor::None;

            if top_invis && bot_invis {
                out += " ";
            } else if top_invis && !bot_invis {
                let [r, g, b, _] = bot_pix.rgba();
                let _ = write!(out, "\x1b[38;2;{r};{g};{b}m{BOTTOM_HALF}\x1b[0m");
            } else if !top_invis && bot_invis {
                let [r, g, b, _] = top_pix.rgba();
                let _ = write!(out, "\x1b[38;2;{r};{g};{b}m{TOP_HALF}\x1b[0m");
            } else {
                let [br, bg, bb, _] = bot_pix.rgba();
                let [tr, tg, tb, _] = top_pix.rgba();
                let _ = write!(
                    out,
                    "\x1b[38;2;{br};{bg};{bb};48;2;{tr};{tg};{tb}m{BOTTOM_HALF}\x1b[0m"
                );
            }
        }
        out += "\n";
    }
    out
}
