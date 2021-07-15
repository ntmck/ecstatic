use std::any::{Any, TypeId};
use std::any::type_name;
use std::collections::{HashMap, BTreeSet};
use std::marker::{Send, Sync};

use crate::ErrEcs;
use super::storage::Storage;
use super::packer::Packer;

pub struct FreeEntry {
    pub next_free: usize,
    pub free_q: BTreeSet<usize>,
}

pub struct Components {
    //component_mask(type of component)->components
    storage: Storage,
    //component_mask(type of component)->active_component_indices
    packer: Packer,
    //component_mask(type of component)->(next_free_index, freed_indices_for_type)
    free: HashMap<TypeId, FreeEntry>,
}

pub struct CManager {
    components: Components,
}

impl CManager {
    pub fn new() -> CManager {
        CManager {
            components: Components {
                storage: Storage::new(),
                packer: Packer::new(),
                free: HashMap::new(),
            }
        }
    }

    pub fn cresize<T>(&mut self, new_size: usize) -> Result<(), ErrEcs>
    where T: Any + Send + Sync {
        self.components.storage.resize::<T>(new_size)
    }

    pub fn cget<T>(&self, i: usize) -> Result<&T, ErrEcs>
    where T: Any + Send + Sync {
        self.components.storage.get::<T>(i)
    }

    pub fn cset<T>(&mut self, i: usize, comp: T) -> Result<(), ErrEcs>
    where T: Any + Send + Sync {
        self.components.storage.set::<T>(i, comp)
    }

    pub fn cinsert<T>(&mut self, comp: T) -> usize
    where T: Any + Send + Sync {
        let i = self.find_available_index::<T>();
        self.components.storage.insert::<T>(i, comp);
        self.components.packer.pack::<T>(i);
        i
    }

    //This function is unsafe because it does NOT update
    // an entity's owned component index which may result in index out of bounds or other errors
    // such as an entity accessing another entity's component. Use with caution.
    //swap element at i with a free index j. c[j] = c[i].
    pub fn unsafe_swap_with_free<T>(&mut self, i: usize) -> usize
    where T: Any + Send + Sync {
        let j = self.find_available_index::<T>();
        self.components.storage.swap::<T>(i, j);
        if let Some(entry) = self.components.free.get_mut(&TypeId::of::<T>()) {
            entry.free_q.insert(i);
        } else { panic!("Failed to find type in unsafe function.") }
        j
    }

    //Resets the FreeEntry of a type to correctly reflect the capacity of the vector and empties the queue.
    //returns next free.
    pub fn reset_free<T>(&mut self, to: usize) -> Result<(), ErrEcs>
    where T: Any + Send + Sync {
        let new_next_free = to + 1;
        if let Some(entry) = self.components.free.get_mut(&TypeId::of::<T>()) {
            entry.next_free = new_next_free;
            entry.free_q.clear();
            Ok(())
        } else { Err(ErrEcs::CManagerTypeNotFound(format!("reset_free type_name: {}", type_name::<T>()))) }
    }

    //Returns capacity of storage for the given type.
    pub fn capacity<T>(&self) -> usize
    where T: Any + Send + Sync {
        self.components.storage.capacity::<T>()
    }

    //Returns len of storage for the given type.
    pub fn len<T>(&self) -> usize
    where T: Any + Send + Sync {
        self.components.storage.len::<T>()
    }

    pub fn plen<T>(&self) -> usize
    where T: Any + Send + Sync {
        self.components.packer.len::<T>()
    }

    pub fn pcapacity<T>(&self) -> usize
    where T: Any + Send + Sync {
        self.components.packer.capacity::<T>()
    }

    pub fn cremove<T>(&mut self, i: usize) -> Result<(), ErrEcs>
    where T: Any + Send + Sync {
        self.cremove_by_id(&TypeId::of::<T>(), i)
    }

    //frees an index for reuse and sets the memory of a type at index to 0.
    pub fn cremove_by_id(&mut self, type_id: &TypeId, i: usize) -> Result<(), ErrEcs> {
        self.free_index_by_id(type_id, i);
        self.components.packer.unpack_by_id(type_id, i)?;
        self.components.storage.remove_by_id(type_id, i)
    }

    pub fn pack<T>(&mut self, i: usize)
    where T: Any + Send + Sync {
        self.components.packer.pack::<T>(i);
    }

    pub fn unpack<T>(&mut self, i: usize) -> Result<(), ErrEcs>
    where T: Any + Send + Sync {
        self.components.packer.unpack::<T>(i)
    }

    fn free_index_by_id(&mut self, type_id: &TypeId, i: usize) {
        loop {
            if let Some(entry) = self.components.free.get_mut(type_id) {
                entry.free_q.insert(i);
                break;
            } else {
                self.components.free.insert(*type_id, FreeEntry{next_free: 0, free_q: BTreeSet::new()});
            }
        }
    }

    //Returns an available index for insertion.
    pub fn find_available_index<T>(&mut self) -> usize
    where T: Any + Send + Sync {
        self.find_available_free_index_by_id(&TypeId::of::<T>())
    }

    pub fn find_available_free_index_by_id(&mut self, type_id: &TypeId) -> usize {
        let i;
        loop {
            if let Some(entry) = self.components.free.get_mut(type_id) {
                if let Some(next) = entry.free_q.first() {
                    i = next.clone();
                } else {
                    i = entry.next_free;
                    entry.next_free += 1;
                }
                entry.free_q.remove(&i);
                break;
            } else { //may cause issues in memory compression later in development...
                self.components.free.insert(*type_id, FreeEntry{next_free: 0, free_q: BTreeSet::new()});
            }
        }
        i
    }
}
