pub mod types;
mod list;
mod ser;
mod de;

pub use list::BlockState;
use crate::schema;

impl <'a> PartialEq for BlockState<'a> {
    fn eq(&self, other: &Self) -> bool {
        schema::BlockState::from(self) == schema::BlockState::from(other)
    }
}
impl <'a> Eq for BlockState<'a> {}

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
