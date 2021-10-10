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
use std::sync::Arc;
use std::marker::{Send, Sync};
use std::any::Any;

use lazy_static::lazy_static;

pub mod data_structures;
pub mod trait_common;

use trait_common::*;

// Implementations:
use ecstatic_storage::*;

pub struct Ecs {
    ecstorage: Arc<EcstaticStorage>,
}
impl ComponentStorage for Ecs {
    fn ecinsert<T>(&self, id: u64, component: T)
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.ecstorage.ecinsert::<T>(id, component).expect("Error on insert\n");
    }

    fn ecset<T>(&self, id: u64, with: T)
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.ecstorage.ecset::<T>(id, with).expect("Error on set\n");
    }

    fn ecmodify<T>(&self, id: u64, f: fn(&mut T))
    where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.ecstorage.ecmodify::<T>(id, f).expect("Error on modify\n");
    }

    fn ecread<T>(&self, id: u64) -> T
    where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.ecstorage.ecread::<T>(id).expect("Error on read\n")
    }

    fn ecempty<T>(&self, id: u64)
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.ecstorage.ecempty::<T>(id).expect("Error on empty\n");
    }

    fn capacity<T>(&self) -> usize
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.ecstorage.capacity::<T>().expect("Error on capacity\n")
    }

    fn len<T>(&self) -> usize
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.ecstorage.len::<T>().expect("Error on len\n")
    }

    fn compress_memory<T>(&self)
    where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        self.ecstorage.compress_memory::<T>().expect("Error on memory\n");
    }
}
impl StaticSystem<'static> for Ecs{}
impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            ecstorage: EcstaticStorage::new(),
        }
    }
}

//Used to guarantee lifetime of the ECS for systems to operate on it via multiple threads.
lazy_static! {
    static ref ECS: Ecs = {
        Ecs::new()
    };
}

fn main() {
    ECS.ecstorage.ecinsert::<u8>(0, 2);
    let (sx, handle) = ECS.system(|cs| {
        print!("test: ");
        print!("{}\n", cs.len::<u8>());
    });
    for _ in 0..10 {
        sx.send(Signal::Pump);
    }
    sx.send(Signal::Stop);
    handle.join();
}