/*

ECStatic game engine designed for use in small games.

Copyright (C) 2021 Nathan McKnight <ntmck1@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

For more details see: <http://www.gnu.org/licenses/>.

*/

mod entity;
mod component;
mod system;
mod data;

use entity::*;
use component::*;
pub use data::*;

//use system::*;

#[derive(Debug)]
pub enum Action {
    Insert,
    Update,
    Remove,
    Free,
}

#[derive(Debug)]
pub enum Component {
    Position(Option<Vec3>),
    Rotation(Option<Vec3>),
    Init(Option<Init>),
    Update(Option<Update>),
    Destroy(Option<Destroy>),
}

//Refactor to allow rotation/position to share the same underlying macro?
#[macro_export]
macro_rules! position {
    () => {
        {
            Component::Position(Some(Vec3{x: 0f32, y: 0f32, z: 0f32}))
        }
    };
    ($x:expr) => {
        {
            Component::Position(Some(Vec3{x: $x as f32, y: 0f32, z: 0f32}))
        }
    };
    ($x:expr, $y:expr) => {
        {
            Component::Position(Some(Vec3{x: $x as f32, y: $y as f32, z: 0f32}))
        }
    };
    ($x:expr, $y:expr, $z:expr) => {
        {
            Component::Position(Some(Vec3{x: $x as f32, y: $y as f32, z: $z as f32}))
        }
    };
}

pub struct Ecs {
    entity_factory: EntityFactory,
    component_factory: ComponentFactory,
}

impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            entity_factory: EntityFactory::new(),
            component_factory: ComponentFactory::new(),
        }
    }

    //Should include position by default.
    //Should call entity's on_create function component.
    pub fn create_entity(&mut self) -> Entity {
        self.entity_factory.create()
    }

    //Should call entity's on_destroy function component.
    pub fn destroy_entity(&mut self, entity: Entity) -> Result<(), ErrEcs> {
        self.entity_factory.authenticate(&entity)?;
        self.entity_factory.free(entity)
    }

    //Entity-Component Action
    pub fn ecact(&mut self, act: Action, entity: &mut Entity, cmp: Component) -> Result<(), ErrEcs> {
        match act {
            Action::Insert => {
                self.component_factory.insert_component(entity, cmp);
                Ok(())
            },
            Action::Update => Ok(()),
            Action::Remove => Ok(()),
            Action::Free => Ok(()),
        }
    }

    //Entity-System Action
    pub fn esact(&mut self) {

    }

    //Component-System Action
    pub fn csact(&mut self) {

    }
}
#[derive(Debug)]
pub enum ErrEcs {
    ComponentFactoryEntityAlreadyHasComponent(String),

    EntityFactoryOwnerAuthNotFound(String),
    EntityFactoryActiveEntityNotFound(String),
    EntityFactoryWrongIdForToken(String),
}
