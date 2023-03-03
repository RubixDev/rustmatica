#![warn(rust_2018_idioms)]

#[cfg(feature = "chrono")]
pub extern crate chrono;
#[allow(unused_imports)]
#[macro_use(nbt)]
pub extern crate fastnbt;

#[macro_use]
pub mod block_state;
#[macro_use]
pub mod entity;
pub mod error;
mod litematic;
mod region;
mod schema;
pub mod util;
#[macro_use]
mod tile_entities;

pub use block_state::BlockState;
pub use entity::Entity;
pub use litematic::*;
pub use region::*;
pub use tile_entities::TileEntity;

#[cfg(test)]
mod tests;
