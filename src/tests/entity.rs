#[cfg(feature = "entity-list")]
mod with_list {
    use std::collections::HashMap;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Entity, Litematic};

    #[test]
    #[wasm_bindgen_test]
    fn read_write() {
        let axolotl =
            Litematic::from_bytes(include_bytes!("../../test_files/axolotl.litematic")).unwrap();
        let region = &axolotl.regions[0];
        assert_eq!(
            region
                .entities
                .iter()
                .find(|e| matches!(e, Entity::ItemFrame { .. })),
            Some(&Entity::ItemFrame {
                uuid: 154833529407457947883461787053576218097,
                Air: 300,
                Facing: 5,
                FallDistance: 0.0,
                Fire: -1,
                Fixed: false,
                Invisible: false,
                Invulnerable: false,
                Item: Some(nbt!({
                    "Count": 1_u8,
                    "id": "minecraft:furnace",
                })),
                ItemDropChance: Some(1.0),
                ItemRotation: Some(0),
                Motion: nbt!([0.0, 0.0, 0.0]),
                OnGround: false,
                PortalCooldown: 0,
                Pos: nbt!([-0.96875, 1.5, 1.5]),
                Rotation: nbt!([270.0_f32, 0.0_f32]),
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
        // assert_eq!(
        //     region.entities.iter().find(|e| matches!(e, Entity::Axolotl { .. })),
        //     Some(&Entity::Axolotl {
        //         uuid: 307716075036743941152627606223512221703,
        //         Variant: 0,
        //         FromBucket: true,
        //         InLove: 0,
        //         LoveCause: None,
        //         Age: 0,
        //         ForcedAge: 0,
        //         CanPickUpLoot: false,
        //         PersistenceRequired: false,
        //         ArmorItems: nbt!([{}, {}, {}, {}]),
        //         HandItems: nbt!([{}, {}]),
        //         ArmorDropChances: nbt!([
        //             0.08500000089406967_f32,
        //             0.08500000089406967_f32,
        //             0.08500000089406967_f32,
        //             0.08500000089406967_f32,
        //         ]),
        //         HandDropChances: nbt!([
        //             0.08500000089406967_f32,
        //             0.08500000089406967_f32,
        //         ]),
        //         Leash: Value, // TODO: this should be Option<Value>
        //         LeftHanded: false,
        //         DeathLootTable: None,
        //         DeathLootTableSeed: None,
        //         NoAI: None,
        //         Health: 14,
        //         HurtTime: 0,
        //         HurtByTimestamp: 0,
        //         DeathTime: 0,
        //         AbsorptionAmount: 0,
        //         Attributes: nbt!([{
        //             "Base": 1.0,
        //             "Name": "minecraft:generic.movement_speed",
        //         }]),
        //         ActiveEffects: None,
        //         FallFlying: false,
        //         Pos: nbt!([-0.5, 0.0, 1.5]),
        //         Motion: nbt!([0.0, 0.0, 0.0]),
        //         Rotation: nbt!([
        //             -107.68714904785156_f32,
        //             0_f32,
        //         ]),
        //         FallDistance: 0,
        //         Fire: -1,
        //         Air: 6000,
        //         OnGround: false,
        //         Invulnerable: false,
        //         PortalCooldown: 0,
        //         CustomName: None,
        //         CustomNameVisible: None,
        //         Silent: None,
        //         NoGravity: None,
        //         Glowing: None,
        //         TicksFrozen: None,
        //         HasVisualFire: None,
        //         Tags: None,
        //         Passengers: None,
        //     })
        // );

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
                Pos: nbt!(0_u8),
                Motion: nbt!(0_u8),
                Rotation: nbt!(0_u8),
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
            entity!(Tnt {
                uuid: 1234,
                Fuse: 123,
                Pos: nbt!(0_u8),
                Motion: nbt!(0_u8),
                Rotation: nbt!(0_u8),
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
            })
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
                    ("key1".into(), nbt!(1_u8)),
                    ("key2".into(), nbt!("value")),
                ]),
            },
            entity!(
                "minecraft:empty",
                1234;
                "key1" => nbt!(1_u8),
                "key2" => nbt!("value"),
            )
        )
    }
}

#[cfg(not(feature = "entity-list"))]
mod without_list {
    use std::collections::HashMap;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Entity, Litematic};

    #[test]
    #[wasm_bindgen_test]
    fn read_write() {
        let axolotl =
            Litematic::from_bytes(include_bytes!("../../test_files/axolotl.litematic")).unwrap();
        let region = &axolotl.regions[0];
        assert_eq!(
            region
                .entities
                .iter()
                .find(|e| e.id == "minecraft:item_frame"),
            Some(&Entity {
                id: "minecraft:item_frame".into(),
                uuid: 154833529407457947883461787053576218097,
                properties: HashMap::from([
                    ("Air".into(), nbt!(300_i16)),
                    ("Facing".into(), nbt!(5_u8)),
                    ("FallDistance".into(), nbt!(0_f32)),
                    ("Fire".into(), nbt!(-1_i16)),
                    ("Fixed".into(), nbt!(false)),
                    ("Invisible".into(), nbt!(false)),
                    ("Invulnerable".into(), nbt!(false)),
                    (
                        "Item".into(),
                        nbt!({
                            "Count": 1_u8,
                            "id": "minecraft:furnace",
                        })
                    ),
                    ("ItemDropChance".into(), nbt!(1_f32)),
                    ("ItemRotation".into(), nbt!(0_u8)),
                    ("Motion".into(), nbt!([0.0, 0.0, 0.0])),
                    ("OnGround".into(), nbt!(false)),
                    ("PortalCooldown".into(), nbt!(0)),
                    ("Pos".into(), nbt!([-0.96875, 1.5, 1.5])),
                    ("Rotation".into(), nbt!([270_f32, 0_f32])),
                    ("TileX".into(), nbt!(-2)),
                    ("TileY".into(), nbt!(-59)),
                    ("TileZ".into(), nbt!(7)),
                ]),
            })
        );
        assert_eq!(
            region.entities.iter().find(|e| e.id == "minecraft:axolotl"),
            Some(&Entity {
                id: "minecraft:axolotl".into(),
                uuid: 307716075036743941152627606223512221703,
                properties: HashMap::from([
                    ("AbsorptionAmount".into(), nbt!(0_f32)),
                    ("Age".into(), nbt!(0)),
                    ("Air".into(), nbt!(6000_i16)),
                    (
                        "ArmorDropChances".into(),
                        nbt!([
                            0.08500000089406967_f32,
                            0.08500000089406967_f32,
                            0.08500000089406967_f32,
                            0.08500000089406967_f32,
                        ])
                    ),
                    ("ArmorItems".into(), nbt!([{}, {}, {}, {}])),
                    (
                        "Attributes".into(),
                        nbt!([{
                            "Base": 1.0,
                            "Name": "minecraft:generic.movement_speed",
                        }])
                    ),
                    ("Brain".into(), nbt!({ "memories": {} })),
                    ("CanPickUpLoot".into(), nbt!(false)),
                    ("DeathTime".into(), nbt!(0_i16)),
                    ("FallDistance".into(), nbt!(0_f32)),
                    ("FallFlying".into(), nbt!(false)),
                    ("Fire".into(), nbt!(-1_i16)),
                    ("ForcedAge".into(), nbt!(0)),
                    ("FromBucket".into(), nbt!(true)),
                    (
                        "HandDropChances".into(),
                        nbt!([0.08500000089406967_f32, 0.08500000089406967_f32,])
                    ),
                    ("HandItems".into(), nbt!([{}, {}])),
                    ("Health".into(), nbt!(14_f32)),
                    ("HurtByTimestamp".into(), nbt!(0)),
                    ("HurtTime".into(), nbt!(0_i16)),
                    ("InLove".into(), nbt!(0)),
                    ("Invulnerable".into(), nbt!(false)),
                    ("LeftHanded".into(), nbt!(false)),
                    ("Motion".into(), nbt!([0.0, 0.0, 0.0])),
                    ("OnGround".into(), nbt!(false)),
                    ("PersistenceRequired".into(), nbt!(false)),
                    ("PortalCooldown".into(), nbt!(0)),
                    ("Pos".into(), nbt!([-0.5, 0.0, 1.5])),
                    ("Rotation".into(), nbt!([-107.68714904785156_f32, 0_f32,])),
                    ("Variant".into(), nbt!(0)),
                ]),
            })
        );

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
                    ("key1".into(), nbt!(1_u8)),
                    ("key2".into(), nbt!("value")),
                ]),
            },
            entity!(
                "minecraft:empty",
                5678;
                "key1" => nbt!(1_u8),
                "key2" => nbt!("value"),
            )
        )
    }
}
