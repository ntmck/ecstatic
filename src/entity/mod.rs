use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::sync::{Arc, RwLock};

use crate::ErrEcs;

//Arc-RwLock Entity
pub type ALEntity = Arc<RwLock<Entity>>;

pub struct Entity {
    pub id: u64,
    pub component_indices: RwLock<HashMap<TypeId, usize>>,
}

impl Entity {
    pub fn new() -> ALEntity {
        let e = Entity {
            id: Entity::make_token(),
            component_indices: RwLock::new(HashMap::new()),
        };
        Arc::new(RwLock::new(e))
    }

    pub fn insert_component<T: Any + Send + Sync>(entity: &ALEntity, i: usize) {
        if Entity::has_component::<T>(entity) { return }
        entity
            .read()
            .unwrap()
            .component_indices
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), i);
    }

    pub fn remove_component<T: Any + Send + Sync>(entity: &ALEntity) {
        entity
            .read()
            .unwrap()
            .component_indices
            .write()
            .unwrap()
            .remove(&TypeId::of::<T>());
    }

    pub fn get_component_index<T: Any + Send + Sync>(entity: &ALEntity) -> Result<usize, ErrEcs> {
        if !Entity::has_component::<T>(entity) { return Err(ErrEcs::EntityComponentNone(format!("entity {} does not have component id: {:?}", entity.read().unwrap().id, TypeId::of::<T>()))) }
        Ok(
            *entity
            .read()
            .unwrap()
            .component_indices
            .read()
            .unwrap()
            .get(&TypeId::of::<T>())
            .unwrap()
        )
    }

    pub fn has_component<T: Any + Send + Sync>(entity: &ALEntity) -> bool {
        entity
            .read()
            .unwrap()
            .component_indices
            .read()
            .unwrap()
            .contains_key(&TypeId::of::<T>())
    }

    fn make_token() -> u64 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        hasher.finish()
    }
}
