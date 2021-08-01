use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::any::TypeId;
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

    pub fn insert_component(entity: &ALEntity, component_id: TypeId, i: usize) {
        if Entity::has_component(entity, &component_id) { return }
        entity
            .read()
            .unwrap()
            .component_indices
            .write()
            .unwrap()
            .insert(component_id, i);
    }

    pub fn remove_component(entity: &ALEntity, component_id: &TypeId) {
        entity
            .read()
            .unwrap()
            .component_indices
            .write()
            .unwrap()
            .remove(component_id);
    }

    pub fn get_component_index(entity: &ALEntity, component_id: &TypeId) -> Result<usize, ErrEcs> {
        if !Entity::has_component(entity, component_id) { return Err(ErrEcs::EntityComponentNone(format!("entity {} does not have component id: {:?}", entity.read().unwrap().id, component_id))) }
        Ok(
            *entity
            .read()
            .unwrap()
            .component_indices
            .read()
            .unwrap()
            .get(component_id)
            .unwrap()
        )
    }

    pub fn has_component(entity: &ALEntity, component_id: &TypeId) -> bool {
        entity
            .read()
            .unwrap()
            .component_indices
            .read()
            .unwrap()
            .contains_key(component_id)
    }

    fn make_token() -> u64 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        hasher.finish()
    }
}
