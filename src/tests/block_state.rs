#[cfg(feature = "lists")]
mod with_list {
    use std::collections::HashMap;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{block_state::types::HorizontalDirection, util::UVec3, BlockState, Litematic};

    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn read_write() -> Result<(), Box<dyn std::error::Error>> {
        let mut donut = Litematic::read_file("./test_files/donut.litematic")?;
        assert_eq!(
            donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &BlockState::Air
        );
        donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::GrassBlock { snowy: false });
        assert_eq!(
            donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &BlockState::GrassBlock { snowy: false }
        );
        donut.write_file("./test_files/donut_modified.litematic")?;

        let mut new_donut = Litematic::read_file("./test_files/donut_modified.litematic")?;
        assert_eq!(
            new_donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &BlockState::GrassBlock { snowy: false }
        );
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::Air);
        assert_eq!(
            new_donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &BlockState::Air
        );
        new_donut.write_file("./test_files/donut.litematic")?;

        Ok(())
    }

    #[wasm_bindgen_test]
    #[cfg(target_family = "wasm")]
    fn read_write() {
        let mut donut =
            Litematic::from_bytes(include_bytes!("../../test_files/donut.litematic")).unwrap();
        assert_eq!(
            donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &BlockState::Air
        );
        donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::GrassBlock { snowy: false });
        assert_eq!(
            donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &BlockState::GrassBlock { snowy: false }
        );
        let buf = donut.to_bytes().unwrap();

        let mut new_donut = Litematic::from_bytes(&buf).unwrap();
        assert_eq!(
            new_donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &BlockState::GrassBlock { snowy: false }
        );
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::Air);
        assert_eq!(
            new_donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &BlockState::Air
        );
        new_donut.to_bytes().unwrap();
    }

    #[test]
    #[wasm_bindgen_test]
    fn block_state_eq() {
        assert_eq!(
            BlockState::Air,
            BlockState::Other {
                name: "minecraft:air".into(),
                properties: None
            },
        );
        assert_eq!(
            BlockState::GrassBlock { snowy: true },
            BlockState::Other {
                name: "minecraft:grass_block".into(),
                properties: Some(HashMap::from([("snowy".into(), "true".into()),])),
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
                name: "minecraft:repeater".into(),
                properties: Some(HashMap::from([
                    ("delay".into(), "3".into()),
                    ("facing".into(), "west".into()),
                    ("locked".into(), "true".into()),
                    ("powered".into(), "true".into()),
                ])),
            },
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn macros() {
        assert_eq!(BlockState::Air, block!());
        assert_eq!(BlockState::Stone, block!(Stone));
        assert_eq!(
            BlockState::GrassBlock { snowy: false },
            block!(GrassBlock { snowy: false })
        );
        assert_eq!(BlockState::Stone, block!("minecraft:stone"));
        assert_eq!(
            BlockState::GrassBlock { snowy: true },
            block!("minecraft:grass_block"; "snowy" => "true")
        );
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
    use std::collections::HashMap;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{util::UVec3, BlockState, Litematic};

    #[test]
    fn read_write() -> Result<(), Box<dyn std::error::Error>> {
        let mut donut = Litematic::read_file("./test_files/donut.litematic")?;
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!());
        donut.regions[0].set_block(
            UVec3::new(1, 1, 1),
            block!("minecraft:grass_block"; "snowy" => "false"),
        );
        assert_eq!(
            donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &block!("minecraft:grass_block"; "snowy" => "false")
        );
        donut.write_file("./test_files/donut_modified.litematic")?;

        let mut new_donut = Litematic::read_file("./test_files/donut_modified.litematic")?;
        assert_eq!(
            new_donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &block!("minecraft:grass_block"; "snowy" => "false")
        );
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), block!());
        assert_eq!(
            new_donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &block!()
        );
        new_donut.write_file("./test_files/donut.litematic")?;

        Ok(())
    }

    #[wasm_bindgen_test]
    fn read_write_wasm() {
        let mut donut =
            Litematic::from_bytes(include_bytes!("../../test_files/donut.litematic")).unwrap();
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &block!());
        donut.regions[0].set_block(
            UVec3::new(1, 1, 1),
            block!("minecraft:grass_block"; "snowy" => "false"),
        );
        assert_eq!(
            donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &block!("minecraft:grass_block"; "snowy" => "false")
        );
        let buf = donut.to_bytes().unwrap();

        let mut new_donut = Litematic::from_bytes(&buf).unwrap();
        assert_eq!(
            new_donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &block!("minecraft:grass_block"; "snowy" => "false")
        );
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), block!());
        assert_eq!(
            new_donut.regions[0].get_block(UVec3::new(1, 1, 1)),
            &block!()
        );
        new_donut.to_bytes().unwrap();
    }

    #[test]
    #[wasm_bindgen_test]
    fn macros() {
        assert_eq!(
            BlockState {
                name: "minecraft:air".into(),
                properties: None,
            },
            block!(),
        );
        assert_eq!(
            BlockState {
                name: "minecraft:stone".into(),
                properties: None,
            },
            block!("minecraft:stone"),
        );
        assert_eq!(
            BlockState {
                name: "minecraft:grass_block".into(),
                properties: Some(HashMap::from([("snowy".into(), "false".into()),])),
            },
            block!("minecraft:grass_block"; "snowy" => "false"),
        );
        assert_eq!(
            BlockState {
                name: "minecraft:repeater".into(),
                properties: Some(HashMap::from([
                    ("delay".into(), "2".into()),
                    ("facing".into(), "south".into()),
                    ("locked".into(), "false".into()),
                    ("powered".into(), "false".into()),
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
