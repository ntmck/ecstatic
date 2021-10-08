use std::thread;
use std::thread::JoinHandle;
//use std::sync::mpsc::channel;

use crate::component::*;

pub type StaticSystemFn = fn(&Component);

pub trait StaticSystem {
    fn start(&self, x: &'static Component, f: StaticSystemFn) -> JoinHandle<()> {
        thread::spawn(move || {
            loop {
                f(x);
                //add break condition later.
                break;
            }
        })
    }
}
