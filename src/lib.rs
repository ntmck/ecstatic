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
mod level;

use entity::*;
use component::*;
use level::*;
pub use data::*;

pub struct Ecs;

impl Ecs {
    pub fn new_level() -> Level {
        Level::new()
    }
}

#[derive(Debug)]
pub enum ErrEcs {
    CManagerComponentTypeNotFound(String),
    CManagerComponentNotFound(String),
    CManagerUnpackIndexNotFound(String),
    CManagerComponentAlreadyExistsForEntity(String),
    CManagerEntityNotFound(String),

    EManagerOwnerAuthNotFound(String),
    EManagerActiveEntityNotFound(String),
    EManagerWrongIdForToken(String),
}
