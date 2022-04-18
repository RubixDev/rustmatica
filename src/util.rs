use serde::{Serialize, Deserialize};

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
        return self.x.abs() as usize
             * self.y.abs() as usize
             * self.z.abs() as usize;
    }

    pub fn abs(&self) -> UVec3 { UVec3 {
        x: self.x.abs() as usize,
        y: self.y.abs() as usize,
        z: self.z.abs() as usize,
    } }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
