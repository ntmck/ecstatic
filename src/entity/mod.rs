use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::any::TypeId;
use std::sync::{Arc, RwLock};

//Arc-RwLock Entity
pub type ALEntity = Arc<RwLock<Entity>>;

pub struct Entity {
    pub id: u64,
    pub components: RwLock<HashSet<TypeId>>,
}

impl Entity {
    pub fn new() -> ALEntity {
        let e = Entity {
            id: Entity::make_token(),
            components: RwLock::new(HashSet::new()),
        };
        Arc::new(RwLock::new(e))
    }

    pub fn insert_component(entity: &ALEntity, component_id: TypeId) {
        entity
            .read()
            .unwrap()
            .components
            .write()
            .unwrap()
            .insert(component_id);
    }

    pub fn remove_component(entity: &ALEntity, component_id: &TypeId) {
        entity
            .read()
            .unwrap()
            .components
            .write()
            .unwrap()
            .remove(component_id);
    }

    pub fn has_component(entity: &ALEntity, component_id: &TypeId) -> bool {
        entity
            .read()
            .unwrap()
            .components
            .read()
            .unwrap()
            .contains(component_id)
    }

    fn make_token() -> u64 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        hasher.finish()
    }
}
