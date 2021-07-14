use std::assert;
use std::vec::Vec;

extern crate ecstatic;
use ecstatic::*;

use ecstatic::level::Level;
use ecstatic::entity::Entity;

//Setup Memory Layout 1: [i = none] [i+1 = none]  ... [i+n-2 = none] [i+n-1 = some]
fn setup_layout_1() -> Level {
    let mut level = Ecs::new_level();
    let mut ev: Vec<Entity> = vec![];
    //fill capacity.
    for i in 0..100 {
        let e = level.espawn();
        level.ecgive::<Position2d>(&e, Position2d{x:0.0, y:0.0});
        ev.push(e.clone());
    }
    //remove up to the end. memory is still allocated after remove.
    for i in 0..99 {
        level.ecfree(ev[i]);
    }
    level
}

fn setup_layout_1_last_entity() -> (Level, Entity) {
    let mut level = Ecs::new_level();
    let mut ev: Vec<Entity> = vec![];

    for i in 0..99 {
        let e = level.espawn();
        level.ecgive::<Position2d>(&e, Position2d{x:0.0, y:0.0});
        ev.push(e.clone());
    }

    let e = level.espawn();
    level.ecgive::<Position2d>(&e, Position2d{x:1.0, y:2.0});
    ev.push(e.clone());

    for i in 0..99 {
        level.ecfree(ev[i]);
    }
    (level, e)
}

//Setup Memory Layout 2: [2i = None], [2i+1 = Some]
fn setup_layout_2() -> Level {
    let mut level = Ecs::new_level();
    let mut ev: Vec<Entity> = vec![];
    //fill capacity.
    for i in 0..100 {
        let e = level.espawn();
        level.ecgive::<Position2d>(&e, Position2d{x:0.0, y:0.0});
        ev.push(e.clone());
    }
    //free half the memory. len = 50, capacity = ~100.
    for i in 0..100 {
        if i%2 == 0 { level.ecfree(ev[i]); }
    }
    level
}

fn setup_layout_2_entities() -> (Level, Vec<Entity>) {
    let mut level = Ecs::new_level();
    let mut ev: Vec<Entity> = vec![];

    for i in 0..100 {
        let e = level.espawn();
        level.ecgive::<Position2d>(&e, Position2d{x:i as f64, y:i as f64 + 1.0}); //x = i, y = i+1
        ev.push(e.clone());
    }
    let mut validate_entities = vec![];
    for i in 0..100 {
        if i%2 == 0 {
            level.ecfree(ev[i]);
        } else {
            validate_entities.push(ev[i]);
        }
    }
    (level, validate_entities)
}

//First element has 1 entity, rest is empty.
fn setup_layout_3() -> Level {
    let mut level = Ecs::new_level();
    let mut ev: Vec<Entity> = vec![];
    for i in 0..100 {
        let e = level.espawn();
        level.ecgive::<Position2d>(&e, Position2d{x:0.0, y:0.0});
        ev.push(e.clone());
    }
    for i in 1..100 {
        level.ecfree(ev[i]);
    }
    level
}

#[test]
pub fn test_compress_memory_layout_1_validate_entity() {
    let (mut level, entity) = setup_layout_1_last_entity();
    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }
    match level.ecget::<Position2d>(&entity) {
        Ok(pos) => {assert!(pos.x == 1.0 && pos.y == 2.0, "Failed to validate entity.")},
        Err(e) => panic!("{:#?}", e)
    }
}

//Throws index out of bounds only sometimes. weird. maybe increase capacity by 1 in resize?
#[test]
pub fn test_compress_memory_layout_2_validate_entities() {
    let (mut level, evec) = setup_layout_2_entities();
    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }
    for (i, e) in evec.iter().enumerate() {
        match level.ecget::<Position2d>(e) {
            Ok(pos) => { //expect x=2i+1 & y=2i+2
                assert!(pos.x == 2.0*i as f64+1.0 && pos.y == 2.0*i as f64+2.0,
                "Failed to validate entities.");
            },
            Err(e) => panic!("{:#?}", e)
        }
    }
}

#[test]
pub fn test_compress_memory_layout_1() {
    let mut level = setup_layout_1();

    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }

    assert!(level.clen::<Position2d>() == 2 && level.ccapacity::<Position2d>() == 2,
     "Failed to compress layout 1. len: {}, capacity: {}",
     level.clen::<Position2d>(), level.ccapacity::<Position2d>());
}

#[test]
pub fn test_compress_memory_layout_2() {
    let mut level = setup_layout_2();

    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }

    assert!(level.clen::<Position2d>() == 51 && level.ccapacity::<Position2d>() == 51,
     "Failed to compress layout 2. len: {}, capacity: {}",
     level.clen::<Position2d>(), level.ccapacity::<Position2d>());
}

#[test]
pub fn test_compress_memory_layout_1_packer() {
    let mut level = setup_layout_1();

    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }

    assert!(level.plen::<Position2d>() == 1,
    "Failed to manage the packer during compression of layout 1. plen: {}",
    level.plen::<Position2d>());
}

#[test]
pub fn test_compress_memory_layout_2_packer() {
    let mut level = setup_layout_2();

    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }

    assert!(level.plen::<Position2d>() == 50,
    "Failed to manage the packer during compression of layout 2. plen: {}",
    level.plen::<Position2d>());
}

#[test]
pub fn test_compress_memory_layout_3() {
    let mut level = setup_layout_3();

    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }

    assert!(level.clen::<Position2d>() == 2 && level.ccapacity::<Position2d>() == 2,
     "Failed to compress layout 3. len: {}, capacity: {}",
     level.clen::<Position2d>(), level.ccapacity::<Position2d>());
}

#[test]
pub fn test_compress_memory_layout_3_packer() {
    let mut level = setup_layout_3();

    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }

    assert!(level.plen::<Position2d>() == 1,
    "Failed to manage the packer during compression of layout 3. plen: {}",
    level.plen::<Position2d>());
}
