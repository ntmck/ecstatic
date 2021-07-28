use std::assert;
extern crate ecstatic;
use ecstatic::*;

#[test]
pub fn test_activate_entity() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();
    assert!(level.is_entity_active(&entity));
}

#[test]
pub fn test_free_entity() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();
    let eclone = entity.clone();
    level.ecfree(entity);
    assert!(!level.is_entity_active(&eclone));
}
