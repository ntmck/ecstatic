use std::any::{Any, TypeId, type_name};

use crate::component::*;
use crate::entity::*;
use crate::ErrEcs;

struct Level {
    components: ALComponentStorage,
    indices: ALIndices,
}

impl Level {
    pub fn new() -> Level {
        let (c, i) = Component::new();
        Level {
            components: c,
            indices: i,
        }
    }

    pub fn ecinsert<T: Any + Send + Sync>(&self, entity: &ALEntity, component: T) -> Result<(), ErrEcs> {
        match Component::insert::<T>(component, &self.components, &self.indices) {
            Ok(i) => {
                Entity::insert_component(entity, TypeId::of::<T>(), i);
                Ok(())
            },
            Err(e) => Err(ErrEcs::LevelComponentInsert(format!("Level could not insert component type: {} into entity {} for reason: {:#?}\n", type_name::<T>(), entity.read().unwrap().id, e))),
        }
    }

    pub fn ecget<T: Any + Send + Sync + Copy>(&self, entity: &ALEntity) -> Result<T, ErrEcs> {
        match Entity::get_component_index(entity, &TypeId::of::<T>()) {
            Ok(i) => {
                let component = Component::get::<T>(i, &self.components)?;
                Ok(component)
            },
            Err(e) => Err(ErrEcs::LevelComponentInsert(format!("Level could not get component type: {} from entity {} for reason: {:#?}\n", type_name::<T>(), entity.read().unwrap().id, e))),
        }
    }
}

#[test]
fn test_new() {
    let level = Level::new();
}

#[test]
fn test_ecget_component_for_entity() {
    let level = Level::new();
    let entity = Entity::new();
    level.ecinsert::<u64>(&entity, 1u64);
    assert!(Entity::has_component(&entity, &TypeId::of::<u64>()));
    let value = level.ecget::<u64>(&entity).unwrap();
    assert!(value == 1u64, "actual: {}", value);

    let entity2 = Entity::new();
    level.ecinsert::<u64>(&entity2, 2u64);
    assert!(Entity::has_component(&entity2, &TypeId::of::<u64>()));
    let value = level.ecget::<u64>(&entity2).unwrap();
    assert!(value == 2u64, "actual: {}", value);

    level.ecinsert::<usize>(&entity, 23 as usize);
    assert!(Entity::has_component(&entity, &TypeId::of::<usize>()));
    assert!(!Entity::has_component(&entity2, &TypeId::of::<usize>()));
    let value = level.ecget::<usize>(&entity).unwrap();
    assert!(value == 23 as usize, "actual: {}", value);
}

#[test]
#[should_panic]
fn test_none_ecget() {
    let level = Level::new();
    let entity = Entity::new();
    let value = level.ecget::<u64>(&entity).unwrap();
}

#[test]
fn test_ecinsert_two_of_same_component_into_same_entity() {
    let level = Level::new();
    let mut entity = Entity::new();
    level.ecinsert::<u64>(&entity, 1u64);
    level.ecinsert::<u64>(&entity, 2u64);
    let value = level.ecget::<u64>(&entity).unwrap();
    assert!(value == 1u64, "actual: {}", value);
}
