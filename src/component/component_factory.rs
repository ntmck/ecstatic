use std::collections::{HashMap, HashSet};

use super::{ComponentBank, PackedBank};
use crate::entity::Entity;
use crate::Component;

pub struct ComponentFactory {
    //entity.id->lookup table for that entity.
    component_lookup: HashMap<u64, HashMap<Component, u32>>, //Only hash Component as: Component::Position(None) for example. Only use None as param otherwise duplicates happen.
    bank: ComponentBank,
    packed_bank: PackedBank,
}

impl ComponentFactory {
    const POSITION_MASK: u32 = 0;
    const ROTATION_MASK: u32 = 1;
    const INIT_MASK: u32 = 2;
    const UPDATE_MASK: u32 = 4;
    const DESTROY_MASK: u32 = 8;

    pub fn new() -> ComponentFactory {
        ComponentFactory {
            component_lookup: HashMap::new(),
            bank: ComponentBank {
                positions: vec![],
                rotations: vec![],
                inits: vec![],
                updates: vec![],
                destroys: vec![],
            },
            packed_bank: PackedBank {
                positions: HashSet::new(),
                rotations: HashSet::new(),
                inits: HashSet::new(),
                updates: HashSet::new(),
                destroys: HashSet::new(),
            }
        }
    }

    //Allocates one space in each vector for a new entity.
    fn allocate(&mut self) {
        self.bank.positions.push(None);
        self.bank.rotations.push(None);
        self.bank.inits.push(None);
        self.bank.updates.push(None);
        self.bank.destroys.push(None);
    }

    //The inverse of Allocate.
    fn deallocate(&mut self) {
        self.bank.positions.pop();
        self.bank.rotations.pop();
        self.bank.inits.pop();
        self.bank.updates.pop();
        self.bank.destroys.pop();
    }

    //fn insert_position(&mut self, en: &mut Entity, data: Option<Vec3>) {}

    pub fn insert_component(&mut self, en: &mut Entity, cmp: Component) {
        panic!("Unimplemented.")
        //insert into bank
        //insert into packed bank
        //make lookup entry
        //update bitmask
    }

    pub fn update_component(&mut self, en: &Entity, with: Component) {
        panic!("Unimplemented.")
    }

    pub fn remove_component(&mut self, en: &mut Entity, which: Component) {
        panic!("Unimplemented.")
    }

    pub fn free_entity_components(&mut self, en: &mut Entity) {
        panic!("Unimplemented.")
    }
}
