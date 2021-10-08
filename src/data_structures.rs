pub type Init = fn();
pub type Update = fn(u64); //delta time parameter.
pub type Destroy = fn();


pub struct Position2d {
    pub x: f64,
    pub y: f64,
}
pub struct Rotation2d {
    pub x: f64,
    pub y: f64,
}
