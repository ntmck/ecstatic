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
    cownership: COwnership,
    //ratio of len/capacity space to trigger memory compression on the storage of a component.
    compression_ratio: f64,
}

impl Level {
    pub fn new(compression_ratio: f64) -> Level {
        Level {
            emanager: EManager::new(),
            cmanager: CManager::new(),
            cownership: COwnership::new(),
            compression_ratio: compression_ratio,
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
            self.check_compress::<T>()?;
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
        self.cmanager.cset::<T>(i, component)?;
        self.check_compress::<T>()?;
        Ok(())
    }

    //Entity-Component: removes component from given entity.
    pub fn ecremove<T: Any>(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        let i = self.get_cindex::<T>(entity)?;
        self.remove_cindex::<T>(entity)?;
        self.cmanager.cremove::<T>(i)?;
        self.check_compress::<T>()?;
        Ok(())
    }

    //Entity-Component: marks an entity's components as free for the memory manager and invalidates the entity.
    pub fn ecfree(&mut self, entity: Entity) -> Result<(), ErrEcs> {
        for (k, v) in self.cownership.get_entity_components_iter_mut(&entity) {
            self.cmanager.cremove_by_id(k, *v)?;
        }
        self.cownership.remove_entry(&entity)?;
        Ok(())
    }

    //Component: returns the length of cmanager.storage.
    pub fn clen<T: Any>(&self) -> usize {
        self.cmanager.len::<T>()
    }

    //Component: returns the capacity of cmanager.storage.
    pub fn ccapacity<T: Any>(&self) -> usize {
        self.cmanager.capacity::<T>()
    }

    pub fn is_entity_active(&self, e: &Entity) -> bool {
        self.cownership.is_entity_active(e)
    }

    //check to see if memory needs to be compressed.
    fn check_compress<T: Any>(&mut self) -> Result<(), ErrEcs> {
        if self.ccapacity::<T>() > 0 {
            if (self.clen::<T>() / self.ccapacity::<T>()) as f64 <= self.compression_ratio {
                self.compress_component_memory::<T>();
            }
            Ok(())
        } else { Err(ErrEcs::LevelStorageCapacityLessThanOrEqualToZero(format!("Avoided divison by 0 or negative in check_compress."))) }
    }

    //compress memory of a type.
    pub fn compress_component_memory<T: Any>(&mut self) -> Result<(), ErrEcs> {
        let mem = Memory::new();
        mem.compress::<T>(&mut self.cmanager, &mut self.cownership);
        Ok(())
    }

    //get the component index if it is owned by the entity.
    fn get_cindex<T: Any>(&mut self, e: &Entity) -> Result<usize, ErrEcs> {
        self.cownership.get_cindex::<T>(e)
    }

    //make a component index owned by an entity.
    fn set_cindex<T: Any>(&mut self, i: usize, e: &Entity) {
        self.cownership.insert::<T>(i, e)
    }

    //remove an entity's ownership of a component.
    fn remove_cindex<T: Any>(&mut self, e: &Entity) -> Result<(), ErrEcs> {
        self.cownership.remove::<T>(e)
    }
}
