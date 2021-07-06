use std::vec::Vec;
use std::any::{Any, TypeId};
use std::any::type_name;
use std::collections::HashMap;
use core::slice::{Iter, IterMut};

use crate::ErrEcs;

pub struct Packer {
    packed: HashMap<TypeId, Vec<usize>>
}

impl Packer {
    pub fn new() -> Packer {
        Packer { packed: HashMap::new() }
    }

    pub fn iter<T: Any>(&self) -> Iter<usize> {
        if let Some(vec) = self.packed.get(&TypeId::of::<T>()) {
            vec.iter()
        } else { panic!("Packer failed to get Iter.") }
    }

    pub fn iter_mut<T: Any>(&mut self) -> IterMut<usize> {
        if let Some(vec) = self.packed.get_mut(&TypeId::of::<T>()) {
            vec.iter_mut()
        } else { panic!("Packer failed to get IterMut.") }
    }

    pub fn capacity<T: Any>(&self) -> usize {
        self.packed.capacity()
    }

    pub fn len<T: Any>(&self) -> usize {
        self.packed.len()
    }

    pub fn pack<T: Any>(&mut self, i: usize) {
        self.pack_by_id(&TypeId::of::<T>(), i);
    }

    pub fn pack_by_id(&mut self, type_id: &TypeId, i: usize) {
        loop {
            if let Some(vec) = self.packed.get_mut(type_id) {
                vec.push(i);
                break;
            } else {
                self.packed.insert(*type_id, vec![]);
            }
        }
    }

    pub fn unpack<T: Any>(&mut self, i: usize) -> Result<(), ErrEcs> {
        self.unpack_by_id(&TypeId::of::<T>(), i)
    }

    pub fn unpack_by_id(&mut self, type_id: &TypeId, i: usize) -> Result<(), ErrEcs> {
        if let Some(vec) = self.packed.get_mut(type_id) {
            if i < vec.len() {
                vec.remove(i);
                return Ok(())
            }
        }
        Err(ErrEcs::PackerUnpackIndexOutOfBounds(format!("attempt to unpack non-existent element from packed. index: {}", i)))
    }
}
