use crate::component::*;
use crate::entity::*;
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::any::type_name;
use crate::ErrEcs;

pub struct Level {
    //entity manager
    emanager: EManager,
    //component manager
    cmanager: CManager,
    //component ownership: entity_id->component_type->index
    cownership: HashMap<u64, HashMap<TypeId, usize>>
}

impl Level {
    pub fn new() -> Level {
        Level {
            emanager: EManager::new(),
            cmanager: CManager::new(),
            cownership: HashMap::new(),
        }
    }

    //Entity: spawns an entity with no components. TODO: Perhaps later, spawn with default components.
    pub fn espawn(&mut self) -> Entity {
        self.emanager.create()
    }

    //Entity-Component: gives an entity the supplied component. Does nothing if it already exists.
    pub fn ecgive<T: Any>(&mut self, entity: &Entity, component: T) -> Result<(), ErrEcs> {
        if let Ok(_) = self.get_cindex::<T>(entity) {
            return Err(ErrEcs::CManagerComponentAlreadyExistsForEntity(
                format!("ecgive entity: {} type: {}", entity.id, type_name::<T>())
            ))
        } else {
            let i = self.cmanager.cinsert::<T>(component);
            self.set_cindex::<T>(i, entity);
            Ok(())
        }
    }

    //Entity-Component: returns a reference to an entity's component.
    pub fn ecget<T: Any>(&mut self, entity: &Entity) -> Result<&T, ErrEcs> {
        let i = self.get_cindex::<T>(entity)?;
        self.cmanager.cget::<T>(i)
    }

    pub fn ecset<T: Any>(&mut self, entity: &Entity, component: T) -> Result<(), ErrEcs> {
        let i = self.get_cindex::<T>(entity)?;
        self.cmanager.cset::<T>(i, component)
    }

    //Entity-Component: removes component from given entity.
    pub fn ecremove<T: Any>(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        let i = self.get_cindex::<T>(entity)?;
        self.remove_cindex::<T>(entity)?;
        self.cmanager.cremove::<T>(i)
    }

    //Entity-Component: marks an entity's components as free for the memory manager and invalidates the entity.
    pub fn ecfree(&mut self, entity: Entity) -> Result<(), ErrEcs> {
        self.emanager.deactivate_entity(&entity)?;
        if let Some(cmap) = self.cownership.get_mut(&entity.id) {
            for (k, v) in cmap.iter_mut() {
                self.cmanager.cremove_by_id(*k, *v)?;
            }
        }
        self.cownership.remove(&entity.id);
        Ok(())
    }

    pub fn is_entity_active(&self, entity: &Entity) -> bool {
        self.emanager.is_entity_active(entity)
    }

    //get the component index if it is owned by the entity.
    fn get_cindex<T: Any>(&mut self, entity: &Entity) -> Result<usize, ErrEcs> {
        if let Some(cmap) = self.cownership.get(&entity.id) {
            if let Some(i) = cmap.get(&TypeId::of::<T>()) {
                Ok(*i)
            } else { Err(ErrEcs::CManagerComponentNotFound(format!("get_cindex entity does not have component {}", type_name::<T>()))) }
        } else { Err(ErrEcs::CManagerEntityNotFound(format!("get_cindex entity not found in cownership. entity: {}", entity.id))) }
    }

    //make a component index owned by an entity.
    fn set_cindex<T: Any>(&mut self, i: usize, entity: &Entity) {
        loop {
            if let Some(cmap) = self.cownership.get_mut(&entity.id) {
                if let Some(i) = cmap.insert(TypeId::of::<T>(), i) {
                    panic!("set_cindex replaced an owned index. ecgive should have covered this case. index: {}", i);
                }
                break;
            } else {
                self.cownership.insert(entity.id, HashMap::new());
            }
        }
    }

    //remove an entity's ownership of a component.
    fn remove_cindex<T: Any>(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        if let Some(cmap) = self.cownership.get_mut(&entity.id) {
            cmap.remove(&TypeId::of::<T>());
            Ok(())
        } else { Err(ErrEcs::CManagerEntityNotFound(format!("remove_cindex entity not found in cownership. entity: {}", entity.id))) }
    }
}
