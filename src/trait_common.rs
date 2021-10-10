use std::marker::{Send, Sync};
use std::any::Any;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;

/// Interface for a Component Storage System.
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

pub enum Signal {
    Stop,
    Pump,
}

pub trait StaticSystem<'a: 'static> {
    fn system(&'a self, f: fn(Arc<&Self>)) -> (Sender<Signal>, JoinHandle<()>) 
    where Self: ComponentStorage + Send + Sync
    {
        let (sx, rx): (Sender<Signal>, Receiver<Signal>) = mpsc::channel();
        let aself = Arc::new(self);
        let handle = thread::spawn(move || {
            loop {
                let aselfc = aself.clone();
                match rx.recv().unwrap() {
                    Signal::Stop => break,
                    Signal::Pump => f(aselfc),
                }
            }
        });
        (sx, handle)
    }
}