use std::assert;
extern crate ecstatic;
use ecstatic::*;

#[test]
pub fn test_insert_component() {
    let mut ecs = Ecs::new();
    let mut entity = ecs.create_entity();
    //TODO: Find a way to abstract some of the 'Some' wrapping.
    match ecs.ecact(Action::Insert, &mut entity, Some(Component::Position(Some(Vec3{x: 0f32, y: 0f32, z: 0f32})))) {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }
}
