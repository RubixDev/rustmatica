#[cfg(feature = "entity-list")]
mod with_list {
    use std::collections::HashMap;

    use fastnbt::Value;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Litematic, Entity};

    #[test]
    #[wasm_bindgen_test]
    fn read_write() {
        let axolotl = Litematic::from_bytes(include_bytes!("../../test_files/axolotl.litematic")).unwrap();
        let region = &axolotl.regions[0];
        assert_eq!(
            region.entities.iter().find(|e| matches!(e, Entity::ItemFrame { .. })),
            Some(&Entity::ItemFrame {
                uuid: 154833529407457947883461787053576218097,
                Air: 300,
                Facing: 5,
                FallDistance: 0.0,
                Fire: -1,
                Fixed: false,
                Invisible: false,
                Invulnerable: false,
                Item: Some(Value::Compound(HashMap::from([
                    ("Count".into(), Value::Byte(1)),
                    ("id".into(), Value::String("minecraft:furnace".into())),
                ]))),
                ItemDropChance: Some(1.0),
                ItemRotation: Some(0),
                Motion: Value::List(vec![
                    Value::Double(0.0),
                    Value::Double(0.0),
                    Value::Double(0.0),
                ]),
                OnGround: false,
                PortalCooldown: 0,
                Pos: Value::List(vec![
                    Value::Double(-0.96875),
                    Value::Double(1.5),
                    Value::Double(1.5),
                ]),
                Rotation: Value::List(vec![
                    Value::Float(270.0),
                    Value::Float(0.0),
                ]),
                TileX: -2,
                TileY: -59,
                TileZ: 7,
                CustomName: None,
                CustomNameVisible: None,
                Glowing: None,
                HasVisualFire: None,
                NoGravity: None,
                Passengers: None,
                Silent: None,
                Tags: None,
                TicksFrozen: None,
            })
        );

        let axolotl_2 = Litematic::from_bytes(&axolotl.to_bytes().unwrap()[..]).unwrap();
        assert_eq!(region.entities, axolotl_2.regions[0].entities);
    }

    #[test]
    #[wasm_bindgen_test]
    fn macros() {
        assert_eq!(
            Entity::Tnt {
                uuid: 1234,
                Fuse: 123,
                Pos: Value::Byte(0),
                Motion: Value::Byte(0),
                Rotation: Value::Byte(0),
                FallDistance: 1.0,
                Fire: 321,
                Air: 42,
                OnGround: false,
                Invulnerable: true,
                PortalCooldown: 1001,
                CustomName: Some("Boom".into()),
                CustomNameVisible: Some(true),
                Silent: Some(true),
                NoGravity: None,
                Glowing: None,
                TicksFrozen: None,
                HasVisualFire: None,
                Tags: None,
                Passengers: None,
            },
            entity!(
                Tnt {
                    uuid: 1234,
                    Fuse: 123,
                    Pos: Value::Byte(0),
                    Motion: Value::Byte(0),
                    Rotation: Value::Byte(0),
                    FallDistance: 1.0,
                    Fire: 321,
                    Air: 42,
                    OnGround: false,
                    Invulnerable: true,
                    PortalCooldown: 1001,
                    CustomName: Some("Boom".into()),
                    CustomNameVisible: Some(true),
                    Silent: Some(true),
                    NoGravity: None,
                    Glowing: None,
                    TicksFrozen: None,
                    HasVisualFire: None,
                    Tags: None,
                    Passengers: None,
                }
            )
        );
        assert_eq!(
            Entity::Other {
                id: "minecraft:invalid".into(),
                uuid: 1234,
                properties: HashMap::new(),
            },
            entity!(
                "minecraft:invalid",
                1234;
            )
        );
        assert_eq!(
            Entity::Other {
                id: "minecraft:empty".into(),
                uuid: 1234,
                properties: HashMap::from([
                    ("key1".into(), Value::Byte(1)),
                    ("key2".into(), Value::String("value".into())),
                ]),
            },
            entity!(
                "minecraft:empty",
                1234;
                "key1" => Value::Byte(1),
                "key2" => Value::String("value".into()),
            )
        )
    }
}

#[cfg(not(feature = "entity-list"))]
mod without_list {
    use std::{borrow::Cow, collections::HashMap};

    use fastnbt::Value;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Litematic, Entity};

    #[test]
    #[wasm_bindgen_test]
    fn read_write() {
        let axolotl = Litematic::from_bytes(include_bytes!("../../test_files/axolotl.litematic")).unwrap();
        let region = &axolotl.regions[0];
        assert_eq!(
            region.entities.iter().find(|e| e.id == "minecraft:item_frame"),
            Some(&Entity {
                id: Cow::Borrowed("minecraft:item_frame"),
                uuid: 154833529407457947883461787053576218097,
                properties: HashMap::from([
                    (Cow::Borrowed("Air"), Value::Short(300)),
                    (Cow::Borrowed("Facing"), Value::Byte(5)),
                    (Cow::Borrowed("FallDistance"), Value::Float(0.0)),
                    (Cow::Borrowed("Fire"), Value::Short(-1)),
                    (Cow::Borrowed("Fixed"), Value::Byte(0)),
                    (Cow::Borrowed("Invisible"), Value::Byte(0)),
                    (Cow::Borrowed("Invulnerable"), Value::Byte(0)),
                    (Cow::Borrowed("Item"), Value::Compound(HashMap::from([
                        ("Count".into(), Value::Byte(1)),
                        ("id".into(), Value::String("minecraft:furnace".into())),
                    ]))),
                    (Cow::Borrowed("ItemDropChance"), Value::Float(1.0)),
                    (Cow::Borrowed("ItemRotation"), Value::Byte(0)),
                    (Cow::Borrowed("Motion"), Value::List(vec![
                        Value::Double(0.0),
                        Value::Double(0.0),
                        Value::Double(0.0),
                    ])),
                    (Cow::Borrowed("OnGround"), Value::Byte(0)),
                    (Cow::Borrowed("PortalCooldown"), Value::Int(0)),
                    (Cow::Borrowed("Pos"), Value::List(vec![
                        Value::Double(-0.96875),
                        Value::Double(1.5),
                        Value::Double(1.5),
                    ])),
                    (Cow::Borrowed("Rotation"), Value::List(vec![
                        Value::Float(270.0),
                        Value::Float(0.0),
                    ])),
                    (Cow::Borrowed("TileX"), Value::Int(-2)),
                    (Cow::Borrowed("TileY"), Value::Int(-59)),
                    (Cow::Borrowed("TileZ"), Value::Int(7)),
                ]),
            })
        );
        // TODO: test axolotl with nbt! macro

        let axolotl_2 = Litematic::from_bytes(&axolotl.to_bytes().unwrap()[..]).unwrap();
        assert_eq!(region.entities, axolotl_2.regions[0].entities);
    }

    #[test]
    #[wasm_bindgen_test]
    fn macros() {
        assert_eq!(
            Entity {
                id: "minecraft:invalid".into(),
                uuid: 1234,
                properties: HashMap::new(),
            },
            entity!(
                "minecraft:invalid",
                1234;
            )
        );
        assert_eq!(
            Entity {
                id: "minecraft:empty".into(),
                uuid: 5678,
                properties: HashMap::from([
                    ("key1".into(), Value::Byte(1)),
                    ("key2".into(), Value::String("value".into())),
                ]),
            },
            entity!(
                "minecraft:empty",
                5678;
                "key1" => Value::Byte(1),
                "key2" => Value::String("value".into()),
            )
        )
    }
}
