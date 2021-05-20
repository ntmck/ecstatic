use std::assert;

extern crate ecstatic;
use ecstatic::{Ecs, ErrEcs};

#[test]
pub fn test_entity_id_recycling() {
    let mut ecs = Ecs::new();

    for i in 0..100 {
        let entity1 = ecs.create_entity();
        assert_eq!(entity1.id, 0);
        let entity2 = ecs.create_entity();
        assert_eq!(entity2.id, 1);
        ecs.destroy_entity(entity1);
        ecs.destroy_entity(entity2);
    }
}

#[test]
#[should_panic]
pub fn test_entity_bad_auth() {
    let mut ecs = Ecs::new();
    let mut e = ecs.create_entity();
    e.auth = 5432;
    match ecs.destroy_entity(e) {
        Ok(_) => assert!(true, "Unauthorized access to entity. Bad token."),
        Err(e) => assert!(false)
    }
}

#[test]
pub fn test_entity_ok_auth() {
    let mut ecs = Ecs::new();
    let mut e = ecs.create_entity();
    match ecs.destroy_entity(e) {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }
}
