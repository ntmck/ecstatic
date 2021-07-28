use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::any::TypeId;

pub struct Entity {
    pub id: u64,
    pub components: HashSet<TypeId>,
}

impl Entity {
    pub fn create() -> Entity {
        Entity {
            id: Entity::make_token(),
            components: HashSet::new(),
        }
    }

    pub fn has_component(entity: &Entity, component_id: &TypeId) -> bool {
        entity.components.contains(component_id)
    }

    fn make_token() -> u64 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        hasher.finish()
    }
}
