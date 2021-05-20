use std::collections::{HashSet, HashMap, VecDeque, hash_map::DefaultHasher};
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};

use super::Entity;
use crate::ErrEcs;

pub struct EntityFactory {
    //entity ids currently in use.
    active_entities: HashSet<u32>,
    //entity ids not in use. Might want to remove data at high N for optimization.
    free_entities: VecDeque<u32>,
    //auth_token -> entity id. proves ownership of the entity if the user supplies the token and the entity id.
    auth_entity: HashMap<u64, u32>,
    next_id: u32,
}

impl EntityFactory {
    pub fn new() -> EntityFactory {
        EntityFactory {
            active_entities: HashSet::new(),
            free_entities: VecDeque::new(),
            auth_entity: HashMap::new(),
            next_id: 0,
        }
    }

    fn make_auth(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        hasher.finish()
    }

    fn get_id(&mut self) -> u32 {
        if let Some(id) = self.free_entities.pop_front() { //free_entities always empty.
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }

    //Removes an entity id from an auth token.
    fn disown_id(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        if let Some(id) = self.auth_entity.get(&entity.auth) {
            if *id == entity.id {
                if let Some(_) = self.auth_entity.remove(&entity.auth) {
                    return Ok(())
                } else {
                    return Err(ErrEcs::EntityFactoryOwnerAuthNotFound(format!("entity_id: {}", &entity.id)))
                }
            } else {
                Err(ErrEcs::EntityFactoryBadToken(format!("entity: {}", &entity.id)))
            }
        } else {
            Err(ErrEcs::EntityFactoryOwnerAuthNotFound(format!("entity_id: {}", &entity.id)))
        }
    }

    //Makes an entity id available for reuse.
    fn free_id(&mut self, entity: &Entity) -> Result<(), ErrEcs> {
        if !self.active_entities.remove(&entity.id) {
            return Err(ErrEcs::EntityFactoryActiveEntityNotFound(format!("entity: {}", &entity.id)))
        }
        self.free_entities.push_back(entity.id);
        Ok(())
    }

    pub fn free(&mut self, entity: Entity) -> Result<(), ErrEcs> {
        self.free_id(&entity)?;
        self.disown_id(&entity)
    }

    pub fn create(&mut self) -> Entity {
        let id = self.get_id();
        let auth = self.make_auth();
        self.active_entities.insert(id);
        self.auth_entity.insert(auth, id);
        Entity {
            id: id,
            auth: auth,
            component_stubs: vec![],
        }
    }
}
