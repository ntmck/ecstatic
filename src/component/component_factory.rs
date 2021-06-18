use std::collections::{HashMap, HashSet};
use std::ops::IndexMut;

use super::{ComponentBank, PackedBank, ComponentStorage};
use crate::entity::Entity;
use crate::{Component, ErrEcs};

pub struct ComponentFactory {
    //entity.id->lookup table for that entity.
    component_lookup: HashMap<u64, HashMap<u32, usize>>, //Use masks for key.
    component_storage: ComponentStorage,
}

impl ComponentFactory {
    const POSITION_MASK: u32 = 1;
    const ROTATION_MASK: u32 = 2;
    const INIT_MASK: u32 = 4;
    const UPDATE_MASK: u32 = 8;
    const DESTROY_MASK: u32 = 16;

    pub fn new() -> ComponentFactory {
        ComponentFactory {
            component_lookup: HashMap::new(),
            component_storage: ComponentStorage {
                bank: ComponentBank {
                    positions: vec![],
                    rotations: vec![],
                    inits: vec![],
                    updates: vec![],
                    destroys: vec![],
                },
                packed: PackedBank {
                    positions: HashSet::new(),
                    rotations: HashSet::new(),
                    inits: HashSet::new(),
                    updates: HashSet::new(),
                    destroys: HashSet::new(),
                }
            },
        }
    }

    //fn insert_position(&mut self, en: &mut Entity, data: Option<Vec3>) {}

    fn bank_set<T>(index: usize, data: T, bank_vec: &mut Vec<T>) {
        bank_vec.insert(index, data);
    }

    fn pack_set(packed_bank: &mut HashSet<u32>, index: usize) {
        packed_bank.insert(index);
    }

    fn alloc_if_needed<T>(desired_index: usize, bank_vec: &mut Vec<T>) {
        if desired_index > bank_vec.capacity() {
            bank_vec.reserve(1);
        }
    }

    fn insert<T>(index: usize, data: T, bank_vec: &mut Vec<T>, packed_bank: &mut HashSet<usize>) {
        ComponentFactory::alloc_if_needed::<T>(index, bank_vec);
        ComponentFactory::bank_set::<T>(index, data, bank_vec);
        ComponentFactory::pack_set(packed_bank, index);
    }

    //lookup in lookup table. if present, don't insert. if not present, make entry and insert.
    fn lookup_insert<T>(&mut self,
        en: &Entity,
        index: usize,
        mask: u32,
        data: T,
        bank_vec: &mut Vec<T>,
        packed_bank: &mut HashSet<usize>) -> Result<(), ErrEcs>{
        if let Some(entity_components_lookup) = component_lookup.get(en.id) { //has components already.
            //EXTRACT BELOW INTO FUNCTION
            if let Some(_) = entity_components_lookup.get(mask) { //uh oh, do nothing because it already has this component. return an error.
                Err(ErrEcs::ComponentFactoryEntityAlreadyHasComponent(format!("entity: {:#?} component: {:#?}", en, data)))
            } else { //okay give it a new component index located at the end.
                ComponentFactory::insert::<T>(index, data, bank_vec, packed_bank);
                entity_components_lookup.insert(mask, index);
            }
        } else { //New entity which does not have any components yet. So we insert some new ones.
            //USE EXTRACTED FUNCTION HERE?
        }
    }

    pub fn insert_component(&mut self, en: &mut Entity, cmp: Component) -> Result<(), ErrEcs>{
        panic!("Unimplemented.");
        //check to see if allocation required.
        //insert into bank
        //insert into packed bank
        //make lookup entry
        //update bitmask

        match cmp {
            Component::Position(op) => {
                //ComponentFactory::insert::<Option<Vec3>>()
            },
            Component::Rotation(op) => {

            },
            Component::Init(op) => {

            },
            Component::Update(op) => {

            },
            Component::Destroy(op) => {

            },
        }
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
