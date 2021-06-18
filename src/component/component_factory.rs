use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::IndexMut;

use super::{ComponentBank, PackedBank, ComponentStorage, FreeIndices};
use crate::entity::Entity;
use crate::{Component, ErrEcs};
use crate::data::*;

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
                },
                free: FreeIndices {
                    pos_next: 0,
                    rot_next: 0,
                    init_next: 0,
                    update_next: 0,
                    destroy_next: 0,
                    positions: VecDeque::new(),
                    rotations: VecDeque::new(),
                    inits: VecDeque::new(),
                    updates: VecDeque::new(),
                    destroys: VecDeque::new(),
                }
            },
        }
    }

    //inserts a component and returns the index it inserted at.
    fn insert<T>(
        data: T,
        into: &mut Vec<T>,
        packed: &mut HashSet<usize>,
        free_indices: &mut VecDeque<usize>,
        next: &mut usize
    ) -> usize
    {
        if let Some(i) = free_indices.pop_front() {
            into[i] = data;
            packed.insert(i);
            i
        } else {
            if *next >= into.len() {
                into.reserve(*next - into.len() + 1);
            }
            into.insert(*next, data);
            packed.insert(*next);
            *next += 1;
            *next
        }
    }

    //lookup in lookup table. if present, don't insert. if not present, make entry and insert.
    fn lookup_insert<T>(
        en: &Entity,
        mask: u32,
        data: T,
        component_lookup: &mut HashMap<u64, HashMap<u32, usize>>,
        into: &mut Vec<T>,
        packed: &mut HashSet<usize>,
        free_indices: &mut VecDeque<usize>,
        next: &mut usize
    ) -> Result<(), ErrEcs>
    {
        if let Some(entity_components_lookup) = component_lookup.get_mut(&en.id) {
            if let Some(_) = entity_components_lookup.get(&mask) {
                Err(ErrEcs::ComponentFactoryEntityAlreadyHasComponent(format!("entity.id: {} component_mask: {}", en.id, mask)))
            } else {
                let i = ComponentFactory::insert::<T>(data, into, packed, free_indices, next);
                entity_components_lookup.insert(mask, i);
                Ok(())
            }
        } else {
            component_lookup.insert(en.id, HashMap::new());
            let i = ComponentFactory::insert::<T>(data, into, packed, free_indices, next);
            component_lookup.get_mut(&en.id).unwrap().insert(mask, i);
            Ok(())
        }
    }

    pub fn insert_component(&mut self, en: &mut Entity, cmp: Component) -> Result<(), ErrEcs> {
        match cmp {
            Component::Position(op) => {
                ComponentFactory::lookup_insert::<Option<Vec3>>(
                    en,
                    ComponentFactory::POSITION_MASK,
                    op,
                    &mut self.component_lookup,
                    &mut self.component_storage.bank.positions,
                    &mut self.component_storage.packed.positions,
                    &mut self.component_storage.free.positions,
                    &mut self.component_storage.free.pos_next
                )?;
                en.component_mask |= ComponentFactory::POSITION_MASK;
                Ok(())
            },
            Component::Rotation(op) => {
                ComponentFactory::lookup_insert::<Option<Vec3>>(
                    en,
                    ComponentFactory::ROTATION_MASK,
                    op,
                    &mut self.component_lookup,
                    &mut self.component_storage.bank.rotations,
                    &mut self.component_storage.packed.rotations,
                    &mut self.component_storage.free.rotations,
                    &mut self.component_storage.free.rot_next
                )?;
                en.component_mask |= ComponentFactory::ROTATION_MASK;
                Ok(())
            },
            Component::Init(op) => {
                ComponentFactory::lookup_insert::<Option<Init>>(
                    en,
                    ComponentFactory::INIT_MASK,
                    op,
                    &mut self.component_lookup,
                    &mut self.component_storage.bank.inits,
                    &mut self.component_storage.packed.inits,
                    &mut self.component_storage.free.inits,
                    &mut self.component_storage.free.init_next
                )?;
                en.component_mask |= ComponentFactory::INIT_MASK;
                Ok(())
            },
            Component::Update(op) => {
                ComponentFactory::lookup_insert::<Option<Update>>(
                    en,
                    ComponentFactory::UPDATE_MASK,
                    op,
                    &mut self.component_lookup,
                    &mut self.component_storage.bank.updates,
                    &mut &mut self.component_storage.packed.updates,
                    &mut self.component_storage.free.updates,
                    &mut self.component_storage.free.update_next
                )?;
                en.component_mask |= ComponentFactory::UPDATE_MASK;
                Ok(())
            },
            Component::Destroy(op) => {
                ComponentFactory::lookup_insert::<Option<Destroy>>(
                    en,
                    ComponentFactory::DESTROY_MASK,
                    op,
                    &mut self.component_lookup,
                    &mut self.component_storage.bank.destroys,
                    &mut self.component_storage.packed.destroys,
                    &mut self.component_storage.free.destroys,
                    &mut self.component_storage.free.destroy_next
                )?;
                en.component_mask |= ComponentFactory::DESTROY_MASK;
                Ok(())
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
