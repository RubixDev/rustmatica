use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn volume(&self) -> usize {
        self.x.abs() as usize * self.y.abs() as usize * self.z.abs() as usize
    }

    pub fn abs(&self) -> UVec3 {
        UVec3 {
            x: self.x.abs() as usize,
            y: self.y.abs() as usize,
            z: self.z.abs() as usize,
        }
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct UVec3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
impl UVec3 {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn volume(&self) -> usize {
        self.x * self.y * self.z
    }
}

// Manually implement Serialize for UVec3 so the values are saved as Int (u32) not Long (u64)
impl Serialize for UVec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ser = serializer.serialize_struct("UVec3", 3)?;
        ser.serialize_field("x", &(self.x as u32))?;
        ser.serialize_field("y", &(self.y as u32))?;
        ser.serialize_field("z", &(self.z as u32))?;
        ser.end()
    }
}

macro_rules! vec_debug {
    ($type:ty) => {
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {}, {})", self.x, self.y, self.z)
            }
        }
    };
}
vec_debug!(Vec3);
vec_debug!(UVec3);

#[cfg(feature = "chrono")]
pub(crate) fn current_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}
#[cfg(all(not(target_family = "wasm"), not(feature = "chrono")))]
pub(crate) fn current_time() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
#[cfg(all(target_family = "wasm", not(feature = "chrono")))]
pub(crate) fn current_time() -> i64 {
    js_sys::Date::now() as i64
}
