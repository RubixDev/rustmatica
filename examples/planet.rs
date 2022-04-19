//! Example taken from [litemapy README](https://github.com/SmylerMC/litemapy#example)

use std::borrow::Cow;

use rustmatica::{Region, util::{Vec3, UVec3}, BlockState, Litematic};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut region = Region::new(Cow::Borrowed("Planet"), UVec3::new(0, 0, 0), Vec3::new(21, 21, 21));

    for (pos, _) in region.clone().blocks() {
        if ((
            (pos.x as i32 - 10).pow(2) +
            (pos.y as i32 - 10).pow(2) +
            (pos.z as i32 - 10).pow(2)
        ) as f64).sqrt().round() <= 10.0 {
            region.set_block(pos, BlockState::LightBlueConcrete)
        }
    }

    let planet = region.as_litematic(Cow::Borrowed("Made with rustmatica"), Cow::Borrowed("RubixDev"));
    planet.write_file("planet.litematic")?;

    let planet = Litematic::read_file("planet.litematic")?;
    let region = &planet.regions[0];

    for x in region.x_range() {
        for z in region.z_range() {
            if region.get_block(UVec3::new(x, 10, z)) == &BlockState::Air {
                print!(" ");
            } else {
                print!("#")
            }
        }
        println!();
    }

    Ok(())
}
