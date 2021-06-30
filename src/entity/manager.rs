use std::collections::{HashSet, hash_map::DefaultHasher};
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};

use crate::ErrEcs;

#[derive(Clone, Copy)]
pub struct Entity {
    pub id: u64,
}

pub struct EManager {
    active_entity_ids: HashSet<u64>,
}

impl EManager {
    pub fn new() -> EManager {
        EManager {
            active_entity_ids: HashSet::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
        let id = self.make_token();
        self.active_entity_ids.insert(id);
        Entity {
            id: id,
        }
    }

    pub fn deactivate_entity(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        if !self.active_entity_ids.remove(&entity.id) {
            return Err(ErrEcs::EManagerActiveEntityNotFound(format!("entity: {}", &entity.id)))
        }
        Ok(())
    }

    pub fn is_entity_active(&self, entity: &Entity) -> bool {
        self.active_entity_ids.get(&entity.id) != None
    }

    fn make_token(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        hasher.finish()
    }
}
