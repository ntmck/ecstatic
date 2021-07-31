use std::any::{Any, TypeId};

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
        Entity::insert_component(entity, TypeId::of::<T>());
        Component::insert::<T>(component, &self.components, &self.indices)?;
        Ok(())
    }

    pub fn ecget<T: Any + Send + Sync>(&self, entity: &ALEntity) -> Result<T, ErrEcs> {
    }
}

#[test]
fn test_new() {
    let level = Level::new();
}

#[test]
fn test_get_component_for_entity() {
    let level = Level::new();
    let mut entity = Entity::new();
    level.ecinsert::<u64>(&mut entity, 1u64);
    assert!(Entity::has_component(&entity, &TypeId::of::<u64>()));
    let value = level.ecget::<u64>(&entity).unwrap();
    assert!(value == 1u64);
}
