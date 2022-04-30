macro_rules! entities {
    ($($str:expr, $name:ident - $($prop:ident : $type:ty $(as $opt:ident)?),+;)+) => {
        use core::marker::PhantomData;
        use std::{collections::HashMap, borrow::Cow};
        use serde::{Deserialize, de::Visitor};

        #[derive(Debug, Clone)]
        #[allow(non_snake_case)]
        pub enum Entity<'a> {
            $(
                $name { uuid: u128, $($prop: prop_type!($type $(, $opt)?)),+ },
            )+
            Other { id: Cow<'a, str>, uuid: u128, properties: HashMap<Cow<'a, str>, fastnbt::Value> },
        }

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
                    where A: serde::de::MapAccess<'de> {
                        let mut id: Option<Cow<'a, str>> = None;
                        let mut uuid = None;
                        let mut properties: HashMap<Cow<'a, str>, Value> = HashMap::new();
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
                        Ok(match id.as_ref() {
                            $(
                                $str => make_entity!($name, id, uuid, properties; $($prop $(as $opt)?),+),
                            )+
                            _ => Self::Value::Other { id, uuid, properties },
                        })
                    }
                }

                deserializer.deserialize_map(_Visitor {
                    marker: PhantomData::<Entity<'a>>,
                    lifetime: PhantomData,
                })
            }
        }

        impl <'a> Entity<'a> {
            pub fn as_other(&self) -> Self {
                match self {
                    $(
                        Self::$name { uuid, $($prop),+ } => Self::Other {
                            id: $str.into(),
                            uuid: uuid.to_owned(),
                            properties: {
                                let mut map = HashMap::new();
                                $(
                                    make_entity_prop!(@map map, $prop $(as $opt)?);
                                )+
                                map
                            },
                        },
                    )+
                    Self::Other { id, uuid, properties } => Self::Other {
                        id: id.to_owned(),
                        uuid: uuid.to_owned(),
                        properties: properties.to_owned()
                    },
                }
            }
        }
    };
}

macro_rules! prop_type {
    ($type:ty) => { $type };
    ($type:ty, $opt:ident) => { Option<$type> };
}

macro_rules! make_entity {
    ($entity:ident, $id:ident, $uuid:ident, $props:ident; $($prop:ident $(as $opt:ident)?),+) => {
        Self::Value::$entity {
            $(
                $prop: match $props.get(stringify!($prop)) {
                    Some(val) => match fastnbt::from_value(&val).ok() {
                        Some(val) => make_entity_prop!(@some val $(as $opt)?),
                        None => return Ok(Self::Value::Other { id: $id, uuid: $uuid, properties: $props }),
                    },
                    None => make_entity_prop!(@none $prop $(as $opt)?, $id, $uuid, $props),
                },
            )+
            uuid: $uuid,
        }
    };
}

macro_rules! make_entity_prop {
    (@none $prop:ident, $id:ident, $uuid:ident, $props:ident) => {
        return Ok(Self::Value::Other { id: $id, uuid: $uuid, properties: $props })
    };
    (@none $prop:ident as $opt:ident, $id:ident, $uuid:ident, $props:ident) => { None };
    (@some $val:ident) => { $val };
    (@some $val:ident as $opt:ident) => { Some($val) };
    (@map $map:ident, $prop:ident) => {
        $map.insert(stringify!($prop).into(), fastnbt::to_value($prop).unwrap());
    };
    (@map $map:ident, $prop:ident as $opt:ident) => {
        #[allow(non_snake_case)]
        if let Some($prop) = $prop {
            $map.insert(stringify!($prop).into(), fastnbt::to_value($prop).unwrap());
        }
    };
}
