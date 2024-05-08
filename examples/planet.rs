//! Example taken from [litemapy README](https://github.com/SmylerMC/litemapy#example)

use mcdata::{
    latest::BlockState,
    util::{BlockPos, UVec3},
};
use rustmatica::{Litematic, Region};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut region: Region<BlockState> =
        Region::new("Planet", BlockPos::new(0, 0, 0), BlockPos::new(21, 21, 21));

    for (pos, _) in region.clone().blocks() {
        if (((pos.x as i32 - 10).pow(2) + (pos.y as i32 - 10).pow(2) + (pos.z as i32 - 10).pow(2))
            as f64)
            .sqrt()
            .round()
            <= 10.0
        {
            region.set_block(pos, BlockState::LightBlueConcrete)
        }
    }

    let planet = region.as_litematic("Made with rustmatica", "RubixDev");
    planet.write_file("planet.litematic")?;

    let planet: Litematic<BlockState> = Litematic::read_file("planet.litematic")?;
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
