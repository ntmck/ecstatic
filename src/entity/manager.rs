use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Entity {
    pub id: u64,
}

pub struct EManager {}

impl EManager {
    pub fn new() -> EManager {
        EManager {}
    }

    pub fn create(&mut self) -> Entity {
        Entity {
            id: self.make_token(),
        }
    }

    fn make_token(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        hasher.finish()
    }
}
