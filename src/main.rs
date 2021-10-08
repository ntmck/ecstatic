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
use std::any::Any;
use std::sync::Arc;
use std::marker::{Send, Sync};
use std::panic;

pub mod data_structures;
pub mod level;

// Implementations:
use ecstatic_storage::*;

pub trait ComponentStorage {
    fn ecinsert<T>           (&self, id: u64, component: T)     where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe;
    fn ecset<T>              (&self, id: u64, with: T)          where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe;
    fn ecmodify<T>           (&self, id: u64, f: fn(&mut T))    where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe;
    fn ecread<T>             (&self, id: u64) -> T              where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe;
    fn ecempty<T>            (&self, id: u64)                   where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe;
    fn capacity<T>           (&self) -> usize                   where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe;
    fn len<T>                (&self) -> usize                   where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe;
    fn compress_memory<T>    (&self)                            where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe;
}

pub struct EcsStorage {
    component_storage: Arc<EcstaticStorage>,
}
impl EcsStorage {
    pub fn new() -> EcsStorage {
        EcsStorage{
            component_storage: EcstaticStorage::new(),
        }
    }
}
impl ComponentStorage for EcsStorage {
    fn ecinsert<T>(&self, id: u64, component: T)
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.component_storage.ecinsert::<T>(id, component).expect("Error on insert\n");
    }

    fn ecset<T>(&self, id: u64, with: T)
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.component_storage.ecset::<T>(id, with).expect("Error on set\n");
    }

    fn ecmodify<T>(&self, id: u64, f: fn(&mut T))
    where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.component_storage.ecmodify::<T>(id, f).expect("Error on modify\n");
    }

    fn ecread<T>(&self, id: u64) -> T
    where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.component_storage.ecread::<T>(id).expect("Error on read\n")
    }

    fn ecempty<T>(&self, id: u64)
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.component_storage.ecempty::<T>(id).expect("Error on empty\n");
    }

    fn capacity<T>(&self) -> usize
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.component_storage.capacity::<T>().expect("Error on capacity\n")
    }

    fn len<T>(&self) -> usize
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.component_storage.len::<T>().expect("Error on len\n")
    }

    fn compress_memory<T>(&self)
    where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.component_storage.compress_memory::<T>().expect("Error on memory\n");
    }
}

fn main() {

}