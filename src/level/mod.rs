use crate::component::*;
use crate::entity::*;

pub trait TLevel {

    fn espawn(&mut self);
}

pub struct Level {
    //entity manager
    emanager: EManager,
    //component manager
    cmanager: CManager,
}

impl Level {
    pub fn new() -> Level {
        Level {
            emanager: EManager::new(),
            cmanager: CManager::new(),
        }
    }

    //Entity: spawns an entity with no components.
    pub fn espawn(&mut self) -> Entity {
        self.emanager.create()
    }

    //Entity-Component: gives an entity the supplied component.
    pub fn ecgive<T>(&mut self, entity: &Entity, component: T) {

    }

    //Entity-Component: returns a reference to an entity's component.
    pub fn ecget<T>(&mut self, entity: &Entity) -> &T {
        self.cmanager.cget::<T>()
    }
}
