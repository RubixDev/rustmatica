use serde::{ser::SerializeMap, Serialize};

use super::Entity;

impl<'a> Serialize for Entity<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.as_other() {
            Self::Other {
                id,
                uuid,
                properties,
            } => {
                let mut state = serializer.serialize_map(Some(properties.len() + 2))?;
                state.serialize_entry("id", &id)?;
                state.serialize_entry("UUID", &uuid)?;
                for (key, value) in properties.iter() {
                    state.serialize_entry(key, value)?;
                }
                state.end()
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> PartialEq for Entity<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self.as_other(), other.as_other()) {
            (
                Self::Other {
                    id: l_id,
                    uuid: l_uuid,
                    properties: l_properties,
                },
                Self::Other {
                    id: r_id,
                    uuid: r_uuid,
                    properties: r_properties,
                },
            ) => l_id == r_id && l_uuid == r_uuid && l_properties == r_properties,
            _ => unreachable!(),
        }
    }
}
impl<'a> Eq for Entity<'a> {}
