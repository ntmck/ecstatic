use std::collections::HashSet;
use std::any::{Any, TypeId};
use std::any::type_name;

use crate::component::*;
use crate::entity::*;
//use crate::system::*;
use crate::ErrEcs;

pub struct Level {
    //entity manager
    emanager: EManager,
    //component manager
    cmanager: CManager,
    //component ownership: entity_id->component_type->index
    cownership: COwnership,
    //all active types.
    type_registry: HashSet<TypeId>,
    //ratio of len/capacity space to trigger memory compression on the storage of a component.
    compression_ratio: f64,
}

impl Level {
    const CAPACITY_LIMIT_TO_BEGIN_CHECKING_MEMORY: usize = 10000;

    pub fn new(compression_ratio: f64) -> Level {
        Level {
            emanager: EManager::new(),
            cmanager: CManager::new(),
            cownership: COwnership::new(),
            type_registry: HashSet::new(),
            compression_ratio: compression_ratio,
        }
    }

    //Entity: spawns an entity with no components. TODO: Perhaps later, spawn with default components.
    pub fn espawn(&mut self) -> Entity {
        let e = self.emanager.create();
        self.cownership.insert_new(&e);
        e
    }

    //Entity-Component: gives an entity the supplied component. Does nothing if it already exists.
    pub fn ecgive<T: Any>(&mut self, entity: &Entity, component: T) -> Result<(), ErrEcs> {
        if let Ok(_) = self.cownership.get_cindex::<T>(entity) {
            return Err(ErrEcs::CManagerComponentAlreadyExistsForEntity(
                format!("ecgive entity: {} type: {}", entity.id, type_name::<T>())
            ))
        } else {
            let i = self.cmanager.cinsert::<T>(component);
            self.cownership.insert::<T>(i, entity);
            self.type_registry.insert(TypeId::of::<T>());
            self.check_compress::<T>()?;
            Ok(())
        }
    }

    //Entity-Component: returns a reference to an entity's component.
    pub fn ecget<T: Any>(&mut self, entity: &Entity) -> Result<&T, ErrEcs> {
        let i = self.cownership.get_cindex::<T>(entity)?;
        self.cmanager.cget::<T>(i)
    }

    //Entity-Component: updates an entity's component.
    pub fn ecset<T: Any>(&mut self, entity: &Entity, component: T) -> Result<(), ErrEcs> {
        let i = self.cownership.get_cindex::<T>(entity)?;
        self.cmanager.cset::<T>(i, component)?;
        self.check_compress::<T>()?;
        Ok(())
    }

    //Entity-Component: removes component from given entity.
    pub fn ecremove<T: Any>(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        let result = self.ecremove_by_id(entity.id, &TypeId::of::<T>());
        self.check_compress::<T>()?;
        result
    }

    fn ecremove_by_id(&mut self, eid: u64, type_id: &TypeId) -> Result<(), ErrEcs> {
        let i = self.cownership.get_cindex_by_id(eid, type_id)?;
        self.cownership.remove_by_id(eid, type_id)?;
        self.cmanager.cremove_by_id(type_id, i)?;
        Ok(())
    }

    //Entity-Component: marks an entity's components as free for the memory manager and invalidates the entity.
    pub fn ecfree(&mut self, entity: Entity) -> Result<(), ErrEcs> {
        let registry_types: Vec<TypeId> = self.type_registry.iter().map(|&x|{x.clone()}).collect();
        for type_id in registry_types {
            self.ecremove_by_id(entity.id, &type_id)?;
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

    //Component: returns the length of the packer.
    pub fn plen<T: Any>(&self) -> usize {
        self.cmanager.plen::<T>()
    }

    //Component: returns the capacity of the packer.
    pub fn pcapacity<T: Any>(&self) -> usize {
        self.cmanager.pcapacity::<T>()
    }

    pub fn is_entity_active(&self, e: &Entity) -> bool {
        self.cownership.is_entity_active(e)
    }

    //check to see if memory needs to be compressed.
    fn check_compress<T: Any>(&mut self) -> Result<(), ErrEcs> {
        if self.ccapacity::<T>() >= Level::CAPACITY_LIMIT_TO_BEGIN_CHECKING_MEMORY {
            if (self.clen::<T>() / self.ccapacity::<T>()) as f64 <= self.compression_ratio {
                self.compress_component_memory::<T>()?;
            }
            Ok(())
        } else { Err(ErrEcs::LevelStorageCapacityLessThanOrEqualToZero(format!("Avoided divison by 0 or negative in check_compress."))) }
    }

    //compress memory of a type.
    pub fn compress_component_memory<T: Any>(&mut self) -> Result<(), ErrEcs> {
        Memory::compress::<T>(&mut self.cmanager, &mut self.cownership)?;
        Ok(())
    }
}
