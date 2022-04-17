macro_rules! blocks {
    ($($str:expr, $name:ident $(-$($prop:ident : $type:ty),+)?);+ $(;)?) => {
        use core::marker::PhantomData;
        use std::{borrow::Cow, collections::HashMap, str::FromStr};
        use serde::{Deserialize, de::Visitor};

        #[derive(Debug, Clone)]
        pub enum BlockState<'a> {
            $(
                $name $({
                    $($prop: $type),+
                })?
            ),+,
            Other { name: Cow<'a, str>, properties: Option<HashMap<Cow<'a, str>, Cow<'a, str>>> }
        }

        impl <'de, 'a> Deserialize<'de> for BlockState<'a> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de> {
                const FIELDS: &[&str] = &["Name", "Properties"];
                enum _Field { Name, Properties }
                impl<'de> Deserialize<'de> for _Field {
                    fn deserialize<D>(deserializer: D) -> Result<_Field, D::Error>
                    where D: serde::Deserializer<'de> {
                        struct _FieldVisitor;

                        impl<'de> Visitor<'de> for _FieldVisitor {
                            type Value = _Field;

                            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                                formatter.write_str("`Name` or `Properties`")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<_Field, E>
                            where E: serde::de::Error {
                                match value {
                                    "Name" => Ok(_Field::Name),
                                    "Properties" => Ok(_Field::Properties),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(_FieldVisitor)
                    }
                }

                struct _Visitor<'de, 'a> {
                    marker: PhantomData<BlockState<'a>>,
                    lifetime: PhantomData<&'de ()>,
                }
                impl <'de, 'a> Visitor<'de> for _Visitor<'de, 'a> {
                    type Value = BlockState<'a>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("BlockState")
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                    where V: serde::de::MapAccess<'de>
                    {
                        let mut name: Option<Cow<'a, str>> = None;
                        let mut properties: Option<HashMap<Cow<'a, str>, Cow<'a, str>>> = None;
                        while let Some(key) = map.next_key()? {
                            match key {
                                _Field::Name => {
                                    if name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    name = Some(map.next_value()?);
                                }
                                _Field::Properties => {
                                    if properties.is_some() {
                                        return Err(serde::de::Error::duplicate_field("properties"));
                                    }
                                    properties = Some(map.next_value()?);
                                }
                            }
                        }
                        let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                        Ok(match name.as_ref() {
                            $(
                                $str => make!($name $(, name, properties; $($prop),+)?)
                            ),+,
                            _ => Self::Value::Other { name, properties },
                        })
                    }
                }

                deserializer.deserialize_struct("BlockState", FIELDS, _Visitor {
                    marker: PhantomData::<BlockState<'a>>,
                    lifetime: PhantomData,
                })
            }
        }

        impl <'a> BlockState<'a> {
            pub fn as_other(&self) -> Self {
                match self {
                    $(
                        Self::$name $({ $($prop),+ })? => Self::Other { name: Cow::Borrowed($str), properties: props!($($($prop),+)?) },
                    )+
                    Self::Other { name, properties } => Self::Other { name: name.to_owned(), properties: properties.to_owned() },
                }
            }
        }
    };
}

macro_rules! make {
    ($block:ident) => { Self::Value::$block };
    ($block:ident, $name:ident, $props:ident; $($prop:ident),+) => {
        match $props.as_ref() {
            Some(props) => Self::Value::$block {
                $(
                    $prop: match props.get(stringify!($prop)) {
                        Some(val) => match <_>::from_str(val).ok() {
                            Some(val) => val,
                            None => return Ok(Self::Value::Other { name: $name, properties: $props }),
                        },
                        None => return Ok(Self::Value::Other { name: $name, properties: $props }),
                    }
                ),+
            },
            None => Self::Value::Other { name: $name, properties: $props },
        }
    };
}

macro_rules! props {
    () => { None };
    ($($prop:ident),+) => {
        Some(HashMap::from([$(
            (Cow::Borrowed(stringify!($prop)), Cow::Owned($prop.to_string()))
        ),+]))
    };
}

macro_rules! enums {
    ($($name:ident => $($variant:ident),+);+ $(;)?) => {
        use strum::{Display, EnumString};

        $(
            #[derive(Debug, Display, EnumString, Clone, PartialEq, Eq)]
            #[strum(serialize_all = "snake_case")]
            pub enum $name { $($variant),+ }
        )+
    };
}
