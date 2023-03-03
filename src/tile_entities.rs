use crate::util::UVec3;
use fastnbt::Value;
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, marker::PhantomData};

#[macro_export]
macro_rules! tile_entity {
    ($x:expr, $y:expr, $z:expr; $($key:expr => $value:expr),* $(,)?) => {
        tile_entity!($crate::util::UVec3::new($x, $y, $z); $($key => $value),*)
    };
    ($pos:expr; $($key:expr => $value:expr),* $(,)?) => {
        $crate::TileEntity {
            pos: $pos,
            properties: std::collections::HashMap::from([$(
                ($key.into(), $value)
            ),*]),
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct TileEntity<'a> {
    pub pos: UVec3,
    pub properties: HashMap<Cow<'a, str>, Value>,
}

impl<'de, 'a> Deserialize<'de> for TileEntity<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct _Visitor<'de, 'a> {
            marker: PhantomData<TileEntity<'a>>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de, 'a> Visitor<'de> for _Visitor<'de, 'a> {
            type Value = TileEntity<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("TileEntity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                let mut z = None;
                let mut properties = HashMap::new();
                while let Some(key) = map.next_key::<Cow<'a, str>>()? {
                    match key.as_ref() {
                        "x" => {
                            if x.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x = Some(map.next_value()?);
                        }
                        "y" => {
                            if y.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y = Some(map.next_value()?);
                        }
                        "z" => {
                            if z.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z = Some(map.next_value()?);
                        }
                        _ => {
                            properties.insert(key, map.next_value()?);
                        }
                    }
                }
                let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;
                let z = z.ok_or_else(|| serde::de::Error::missing_field("z"))?;
                Ok(Self::Value {
                    pos: UVec3::new(x, y, z),
                    properties,
                })
            }
        }
        deserializer.deserialize_map(_Visitor {
            marker: PhantomData::<TileEntity<'a>>,
            lifetime: PhantomData,
        })
    }
}

impl<'a> Serialize for TileEntity<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ser = serializer.serialize_map(Some(self.properties.len() + 3))?;
        ser.serialize_entry("x", &self.pos.x)?;
        ser.serialize_entry("y", &self.pos.y)?;
        ser.serialize_entry("z", &self.pos.z)?;
        for (key, value) in self.properties.iter() {
            ser.serialize_entry(key, value)?;
        }
        ser.end()
    }
}
