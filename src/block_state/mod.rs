#[macro_use]
#[cfg(feature = "lists")]
mod macros;
#[cfg(feature = "lists")]
mod list;
#[cfg(feature = "lists")]
mod traits;
#[cfg(feature = "lists")]
pub mod types;

#[cfg(feature = "lists")]
pub use list::*;

#[cfg(not(feature = "lists"))]
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "lists"))]
use std::{borrow::Cow, collections::HashMap};

#[cfg(feature = "lists")]
#[macro_export]
macro_rules! block {
    () => { $crate::BlockState::Air };
    ($block:ident) => { $crate::BlockState::$block };
    ($block:ident $props:tt) => { $crate::BlockState::$block $props };
    ($name:expr) => {
        $crate::BlockState::Other {
            name: $name.into(),
            properties: None,
        }
    };
    ($name:expr; $($key:expr => $val:expr),+ $(,)?) => {
        $crate::BlockState::Other {
            name: $name.into(),
            properties: Some(std::collections::HashMap::from([
                $(
                    ($key.into(), $val.into())
                ),+
            ]))
        }
    };
}

#[cfg(not(feature = "lists"))]
#[macro_export]
macro_rules! block {
    () => { block!("minecraft:air") };
    ($name:expr) => {
        $crate::BlockState {
            name: $name.into(),
            properties: None,
        }
    };
    ($name:expr; $($key:expr => $val:expr),+ $(,)?) => {
        $crate::BlockState {
            name: $name.into(),
            properties: Some(std::collections::HashMap::from([
                $(
                    ($key.into(), $val.into())
                ),+
            ]))
        }
    };
}

#[cfg(not(feature = "lists"))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct BlockState<'a> {
    pub name: Cow<'a, str>,
    pub properties: Option<HashMap<Cow<'a, str>, Cow<'a, str>>>,
}
