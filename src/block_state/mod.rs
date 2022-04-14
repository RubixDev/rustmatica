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
