use wasm_bindgen_test::wasm_bindgen_test;

use crate::{Litematic, util::UVec3};

#[test]
#[wasm_bindgen_test]
fn tile_entities() {
    let mut axolotl = Litematic::from_bytes(include_bytes!("../../test_files/axolotl.litematic")).unwrap();
    {
        let region = axolotl.regions.get_mut(0).unwrap();
        assert_eq!(region.get_tile_entity(UVec3::new(2, 2, 2)), None);
        assert_eq!(region.get_tile_entity(UVec3::new(1, 1, 1)), Some(&tile_entity!(
            1, 1, 1;
            "Text1" => nbt!(r#"{"text":"a"}"#),
            "Text2" => nbt!(r#"{"text":"b"}"#),
            "Text3" => nbt!(r#"{"text":"c"}"#),
            "Text4" => nbt!(r#"{"text":"d"}"#),
            "Color" => nbt!("black"),
            "GlowingText" => nbt!(0_u8),
        )));
        assert_eq!(region.get_tile_entity(UVec3::new(1, 1, 0)), Some(&tile_entity!(
            1, 1, 0;
            "Items" => nbt!([
                {
                    "Count": 1_u8,
                    "Slot": 0_u8,
                    "id": "minecraft:axolotl_bucket",
                    "tag": {},
                },
                {
                    "Count": 1_u8,
                    "Slot": 1_u8,
                    "id": "minecraft:stick",
                },
                {
                    "Count": 1_u8,
                    "Slot": 11_u8,
                    "id": "minecraft:stone",
                },
                {
                    "Count": 1_u8,
                    "Slot": 21_u8,
                    "id": "minecraft:chest",
                },
                {
                    "Count": 1_u8,
                    "Slot": 23_u8,
                    "id": "minecraft:oak_sign",
                },
            ]),
        )));
        region.set_tile_entity(tile_entity!(UVec3::new(2, 2, 2);));
        assert_eq!(
            region.get_tile_entity(UVec3::new(2, 2, 2)),
            Some(&tile_entity!(2, 2, 2;))
        );
        region.remove_tile_entity(UVec3::new(2, 2, 2));
        assert_eq!(region.get_tile_entity(UVec3::new(2, 2, 2)), None);
    }

    let axolotl_2 = Litematic::from_bytes(&axolotl.to_bytes().unwrap()).unwrap();
    assert_eq!(
        axolotl.regions[0].get_tile_entity(UVec3::new(1, 1, 0)),
        axolotl_2.regions[0].get_tile_entity(UVec3::new(1, 1, 0))
    );
}

#[test]
#[wasm_bindgen_test]
fn iterator() {
    let donut = Litematic::from_bytes(include_bytes!("../../test_files/donut.litematic")).unwrap();
    let region = &donut.regions[0];
    for (pos, block) in region.blocks() {
        assert_eq!(block, region.get_block(pos));
    }
    assert_eq!(region.blocks().filter(|(_, b)| b == &&block!("minecraft:diamond_block")).count(), 3);
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn time() {
    use std::time::SystemTime;

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64;
    let donut = Litematic::from_bytes(include_bytes!("../../test_files/donut.litematic")).unwrap();
    assert!(1000 > (donut.to_raw().metadata.time_modified - now).abs());
}

#[wasm_bindgen_test]
#[cfg(target_family = "wasm")]
fn time() {
    let now = js_sys::Date::now() as i64;
    let donut = Litematic::from_bytes(include_bytes!("../../test_files/donut.litematic")).unwrap();
    assert!(1000 > (donut.to_raw().metadata.time_modified - now).abs());
}
