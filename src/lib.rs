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

pub mod entity;
pub mod component;
pub mod system;
pub mod data;
pub mod level;

use level::*;
pub use data::*;

pub struct Ecs;

impl Ecs {
    pub fn new_level() -> Level {
        //Default compression ratio: 0.2 in which 80% of capacity is unused.
        Level::new(0.2f64)
    }
}

#[derive(Debug)]
pub enum ErrEcs {
    StorageComponentTypeNotFound(String),
    StorageComponentNotFound(String),

    PackerUnpackIndexOutOfBounds(String),

    CManagerComponentAlreadyExistsForEntity(String),
    CManagerTypeNotFound(String),

    COwnershipEntityNotFound(String),
    COwnershipComponentNotFound(String),

    EManagerOwnerAuthNotFound(String),
    EManagerActiveEntityNotFound(String),
    EManagerWrongIdForToken(String),

    LevelStorageCapacityLessThanOrEqualToZero(String),

    UnimplementedErrorType(String),
}
