pub mod component_factory;
pub use component_factory::ComponentFactory;

use std::vec::Vec;
use std::collections::HashSet;

use crate::data::*;

pub struct ComponentBank {
    positions: Vec<Option<Vec3>>,
    rotations: Vec<Option<Vec3>>,
    inits: Vec<Option<Init>>,
    updates: Vec<Option<Update>>,
    destroys: Vec<Option<Destroy>>,
}

pub struct PackedBank {
    positions: HashSet<u32>,
    rotations: HashSet<u32>,
    inits: HashSet<u32>,
    updates: HashSet<u32>,
    destroys: HashSet<u32>,
}
