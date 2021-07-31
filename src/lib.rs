/*

ECStatic game engine designed for use in small 2d games.

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

#![feature(map_first_last)]

pub mod entity;
pub mod component;
pub mod data;
pub mod level;

//use level::*;
//pub use data::*;

pub struct Ecs;

impl Ecs {
}

#[derive(Debug)]
pub enum ErrEcs {
    ComponentLock(String),
    ComponentMapNone(String),
    ComponentValueNone(String),
    ComponentDowncast(String),
}
