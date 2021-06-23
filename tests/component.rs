use std::assert;
extern crate ecstatic;
use ecstatic::*;

#[test]
pub fn test_insert_component_and_get_component() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();

    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    let pos = level.ecget::<Position2d>(&entity).unwrap();
    assert!(0.0 == pos.x && 0.1 == pos.y);
}

#[test]
#[should_panic]
pub fn test_multiple_insert_component_of_same_type() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();

    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    match level.ecgive::<Position2d>(&entity, Position2d{x:0.1, y:0.1}) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
pub fn test_insert_multiple_components() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();

    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    level.ecgive::<Position3d>(&entity, Position3d{x:0.2, y:0.3, z: 0.4});
    let pos1 = level.ecget::<Position2d>(&entity).unwrap();
    assert!(0.0 == pos1.x && 0.1 == pos1.y);
    let pos2 = level.ecget::<Position3d>(&entity).unwrap();
    assert!(0.2 == pos2.x && 0.3 == pos2.y, 0.4 == pos2.z);
}

#[test]
#[should_panic]
pub fn test_insert_then_remove() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();

    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    level.ecremove::<Position2d>(&entity);
    match level.ecget::<Position2d>(&entity) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
pub fn test_insert_then_update() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();

    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    level.ecset::<Position2d>(&entity, Position2d{x:1.1, y:2.2});
    let pos = level.ecget::<Position2d>(&entity).unwrap();
    assert!(1.1 == pos.x && 2.2 == pos.y);
}

#[test]
#[should_panic]
pub fn test_update_on_none() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();
    match level.ecset::<Position2d>(&entity, Position2d{x:1.1, y:2.2}) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
#[should_panic]
pub fn test_remove_on_none() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();
    match level.ecremove::<Position2d>(&entity) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
#[should_panic]
pub fn test_components_removed() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();
    let eclone = entity.clone();
    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    level.ecgive::<Position3d>(&entity, Position3d{x:0.2, y:0.3, z: 0.4});
    level.ecfree(entity);

    match level.ecget::<Position2d>(&eclone) {
        Ok(_) => (),
        Err(_) => match level.ecget::<Position3d>(&eclone) {
            Ok(_) => (),
            Err(_) => panic!()
        }
    }
}

#[test]
pub fn test_replace_component() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();

    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    level.ecremove::<Position2d>(&entity);
    level.ecgive::<Position2d>(&entity, Position2d{x:0.1, y:0.2});
    let pos = level.ecget::<Position2d>(&entity).unwrap();
    assert!(pos.x == 0.1 && pos.y == 0.2);
}

#[test]
pub fn test_insert_set_remove_insert_set() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();

    level.ecgive::<Position2d>(&entity, Position2d{x:0.0, y:0.1});
    level.ecset::<Position2d>(&entity, Position2d{x:0.2, y:0.2});
    level.ecremove::<Position2d>(&entity);
    level.ecgive::<Position2d>(&entity, Position2d{x:1.0, y:2.1});
    let pos = level.ecget::<Position2d>(&entity).unwrap();
    assert!(pos.x == 1.0 && pos.y == 2.1);
}

fn my_func() -> u32{
    9001u32
}

#[test]
pub fn test_insert_and_get_fn_pointer() {
    let mut level = Ecs::new_level();
    let entity = level.espawn();

    type MyFunc = fn() -> u32;
    let f: MyFunc = my_func;

    level.ecgive::<MyFunc>(&entity, f);

    let ec_func = level.ecget::<MyFunc>(&entity).unwrap();
    let x = ec_func();
    assert!(x == 9001u32);
}
