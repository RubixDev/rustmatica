pub mod util;
pub mod error;
mod schema;
mod litematic;
mod region;

pub use litematic::*;
pub use region::*;
pub use schema::BlockState;

#[cfg(test)]
mod tests {
    use crate::{Litematic, schema::BlockState, util::UVec3};

    #[test]
    fn read_write() -> Result<(), Box<dyn std::error::Error>> {
        let mut donut = Litematic::read_file("./test_files/donut.litematic")?;
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::new("minecraft:air"));
        donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::new("minecraft:grass_block"));
        assert_eq!(donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::new("minecraft:grass_block"));
        donut.write_file("./test_files/donut_modified.litematic")?;

        let mut new_donut = Litematic::read_file("./test_files/donut_modified.litematic")?;
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::new("minecraft:grass_block"));
        new_donut.regions[0].set_block(UVec3::new(1, 1, 1), BlockState::new("minecraft:air"));
        assert_eq!(new_donut.regions[0].get_block(UVec3::new(1, 1, 1)), &BlockState::new("minecraft:air"));
        new_donut.write_file("./test_files/donut.litematic")?;

        Ok(())
    }
}
