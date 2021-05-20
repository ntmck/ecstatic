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

use entity::{Entity, entity_factory::EntityFactory};

use component::*;
use system::*;

pub struct Ecs {
    entity_factory: EntityFactory,
}

impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            entity_factory: EntityFactory::new(),
        }
    }

    //Should include position by default.
    //Should call entity's on_create function component.
    pub fn create_entity(&mut self) -> Entity {
        self.entity_factory.create()
    }

    //Should call entity's on_destroy function component.
    pub fn destroy_entity(&mut self, entity: Entity) -> Result<(), ErrEcs> {
        self.entity_factory.free(entity)
    }
}
#[derive(Debug)]
pub enum ErrEcs {
    EntityFactoryOwnerAuthNotFound(String),
    EntityFactoryActiveEntityNotFound(String),
    EntityFactoryBadToken(String),
}
