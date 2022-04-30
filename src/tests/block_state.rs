#[cfg(feature = "lists")]
mod with_list {
    use std::{borrow::Cow, collections::HashMap};
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Litematic, BlockState, util::UVec3, block_state::types::HorizontalDirection};

    #[test]
    #[cfg(not(target_family = "wasm"))]
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

    #[wasm_bindgen_test]
    #[cfg(target_family = "wasm")]
    fn read_write() {
        let mut donut = Litematic::from_bytes(include_bytes!("../../test_files/donut.litematic")).unwrap();
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::Air);
        donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::GrassBlock { snowy: false });
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::GrassBlock { snowy: false });
        let buf = donut.to_bytes().unwrap();

        let mut new_donut = Litematic::from_bytes(&buf).unwrap();
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::GrassBlock { snowy: false });
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::Air);
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::Air);
        new_donut.to_bytes().unwrap();
    }

    #[test]
    #[wasm_bindgen_test]
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
    #[wasm_bindgen_test]
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

#[cfg(not(feature = "lists"))]
mod without_list {
    use std::{borrow::Cow, collections::HashMap};
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Litematic, BlockState, util::UVec3};

    #[test]
    fn read_write() -> Result<(), Box<dyn std::error::Error>> {
        let mut donut = Litematic::read_file("./test_files/donut.litematic")?;
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!());
        donut.regions[0].set_block(UVec3::new(1, 1, 1), block!("minecraft:grass_block"; "snowy" => "false"));
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!("minecraft:grass_block"; "snowy" => "false"));
        donut.write_file("./test_files/donut_modified.litematic")?;

        let mut new_donut = Litematic::read_file("./test_files/donut_modified.litematic")?;
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!("minecraft:grass_block"; "snowy" => "false"));
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), block!());
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!());
        new_donut.write_file("./test_files/donut.litematic")?;

        Ok(())
    }

    #[wasm_bindgen_test]
    fn read_write_wasm() {
        let mut donut = Litematic::from_bytes(include_bytes!("../../test_files/donut.litematic")).unwrap();
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!());
        donut.regions[0].set_block(UVec3::new(1, 1, 1), block!("minecraft:grass_block"; "snowy" => "false"));
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!("minecraft:grass_block"; "snowy" => "false"));
        let buf = donut.to_bytes().unwrap();

        let mut new_donut = Litematic::from_bytes(&buf).unwrap();
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!("minecraft:grass_block"; "snowy" => "false"));
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), block!());
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!());
        new_donut.to_bytes().unwrap();
    }

    #[test]
    #[wasm_bindgen_test]
    fn macros() {
        assert_eq!(
            BlockState {
                name: Cow::Borrowed("minecraft:air"),
                properties: None,
            },
            block!(),
        );
        assert_eq!(
            BlockState {
                name: Cow::Borrowed("minecraft:stone"),
                properties: None,
            },
            block!("minecraft:stone"),
        );
        assert_eq!(
            BlockState {
                name: Cow::Borrowed("minecraft:grass_block"),
                properties: Some(HashMap::from([
                    (Cow::Borrowed("snowy"), Cow::Borrowed("false")),
                ])),
            },
            block!("minecraft:grass_block"; "snowy" => "false"),
        );
        assert_eq!(
            BlockState {
                name: Cow::Borrowed("minecraft:repeater"),
                properties: Some(HashMap::from([
                    (Cow::Borrowed("delay"), Cow::Borrowed("2")),
                    (Cow::Borrowed("facing"), Cow::Borrowed("south")),
                    (Cow::Borrowed("locked"), Cow::Borrowed("false")),
                    (Cow::Borrowed("powered"), Cow::Borrowed("false")),
                ])),
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
