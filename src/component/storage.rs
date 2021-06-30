use std::vec::Vec;
use std::any::{Any, TypeId};
use std::any::type_name;
use std::collections::HashMap;

use crate::ErrEcs;

//Component storage
pub struct Storage {
    stored: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl Storage {
        pub fn new() -> Storage {
            Storage { stored: HashMap::new() }
        }

        pub fn swap<T: Any>(&mut self, i: usize, j: usize) {
            self.swap_by_id(&TypeId::of::<T>(), i, j);
        }

        pub fn swap_by_id(&mut self, type_id: &TypeId, i: usize, j: usize) {
            if let Some(cmps) = self.stored.get_mut(type_id) {
                let tempi = cmps.remove(i);
                let tempj = cmps.remove(j);
                cmps[i] = tempj;
                cmps[j] = tempi;
            } else { panic!("unimplemented error handling") }
        }

        pub fn capacity<T: Any>(&self) -> usize {
            self.capacity_by_id(&TypeId::of::<T>())
        }

        pub fn capacity_by_id(&self, type_id: &TypeId) -> usize {
            if let Some(vec) = self.stored.get(type_id) {
                vec.capacity()
            } else { panic!("unimplemented error handling") }
        }

        pub fn len<T: Any>(&self) -> usize {
            self.len_by_id(&TypeId::of::<T>())
        }

        pub fn len_by_id(&self, type_id: &TypeId) -> usize {
            if let Some(vec) = self.stored.get(type_id) {
                vec.len()
            } else { panic!("unimplemented error handling") }
        }

        pub fn get<T: Any>(&self, i: usize) -> Result<&T, ErrEcs> {
            if let Some(vec) = self.stored.get(&TypeId::of::<T>()) {
                if let Some(cmp) = vec[i].downcast_ref::<T>() {
                    Ok(cmp)
                } else { Err(ErrEcs::StorageComponentNotFound(format!("cget type: {} index: {}", type_name::<T>(), i))) }
            } else { Err(ErrEcs::StorageComponentTypeNotFound(format!("cget type: {}", type_name::<T>()))) }
        }

        pub fn insert<T: Any>(&mut self, i: usize, comp: T) {
            loop {
                if let Some(vec) = self.stored.get_mut(&TypeId::of::<T>()) {
                    if i >= vec.capacity() {
                        vec.reserve((i - vec.capacity()) + 1);
                    }
                    vec.insert(i, Box::new(comp));
                    break;
                } else {
                    self.stored.insert(TypeId::of::<T>(), vec![]);
                }
            }
        }

        pub fn remove<T: Any>(&mut self, i: usize) -> Result<(), ErrEcs> {
            self.remove_by_id(&TypeId::of::<T>(), i)
        }

        pub fn remove_by_id(&mut self, type_id: &TypeId, i: usize) -> Result<(), ErrEcs> {
            if let Some(vec) = self.stored.get_mut(type_id) {
                vec[i] = Box::new(0u8);
                Ok(())
            } else { Err(ErrEcs::StorageComponentTypeNotFound(format!("cremove type_id: {:#?}", type_id))) }
        }

        pub fn set<T: Any>(&mut self, i: usize, comp: T) -> Result<(), ErrEcs> {
            if let Some(vec) = self.stored.get_mut(&TypeId::of::<T>()) {
                vec[i] = Box::new(comp);
                Ok(())
            } else { Err(ErrEcs::StorageComponentTypeNotFound(format!("cset type: {}", type_name::<T>()))) }
        }
}
