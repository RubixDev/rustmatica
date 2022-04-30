#[macro_use]
#[cfg(feature = "entity-list")]
mod macros;
#[cfg(feature = "entity-list")]
mod list;
#[cfg(feature = "entity-list")]
mod traits;

#[cfg(feature = "entity-list")]
pub use list::*;

#[cfg(not(feature = "entity-list"))]
use core::marker::PhantomData;
#[cfg(not(feature = "entity-list"))]
use std::{borrow::Cow, collections::HashMap};
#[cfg(not(feature = "entity-list"))]
use serde::{Serialize, Deserialize, ser::SerializeMap, de::Visitor};

#[cfg(feature = "entity-list")]
#[macro_export]
macro_rules! entity {
    ($entity:ident $props:tt) => {
        $crate::Entity::$entity $props
    };
    ($id:expr, $uuid:expr; $($prop:expr => $val:expr),* $(,)?) => {
        $crate::Entity::Other {
            id: $id.into(),
            uuid: $uuid,
            properties: std::collections::HashMap::from([$(
                ($prop.into(), $val),
            )*]),
        }
    };
}

#[cfg(not(feature = "entity-list"))]
#[macro_export]
macro_rules! entity {
    ($id:expr, $uuid:expr; $($prop:expr => $val:expr),* $(,)?) => {
        $crate::Entity {
            id: $id.into(),
            uuid: $uuid,
            properties: std::collections::HashMap::from([$(
                ($prop.into(), $val),
            )*]),
        }
    };
}

#[cfg(not(feature = "entity-list"))]
#[derive(Debug, Clone, PartialEq)]
pub struct Entity<'a> {
    pub id: Cow<'a, str>,
    pub uuid: u128,
    pub properties: HashMap<Cow<'a, str>, fastnbt::Value>,
}

#[cfg(not(feature = "entity-list"))]
impl <'de, 'a> Deserialize<'de> for Entity<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct _Visitor<'de, 'a> {
            marker: PhantomData<Entity<'a>>,
            lifetime: PhantomData<&'de ()>,
        }
        impl <'de, 'a> Visitor<'de> for _Visitor<'de, 'a> {
            type Value = Entity<'a>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Entity")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where A: serde::de::MapAccess<'de>, {
                let mut id: Option<Cow<'a, str>> = None;
                let mut uuid = None;
                let mut properties: HashMap<Cow<'a, str>, fastnbt::Value> = HashMap::new();
                while let Some(key) = map.next_key::<Cow<'a, str>>()? {
                    match key.as_ref() {
                        "id" => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        },
                        "UUID" => {
                            if uuid.is_some() {
                                return Err(serde::de::Error::duplicate_field("UUID"));
                            }
                            uuid = Some(map.next_value()?);
                        },
                        _ => {
                            properties.insert(key, map.next_value()?);
                        },
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let uuid = uuid.ok_or_else(|| serde::de::Error::missing_field("UUID"))?;
                Ok(Self::Value { id, uuid, properties })
            }
        }

        deserializer.deserialize_map(_Visitor {
            marker: PhantomData::<Entity<'a>>,
            lifetime: PhantomData,
        })
    }
}

#[cfg(not(feature = "entity-list"))]
impl <'a> Serialize for Entity<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut state = serializer.serialize_map(Some(self.properties.len() + 2))?;
        state.serialize_entry("id", &self.id)?;
        state.serialize_entry("UUID", &self.uuid)?;
        for (key, value) in self.properties.iter() {
            state.serialize_entry(key, value)?;
        }
        state.end()
    }
}
