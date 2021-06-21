use std::assert;
extern crate ecstatic;
use ecstatic::*;

#[test]
pub fn test_insert_component() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();
    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    let pos = level.ecget::<Position2d>(&entity);
    assert!(0.0 == pos.x && 0.1 == pos.y);
}
