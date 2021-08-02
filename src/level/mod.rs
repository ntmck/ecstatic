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
                Entity::insert_component::<T>(entity, i);
                Ok(())
            },
            Err(e) => Err(ErrEcs::LevelComponentInsert(format!("Level could not insert component type: {} into entity {} for reason: {:#?}\n", type_name::<T>(), entity.read().unwrap().id, e))),
        }
    }

    pub fn ecread<T: Any + Send + Sync + Copy>(&self, entity: &ALEntity) -> Result<T, ErrEcs> {
        match Entity::get_component_index::<T>(entity) {
            Ok(i) => {
                let component = Component::read::<T>(i, &self.components)?;
                Ok(component)
            },
            Err(e) => Err(ErrEcs::LevelComponentInsert(format!("Level could not get component type: {} from entity {} for reason: {:#?}\n", type_name::<T>(), entity.read().unwrap().id, e))),
        }
    }

    pub fn ecmodify<T: Any + Send + Sync>(&self, entity: &ALEntity, modify: Modify<T>) -> Result<(), ErrEcs> {
        match Entity::get_component_index::<T>(entity) {
            Ok(i) => {
                Component::modify::<T>(i, &self.components, modify)?;
                Ok(())
            },
            Err(e) => Err(ErrEcs::LevelComponentInsert(format!("Level could not get component type: {} from entity {} for reason: {:#?}\n", type_name::<T>(), entity.read().unwrap().id, e))),
        }
    }
}

/*#[test]
fn test_ecremove() {
    let level = Level::new();
    let entity = Entity::new();
    level.ecinsert::<u64>(&entity, 0u64);
    assert!(Entity::has_component::<u64>(&entity));
    level.ecremove::<u64>(&entity);
    assert!(!Entity::has_component::<u64>(&entity));
}*/

#[test]
fn test_ecget_modify() {
    let level = Level::new();
    let entity = Entity::new();
    level.ecinsert::<u64>(&entity, 0u64);

    let previous = level.ecread::<u64>(&entity).unwrap();
    level.ecmodify::<u64>(&entity, |component| {
        *component += 1;
    });
    let after = level.ecread::<u64>(&entity).unwrap();
    assert!(previous == 0 && after == 1, "previous: {}, after: {}", previous, after);

    let previous = level.ecread::<u64>(&entity).unwrap();
    level.ecmodify::<u64>(&entity, |component| {
        *component -= 1;
    });
    let after = level.ecread::<u64>(&entity).unwrap();
    assert!(previous == 1 && after == 0, "previous: {}, after: {}", previous, after);
}

#[test]
fn test_ecget_component_for_entity() {
    let level = Level::new();
    let entity = Entity::new();
    level.ecinsert::<u64>(&entity, 1u64);
    assert!(Entity::has_component::<u64>(&entity));
    let value = level.ecread::<u64>(&entity).unwrap();
    assert!(value == 1u64, "actual: {}", value);

    let entity2 = Entity::new();
    level.ecinsert::<u64>(&entity2, 2u64);
    assert!(Entity::has_component::<u64>(&entity2));
    let value = level.ecread::<u64>(&entity2).unwrap();
    assert!(value == 2u64, "actual: {}", value);

    level.ecinsert::<usize>(&entity, 23 as usize);
    assert!(Entity::has_component::<usize>(&entity));
    assert!(!Entity::has_component::<usize>(&entity2));
    let value = level.ecread::<usize>(&entity).unwrap();
    assert!(value == 23 as usize, "actual: {}", value);
}

#[test]
#[should_panic]
fn test_none_ecget() {
    let level = Level::new();
    let entity = Entity::new();
    let value = level.ecread::<u64>(&entity).unwrap();
}

#[test]
fn test_ecinsert_two_of_same_component_into_same_entity() {
    let level = Level::new();
    let entity = Entity::new();
    level.ecinsert::<u64>(&entity, 1u64);
    level.ecinsert::<u64>(&entity, 2u64);
    let value = level.ecread::<u64>(&entity).unwrap();
    assert!(value == 1u64, "actual: {}", value);
}
