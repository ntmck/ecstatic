use std::vec::Vec;
use std::any::{Any, TypeId};
use std::any::type_name;
use std::collections::HashMap;

pub struct Packer {
    packed: HashMap<TypeId, Vec<usize>>
}

impl Packer {
    pub fn new() -> Packer {
        Packer { HashMap::new() }
    }

    pub fn iter<T: Any>(&self) -> Iter<usize> {

    }

    pub fn iter_mut<T: Any>(&mut self) -> IterMut<usize> {

    }

    pub fn capacity<T: Any>(&self) -> usize {

    }

    pub fn len<T: Any>(&self) -> usize {

    }

    pub fn pack() {

    }

    pub fn unpack() {

    }
}
