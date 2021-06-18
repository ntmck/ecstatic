pub mod component_factory;
pub use component_factory::ComponentFactory;

use std::vec::Vec;
use std::collections::{HashSet, VecDeque};

use crate::data::*;

//storage for the underlying type of a component.
pub struct ComponentBank {
    positions: Vec<Option<Vec3>>,
    rotations: Vec<Option<Vec3>>,
    inits: Vec<Option<Init>>,
    updates: Vec<Option<Update>>,
    destroys: Vec<Option<Destroy>>,
}

//indicies into ComponentBank
pub struct PackedBank {
    positions: HashSet<usize>,
    rotations: HashSet<usize>,
    inits: HashSet<usize>,
    updates: HashSet<usize>,
    destroys: HashSet<usize>,
}

pub struct FreeIndices {
    pos_next: usize,
    rot_next: usize,
    init_next: usize,
    update_next: usize,
    destroy_next: usize,
    positions: VecDeque<usize>,
    rotations: VecDeque<usize>,
    inits: VecDeque<usize>,
    updates: VecDeque<usize>,
    destroys: VecDeque<usize>,
}

pub struct ComponentStorage {
    bank: ComponentBank,
    packed: PackedBank,
    free: FreeIndices,
}
