/*
Ideas:
    try to remove from len to capacity then shrink to fit?
    try to replace the underlying vector with a new vector?
    try to keep our own count of the number of components inside a vector and compare?
*/

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

#[test]
pub fn test_compress_memory_layout_1() {
    let mut level = setup_layout_1();

    match level.compress_component_memory::<Position2d>() {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }

    assert!(level.clen::<Position2d>() == 1 && level.ccapacity::<Position2d>() == 1,
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

    assert!(level.clen::<Position2d>() == 50 && level.ccapacity::<Position2d>() == 50,
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