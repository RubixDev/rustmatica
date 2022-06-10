use serde::{ser::SerializeStruct, Serialize};

use super::BlockState;

impl<'a> Serialize for BlockState<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.as_other() {
            Self::Other { name, properties } => {
                let mut state = serializer.serialize_struct("BlockState", 2)?;
                state.serialize_field("Name", &name)?;
                state.serialize_field("Properties", &properties)?;
                state.end()
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> PartialEq for BlockState<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self.as_other(), other.as_other()) {
            (
                Self::Other {
                    name: l_name,
                    properties: l_properties,
                },
                Self::Other {
                    name: r_name,
                    properties: r_properties,
                },
            ) => l_name == r_name && l_properties == r_properties,
            _ => unreachable!(),
        }
    }
}
impl<'a> Eq for BlockState<'a> {}
