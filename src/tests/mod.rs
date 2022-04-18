use fastnbt::Value;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{Litematic, util::UVec3};

#[cfg(feature = "lists")]
mod with_lists;
#[cfg(not(feature = "lists"))]
mod without_lists;

#[test]
#[wasm_bindgen_test]
fn tile_entities() {
    let mut axolotl = Litematic::from_bytes(include_bytes!("../../test_files/axolotl.litematic")).unwrap();
    {
        let region = axolotl.regions.get_mut(0).unwrap();
        assert_eq!(region.get_tile_entity(UVec3::new(2, 2, 2)), None);
        // TODO: test chest when fastnbt has nbt! macro
        assert_eq!(region.get_tile_entity(UVec3::new(1, 1, 1)), Some(&tile_entity!(
            1, 1, 1;
            "Text1" => Value::String(r#"{"text":"a"}"#.to_string()),
            "Text2" => Value::String(r#"{"text":"b"}"#.to_string()),
            "Text3" => Value::String(r#"{"text":"c"}"#.to_string()),
            "Text4" => Value::String(r#"{"text":"d"}"#.to_string()),
            "Color" => Value::String("black".to_string()),
            "GlowingText" => Value::Byte(0),
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
