use crate::component::*;
use crate::entity::*;
use std::collections::{HashSet, HashMap};
use std::any::{Any, TypeId};
use std::any::type_name;
use crate::ErrEcs;

pub trait TLevel {

    fn espawn(&mut self);
}

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
            self.set_cindex::<T>(entity, i);
            Ok(())
        }
    }

    //Entity-Component: returns a reference to an entity's component.
    pub fn ecget<T: Any>(&mut self, entity: &Entity) -> Result<&T, ErrEcs> {
        let i = self.get_cindex::<T>(entity)?;
        self.cmanager.cget::<T>(i)
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
    fn set_cindex<T: Any>(&mut self, entity: &Entity, i: usize) {
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
}
