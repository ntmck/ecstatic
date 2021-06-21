pub mod entity_factory;
pub use entity_factory::EntityFactory;

pub struct Entity {
    //identification token.
    pub id: u64,
    //authentication token. proves that a user actually owns the entity and its components.
    pub auth: u64, //use in lib functions to authenticate.
}
