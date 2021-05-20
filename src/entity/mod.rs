pub mod entity_factory;

use std::vec::Vec;

use super::component::ComponentStub;

pub struct Entity {
    pub id: u32,
    //authentication token. proves that a user actually owns the entity and its components.
    pub auth: u64,
    //indicies into ComponentBank's component arrays. index = component type, value = index into that component type's array.
    pub component_stubs: Vec<ComponentStub> //May want to make this into a unique-per-type hash map later.
}
