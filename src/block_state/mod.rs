#[cfg(feature = "lists")]
pub mod types;
#[macro_use]
#[cfg(feature = "lists")]
mod macros;
#[cfg(feature = "lists")]
mod list;
#[cfg(feature = "lists")]
mod traits;

#[cfg(feature = "lists")]
pub use list::*;

#[cfg(not(feature = "lists"))]
use serde::{Serialize, Deserialize};
#[cfg(not(feature = "lists"))]
use std::{borrow::Cow, collections::HashMap};

#[cfg(feature = "lists")]
#[macro_export]
macro_rules! block {
    () => { BlockState::Air };
    ($block:ident) => { BlockState::$block };
    ($block:ident $props:tt) => { BlockState::$block $props };
    ($name:expr) => {
        BlockState::Other {
            name: std::borrow::Cow::Borrowed($name),
            properties: None,
        }
    };
    ($name:expr; $($key:expr => $val:expr),+ $(,)?) => {
        BlockState::Other {
            name: std::borrow::Cow::Borrowed($name),
            properties: Some(std::collections::HashMap::from([
                $(
                    (std::borrow::Cow::Borrowed($key), std::borrow::Cow::Borrowed($val))
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
        BlockState {
            name: std::borrow::Cow::Borrowed($name),
            properties: None,
        }
    };
    ($name:expr; $($key:expr => $val:expr),+ $(,)?) => {
        BlockState {
            name: std::borrow::Cow::Borrowed($name),
            properties: Some(std::collections::HashMap::from([
                $(
                    (std::borrow::Cow::Borrowed($key), std::borrow::Cow::Borrowed($val))
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
