use std::vec::Vec;
use std::any::{Any, TypeId};
use std::any::type_name;
use std::collections::HashMap;
use std::marker::{Send, Sync};

use crate::ErrEcs;

//Storage empty type.
enum S {
    Empty,
}

//Component storage
pub struct Storage {
    stored: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
}

impl Storage {
        pub fn new() -> Storage {
            Storage { stored: HashMap::new() }
        }

        pub fn swap<T>(&mut self, i: usize, j: usize)
        where T: Any + Send + Sync {
            self.swap_by_id(&TypeId::of::<T>(), i, j);
        }

        pub fn swap_by_id(&mut self, type_id: &TypeId, i: usize, j: usize) {
            if let Some(cmps) = self.stored.get_mut(type_id) {
                cmps.swap(i, j);
            } else { panic!("unimplemented error handling") }
        }

        pub fn capacity<T>(&self) -> usize
        where T: Any + Send + Sync {
            self.capacity_by_id(&TypeId::of::<T>())
        }

        pub fn capacity_by_id(&self, type_id: &TypeId) -> usize {
            if let Some(vec) = self.stored.get(type_id) {
                vec.capacity()
            } else { panic!("unimplemented error handling") }
        }

        pub fn len<T>(&self) -> usize
        where T: Any + Send + Sync {
            self.len_by_id(&TypeId::of::<T>())
        }

        pub fn len_by_id(&self, type_id: &TypeId) -> usize {
            if let Some(vec) = self.stored.get(type_id) {
                vec.len()
            } else { panic!("unimplemented error handling") }
        }

        pub fn get<T>(&self, i: usize) -> Result<&T, ErrEcs>
        where T: Any + Send + Sync {
            if let Some(vec) = self.stored.get(&TypeId::of::<T>()) {
                if let Some(cmp) = vec[i].downcast_ref::<T>() {
                    Ok(cmp)
                } else { Err(ErrEcs::StorageComponentNotFound(format!("get type: {} index: {}", type_name::<T>(), i))) }
            } else { Err(ErrEcs::StorageComponentTypeNotFound(format!("get type: {}", type_name::<T>()))) }
        }

        pub fn insert<T>(&mut self, i: usize, comp: T)
        where T: Any + Send + Sync {
            loop {
                if let Some(vec) = self.stored.get_mut(&TypeId::of::<T>()) {
                    if i >= vec.capacity() {
                        vec.reserve((i - vec.capacity()) + 1);
                    }
                    vec.insert(i, Box::new(comp));
                    break;
                } else {
                    self.stored.insert(TypeId::of::<T>(), Vec::with_capacity(1));
                }
            }
        }

        pub fn remove<T>(&mut self, i: usize) -> Result<(), ErrEcs>
        where T: Any + Send + Sync {
            self.remove_by_id(&TypeId::of::<T>(), i)
        }

        pub fn remove_by_id(&mut self, type_id: &TypeId, i: usize) -> Result<(), ErrEcs> {
            if let Some(vec) = self.stored.get_mut(type_id) {
                vec[i] = Box::new(S::Empty);
                Ok(())
            } else { Err(ErrEcs::StorageComponentTypeNotFound(format!("remove_by_id type_id: {:#?}", type_id))) }
        }

        pub fn set<T>(&mut self, i: usize, comp: T) -> Result<(), ErrEcs>
        where T: Any + Send + Sync {
            if let Some(vec) = self.stored.get_mut(&TypeId::of::<T>()) {
                vec[i] = Box::new(comp);
                Ok(())
            } else { Err(ErrEcs::StorageComponentTypeNotFound(format!("set type: {}", type_name::<T>()))) }
        }

        pub fn resize<T>(&mut self, new_size: usize) -> Result<(), ErrEcs>
        where T: Any + Send + Sync {
            if let Some(vec) = self.stored.get_mut(&TypeId::of::<T>()) {
                vec.resize_with(new_size, || {Box::new(S::Empty)});
                vec.shrink_to_fit();
                Ok(())
            } else { Err(ErrEcs::StorageComponentTypeNotFound(format!("resize type_id: {:#?}", TypeId::of::<T>()))) }
        }
}
