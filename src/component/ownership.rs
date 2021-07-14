use std::collections::HashMap;
use std::any::{Any, TypeId};

use crate::entity::Entity;
use crate::ErrEcs;

pub struct COwnership {
    //entity.id -> their components as typeid->index
    ownership: HashMap<u64, HashMap<TypeId, usize>>
}

impl COwnership {
    pub fn new() -> COwnership {
        COwnership {
            ownership: HashMap::new(),
        }
    }

    pub fn is_entity_active(&self, e: &Entity) -> bool {
        self.ownership.contains_key(&e.id)
    }

    //returns a mapping of index->entity_id for a specific type.
    pub fn get_index_entity_map<T: Any>(&self) -> HashMap<usize, u64> {
        let mut map = HashMap::new();
        for (eid, type_map) in self.ownership.iter() {
            if let Some(i) = type_map.get(&TypeId::of::<T>()) {
                map.insert(*i, *eid);
            }
        }
        map
    }

    pub fn insert_new(&mut self, entity: &Entity) {
        self.ownership.insert(entity.id, HashMap::new());
    }

    pub fn insert<T: Any>(&mut self, i: usize, e: &Entity) {
        self.insert_by_id(i, e.id, TypeId::of::<T>());
    }

    pub fn insert_by_id(&mut self, i: usize, eid: u64, type_id: TypeId) {
        loop {
            if let Some(cmap) = self.ownership.get_mut(&eid) {
                if let Some(i) = cmap.insert(type_id, i) {
                    panic!("ownership.insert() replaced an owned index. ecgive should have covered this case. index: {}", i);
                }
                break;
            } else {
                self.ownership.insert(eid, HashMap::new());
            }
        }
    }

    pub fn update_index<T: Any>(&mut self, e: &Entity, new_index: usize) -> Result<(), ErrEcs> {
        self.update_index_by_id(e.id, &TypeId::of::<T>(), new_index)
    }

    pub fn update_index_by_id(&mut self, eid: u64, type_id: &TypeId, new_index: usize) -> Result<(), ErrEcs> {
        if let Some(cmap) = self.ownership.get_mut(&eid) {
            if let Some(i) = cmap.get_mut(&type_id) {
                *i = new_index;
                Ok(())
            } else { Err(ErrEcs::COwnershipComponentNotFound(format!("get_cindex entity does not have component. type_id: {:#?}", type_id))) }
        } else { Err(ErrEcs::COwnershipEntityNotFound(format!("get_cindex entity not found in ownership. entity: {}", eid))) }
    }

    pub fn get_cindex<T: Any>(&self, e: &Entity) -> Result<usize, ErrEcs> {
        self.get_cindex_by_id(e.id, &TypeId::of::<T>())
    }

    pub fn get_cindex_by_id(&self, eid: u64, type_id: &TypeId) -> Result<usize, ErrEcs> {
        if let Some(cmap) = self.ownership.get(&eid) {
            if let Some(i) = cmap.get(type_id) {
                Ok(*i)
            } else { Err(ErrEcs::COwnershipComponentNotFound(format!("get_cindex entity does not have component. type_id: {:#?}", type_id))) }
        } else { Err(ErrEcs::COwnershipEntityNotFound(format!("get_cindex entity not found in ownership. entity: {}", eid))) }
    }

    pub fn remove<T: Any>(&mut self, e: &Entity) -> Result<(), ErrEcs> {
        self.remove_by_id(e.id, &TypeId::of::<T>())
    }

    pub fn remove_by_id(&mut self, eid: u64, type_id: &TypeId) -> Result<(), ErrEcs> {
        if let Some(cmap) = self.ownership.get_mut(&eid) {
            cmap.remove(&type_id);
            Ok(())
        } else { Err(ErrEcs::COwnershipEntityNotFound(format!("ownership.remove() entity not found in ownership. entity: {}", eid))) }
    }

    pub fn remove_entry(&mut self, e: &Entity) -> Result<(), ErrEcs> {
        match self.ownership.remove(&e.id) {
            Some(_) => Ok(()),
            None => Err(ErrEcs::COwnershipEntityNotFound(format!("ownership.remove_entry() entity not found in ownership. entity: {}", e.id)))
        }
    }
}
