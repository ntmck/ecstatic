use std::assert;
extern crate ecstatic;
use ecstatic::*;

#[test]
pub fn test_position_macro() {
    let pos1 = unwrap_position(position![]);
    let pos2 = unwrap_position(position![1]);
    let pos3 = unwrap_position(position![1, 2]);
    let pos4 = unwrap_position(position![1, 2, 3]);
    assert!(pos1.x == 0f32 && pos1.y == 0f32 && pos1.z == 0f32, "pos1");
    assert!(pos2.x == 1f32 && pos2.y == 0f32 && pos2.z == 0f32, "pos2");
    assert!(pos3.x == 1f32 && pos3.y == 2f32 && pos3.z == 0f32, "pos3");
    assert!(pos4.x == 1f32 && pos4.y == 2f32 && pos4.z == 3f32, "pos4");
}

fn unwrap_position(wrap: Option<Component>) -> Vec3 {
    if let Some(comp) = wrap {
        match comp {
            Component::Position(opv3) => {
                if let Some(v3) = opv3 {
                    v3
                } else { panic!("unwrap_position: Vec3 is None.") }
            },
            _ => panic!("unwrap_position: Component is not a position component.")
        }
    } else { panic!("unwrap_position: Component is None.") }
}

#[test]
pub fn test_insert_component() {
    let mut ecs = Ecs::new();
    let mut entity = ecs.create_entity();
    match ecs.ecact(Action::Insert, &mut entity, position![]) {
        Ok(_) => (),
        Err(e) => assert!(false, "{:#?}", e)
    }
}
