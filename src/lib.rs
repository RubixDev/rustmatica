#[macro_use]
pub mod block_state;
pub mod util;
pub mod error;
mod schema;
mod litematic;
mod region;

pub use litematic::*;
pub use region::*;
pub use block_state::BlockState;

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, collections::HashMap};

    use crate::{Litematic, BlockState, util::UVec3, block_state::types::HorizontalDirection};

    #[test]
    fn read_write() -> Result<(), Box<dyn std::error::Error>> {
        let mut donut = Litematic::read_file("./test_files/donut.litematic")?;
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::Air);
        donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::GrassBlock { snowy: false });
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::GrassBlock { snowy: false });
        donut.write_file("./test_files/donut_modified.litematic")?;

        let mut new_donut = Litematic::read_file("./test_files/donut_modified.litematic")?;
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::GrassBlock { snowy: false });
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::Air);
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::Air);
        new_donut.write_file("./test_files/donut.litematic")?;

        Ok(())
    }

    #[test]
    fn block_state_eq() {
        assert_eq!(
            BlockState::Air,
            BlockState::Other { name: Cow::Borrowed("minecraft:air"), properties: None },
        );
        assert_eq!(
            BlockState::GrassBlock { snowy: true },
            BlockState::Other {
                name: Cow::Borrowed("minecraft:grass_block"),
                properties: Some(HashMap::from([
                    (Cow::Borrowed("snowy"), Cow::Borrowed("true")),
                ])),
            },
        );
        assert_eq!(
            BlockState::Repeater {
                delay: 3,
                facing: HorizontalDirection::West,
                locked: true,
                powered: true,
            },
            BlockState::Other {
                name: Cow::Borrowed("minecraft:repeater"),
                properties: Some(HashMap::from([
                    (Cow::Borrowed("delay"), Cow::Borrowed("3")),
                    (Cow::Borrowed("facing"), Cow::Borrowed("west")),
                    (Cow::Borrowed("locked"), Cow::Borrowed("true")),
                    (Cow::Borrowed("powered"), Cow::Borrowed("true")),
                ])),
            },
        );
    }

    #[test]
    fn macros() {
        assert_eq!(BlockState::Air, block!());
        assert_eq!(BlockState::Stone, block!(Stone));
        assert_eq!(BlockState::GrassBlock { snowy: false }, block!(GrassBlock { snowy: false }));
        assert_eq!(BlockState::Stone, block!("minecraft:stone"));
        assert_eq!(BlockState::GrassBlock { snowy: true }, block!("minecraft:grass_block"; "snowy" => "true"));
        assert_eq!(
            BlockState::Repeater {
                delay: 2,
                facing: HorizontalDirection::South,
                locked: false,
                powered: false,
            },
            block!(
                "minecraft:repeater";
                "delay" => "2",
                "facing" => "south",
                "locked" => "false",
                "powered" => "false",
            ),
        );
    }
}
