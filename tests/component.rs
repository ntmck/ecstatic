use std::assert;
extern crate ecstatic;
use ecstatic::*;

#[test]
pub fn test_insert_component() {
    let mut ecs = Ecs::new();
    let mut entity = ecs.create_entity();
    match ecs.ecact(Action::Insert, &mut entity, position![]) {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }
}
