/*
    Re-export all new data types.
*/
pub mod vec3;
pub use vec3::Vec3;
pub type Init = fn();
pub type Update = fn(u64); //delta time parameter.
pub type Destroy = fn();
