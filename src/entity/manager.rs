use std::collections::{HashSet, HashMap, hash_map::DefaultHasher};
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};

use crate::ErrEcs;

pub struct Entity {
    //identification token.
    pub id: u64,
    //authentication token. proves that a user actually owns the entity and its components.
    pub auth: u64, //use in lib functions to authenticate.
}

pub struct EManager {
    //entity ids currently in use.
    active_entities: HashSet<u64>,
    //auth_token -> entity id. proves ownership of the entity if the user supplies the token and the entity id.
    auth_entity: HashMap<u64, u64>,
}

impl EManager {
    pub fn new() -> EManager {
        EManager {
            active_entities: HashSet::new(),
            auth_entity: HashMap::new(),
        }
    }

    fn make_token(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        hasher.finish()
    }

    //Removes an entity id from an auth token.
    fn disown_id(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        if let Some(id) = self.auth_entity.get(&entity.auth) {
            if *id == entity.id {
                if let Some(_) = self.auth_entity.remove(&entity.auth) {
                    return Ok(())
                } else {
                    return Err(ErrEcs::EManagerOwnerAuthNotFound(format!("entity: {}", &entity.id)))
                }
            } else {
                Err(ErrEcs::EManagerWrongIdForToken(format!("entity: {}", &entity.id)))
            }
        } else {
            Err(ErrEcs::EManagerOwnerAuthNotFound(format!("entity: {}", &entity.id)))
        }
    }

    //Makes an entity id available for reuse.
    fn free_id(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        if !self.active_entities.remove(&entity.id) {
            return Err(ErrEcs::EManagerActiveEntityNotFound(format!("entity: {}", &entity.id)))
        }
        Ok(())
    }

    pub fn authenticate(&self, en: &Entity) -> Result<(), ErrEcs> {
        Ok(())
    }

    pub fn free(&mut self, entity: Entity) -> Result<(), ErrEcs> {
        self.free_id(&entity)?;
        self.disown_id(&entity)
    }

    pub fn create(&mut self) -> Entity {
        let id = self.make_token();
        let auth = self.make_token();
        self.active_entities.insert(id);
        self.auth_entity.insert(auth, id);
        Entity {
            id: id,
            auth: auth,
        }
    }
}
