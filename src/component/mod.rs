use std::any::{Any, TypeId};
use std::any::type_name;
use std::marker::{Send, Sync};
use std::collections::{HashMap, BTreeSet};
use std::sync::{Arc, RwLock};

//ignore these warnings as they are relevant for testing.
use std::sync::Barrier;
use std::thread;
use std::sync::mpsc::channel;

use crate::ErrEcs;

//RwLock Component
pub type LComponent = RwLock<Box<dyn Any + Send + Sync>>;

//RwLock Component store
pub type LComponentStore = RwLock<Vec<LComponent>>;

//Arc-RwLock Component Storage {Arc<RwLock<HashMap<TypeId, Arc<RwLock<Vec<Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>>>>}
pub type ALComponentStorage = Arc<RwLock<HashMap<TypeId, LComponentStore>>>;

//Arc-RwLock index storage. Used for both indices indices and freed indices. [0] = indices, [1] = free
const PACKED: usize = 0;
const FREE: usize = 1;
pub type ALIndices = Arc<RwLock<[HashMap<TypeId, RwLock<BTreeSet<usize>>>; 2]>>;

//An empty type for emptying component memory.
enum Empty { Empty }

pub struct Component;
impl Component {
    pub fn new() -> (ALComponentStorage, ALIndices) {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        let indices = Arc::new(RwLock::new([HashMap::new(), HashMap::new()]));
        (storage, indices)
    }

    //inserts a component into storage by type and returns the index it was inserted into.
    pub fn insert<T: Any + Send + Sync>(component: T, storage: &ALComponentStorage, indices: &ALIndices) -> Result<usize, ErrEcs> {
        Component::check_initialized_component_vector::<T>(storage);
        let len = Component::len::<T>(storage).unwrap();
        let i = Component::get_index::<T>(indices, len);
        match storage.read() {
            Ok(map) => {
                match map.get(&TypeId::of::<T>()) {
                    Some(lvec) => {
                        match lvec.write() {
                            Ok(mut vec) => {
                                if i == len {
                                    vec.push(RwLock::new(Box::new(component)));
                                } else {
                                    vec[i] = RwLock::new(Box::new(component));
                                }
                            },
                            Err(e) => return Err(ErrEcs::ComponentLock(format!("Component::insert || Error acquiring vector lock. {:#?}", e)))
                        }
                    },
                    None => return Err(ErrEcs::ComponentMapNone(format!("Component::insert || Component map is None.")))
                }
            },
            Err(e) => return Err(ErrEcs::ComponentLock(format!("Component::insert || Error acquiring storage lock. {:#?}", e)))
        }
        Ok(i)
    }

    //replaces a component at the given index with the given component.
    pub fn set<T: Any + Send + Sync + Copy>(i: usize, component: T, storage: &ALComponentStorage) -> Result<(), ErrEcs>{
        Component::check_initialized_component_vector::<T>(storage);
        match storage.read() {
            Ok(map) => {
                match map.get(&TypeId::of::<T>()) {
                    Some(lvec) => {
                        match lvec.read() {
                            Ok(vec) => {
                                match vec.get(i) {
                                    Some(lvalue) => {
                                        match lvalue.write() {
                                            Ok(mut value) => {
                                                *value = Box::new(component);
                                                Ok(())
                                            },
                                            Err(e) =>  Err(ErrEcs::ComponentLock(format!("Component::set || Error acquiring value lock. {:#?}", e)))
                                        }
                                    },
                                    None => Err(ErrEcs::ComponentValueNone(format!("Component::set || Value in vector is None.")))
                                }
                            },
                            Err(e) => Err(ErrEcs::ComponentLock(format!("Component::set || Error acquiring vector lock. {:#?}", e)))
                        }
                    },
                    None => Err(ErrEcs::ComponentMapNone(format!("Component::set || Component map is None.")))
                }
            },
            Err(e) => Err(ErrEcs::ComponentLock(format!("Component::set || Error acquiring storage lock. {:#?}", e)))
        }
    }

    //gets the component at given index. NOTE: This works for now, but it required an additional + Copy trait...
    //TODO: Consider just returning the locked value.
    pub fn get<T: Any + Send + Sync + Copy>(i: usize, storage: &ALComponentStorage) -> Result<T, ErrEcs> {
        Component::check_initialized_component_vector::<T>(storage);
        if !Component::is_empty::<T>(i, storage)? {
            match storage.read() {
                Ok(map) => {
                    match map.get(&TypeId::of::<T>()) {
                        Some(lvec) => {
                            match lvec.read() {
                                Ok(vec) => {
                                    match vec.get(i) {
                                        Some(lvalue) => {
                                            match lvalue.read() {
                                                Ok(value) => match value.downcast_ref::<T>() {
                                                    Some(value) => Ok(*value),
                                                    None => Err(ErrEcs::ComponentDowncast(format!("Component::get || Failed to downcast to type: {:#?}", type_name::<T>())))
                                                },
                                                Err(e) => Err(ErrEcs::ComponentLock(format!("Component::get || Error acquiring value lock. {:#?}", e)))
                                            }
                                        },
                                        None => Err(ErrEcs::ComponentValueNone(format!("Component::get || Value in vector is None.")))
                                    }
                                },
                                Err(e) => Err(ErrEcs::ComponentLock(format!("Component::get || Error acquiring vector lock. {:#?}", e)))
                            }
                        },
                        None => Err(ErrEcs::ComponentMapNone(format!("Component::get || Component map is None.")))
                    }
                },
                Err(e) => Err(ErrEcs::ComponentLock(format!("Component::get || Error acquiring storage lock. {:#?}", e)))
            }
        } else { panic!("unimplemented error handling on empty.") }
    }

    //Empties, but doesn't deallocate, memory at index for a component type.
    pub fn empty<T: Any + Send + Sync>(i: usize, storage: &ALComponentStorage, indices: &ALIndices) -> Result<(), ErrEcs> {
        Component::check_initialized_component_vector::<T>(storage);
        match storage.read() {
            Ok(map) => {
                match map.get(&TypeId::of::<T>()) {
                    Some(lvec) => {
                        match lvec.read() {
                            Ok(vec) => {
                                match vec.get(i) {
                                    Some(lvalue) => {
                                        match lvalue.write() {
                                            Ok(mut value) => {
                                                *value = Box::new(Empty::Empty);
                                            },
                                            Err(e) => return Err(ErrEcs::ComponentLock(format!("Component::empty || Error acquiring value lock. {:#?}", e)))
                                        }
                                    },
                                    None => return Err(ErrEcs::ComponentValueNone(format!("Component::empty || Value in vector is None.")))
                                }
                            },
                            Err(e) => return Err(ErrEcs::ComponentLock(format!("Component::empty || Error acquiring vector lock. {:#?}", e)))
                        }
                    },
                    None => return Err(ErrEcs::ComponentMapNone(format!("Component::empty || Component map is None.")))
                }
            },
            Err(e) => return Err(ErrEcs::ComponentLock(format!("Component::empty || Error acquiring storage lock. {:#?}", e)))
        }
        Component::free_index::<T>(i, indices);
        Ok(())
    }

    //returns the length of the underlying component vector by type.
    pub fn len<T: Any + Send + Sync>(storage: &ALComponentStorage) -> Result<usize, ErrEcs> {
        match storage.read() {
            Ok(map) => {
                match map.get(&TypeId::of::<T>()) {
                    Some(lvec) => {
                        match lvec.read() {
                            Ok(vec) => {
                                Ok(vec.len())
                            },
                            Err(e) => Err(ErrEcs::ComponentLock(format!("Component::len || Error acquiring vector lock. {:#?}", e)))
                        }
                    },
                    None => Err(ErrEcs::ComponentMapNone(format!("Component::len || Component map is None.")))
                }
            },
            Err(e) => Err(ErrEcs::ComponentLock(format!("Component::len || Error acquiring storage lock. {:#?}", e)))
        }
    }

    //checks if the value of a type is empty.
    fn is_empty<T: Any + Send + Sync>(i: usize, storage: &ALComponentStorage) -> Result<bool, ErrEcs> {
        Component::check_initialized_component_vector::<T>(storage);
        match storage.read() {
            Ok(map) => {
                match map.get(&TypeId::of::<T>()) {
                    Some(lvec) => {
                        match lvec.read() {
                            Ok(vec) => {
                                match vec.get(i) {
                                    Some(lvalue) => {
                                        match lvalue.read() {
                                            Ok(value) => match value.downcast_ref::<Empty>() {
                                                Some(_) => Ok(true),
                                                None => Ok(false)
                                            },
                                            Err(e) => Err(ErrEcs::ComponentLock(format!("Component::is_empty || Error acquiring value lock. {:#?}", e)))
                                        }
                                    },
                                    None => Err(ErrEcs::ComponentValueNone(format!("Component::is_empty || Value in vector is None.")))
                                }
                            },
                            Err(e) => Err(ErrEcs::ComponentLock(format!("Component::is_empty || Error acquiring vector lock. {:#?}", e)))
                        }
                    },
                    None => Err(ErrEcs::ComponentMapNone(format!("Component::is_empty || Component map is None.")))
                }
            },
            Err(e) => Err(ErrEcs::ComponentLock(format!("Component::is_empty || Error acquiring storage lock. {:#?}", e)))
        }
    }

    //attempts to retrieve an index from freed indices. if it finds none, it uses the provided default index. returns the index it used.
    fn get_index<T: Any + Send + Sync>(indices: &ALIndices, default_index: usize) -> usize {
        Component::check_initialized_index_set::<T>(PACKED, indices);
        Component::check_initialized_index_set::<T>(FREE, indices);
        let index: usize;
        match indices
            .read().unwrap()
            .get(FREE).unwrap()
            .get(&TypeId::of::<T>()).unwrap()
            .write().unwrap()
            .first() {
                Some(first) => index = *first,
                None => index = default_index,
            }

        indices
            .read().unwrap()
            .get(PACKED).unwrap()
            .get(&TypeId::of::<T>()).unwrap()
            .write().unwrap()
            .insert(index);

        index
    }

    //removes index from indices and places into free.
    fn free_index<T: Any + Send + Sync>(i: usize, indices: &ALIndices) {
        Component::check_initialized_index_set::<T>(PACKED, indices);
        Component::check_initialized_index_set::<T>(FREE, indices);
        let removed = indices
            .read().unwrap()
            .get(PACKED).unwrap()
            .get(&TypeId::of::<T>()).unwrap()
            .write().unwrap()
            .remove(&i);

        if removed {
            indices
                .read().unwrap()
                .get(FREE).unwrap()
                .get(&TypeId::of::<T>()).unwrap()
                .write().unwrap()
                .insert(i);
        }
    }

    fn check_initialized_component_vector<T: Any + Send + Sync>(storage: &ALComponentStorage) {
        let is_initialized;
        match storage
            .read().unwrap()
            .get(&TypeId::of::<T>()) {
                Some(_) => is_initialized = true,
                None => is_initialized = false,
            }
        if !is_initialized {
            Component::initialize_storage_vector::<T>(storage);
        }
    }

    fn initialize_storage_vector<T: Any + Send + Sync>(storage: &ALComponentStorage) {
        storage
            .write().unwrap()
            .insert(TypeId::of::<T>(), RwLock::new(vec![]));
    }

    fn check_initialized_index_set<T: Any + Send + Sync>(which: usize, indices: &ALIndices) {
        let is_initialized;
        match indices
            .read().unwrap()
            .get(which).unwrap()
            .get(&TypeId::of::<T>()) {
                Some(_) => is_initialized = true,
                None => is_initialized = false,
            }
        if !is_initialized {
            Component::initialize_index_set::<T>(which, indices);
        }
    }

    fn initialize_index_set<T: Any + Send + Sync>(which: usize, indices: &ALIndices) {
        indices
            .write().unwrap()
            .get_mut(which).unwrap()
            .insert(TypeId::of::<T>(), RwLock::new(BTreeSet::new()));
    }
}

#[test]
fn test_multi_thread_get() {
    let (storage, indices) = Component::new();
    let (tx, rx) = channel::<u64>();
    let loc = Component::insert::<u64>(372u64, &storage, &indices).unwrap();
    for i in 0..5 {
        let tx = tx.clone();
        let loc = loc.clone();
        let storage = storage.clone();
        thread::spawn(move|| {
            tx.send(Component::get::<u64>(loc, &storage).unwrap()).unwrap();
        });
    }
    for _ in 0..5 {
        let value = rx.recv().unwrap();
        assert!(value == 372u64);
    }
}

#[test]
fn test_multi_thread_set() {
    let (storage, indices) = Component::new();
    let location = Component::insert::<u64>(372u64, &storage, &indices).unwrap();
    let gate = Arc::new(Barrier::new(5));
    let mut handles = Vec::with_capacity(5);

    for set_value in 0..5 {
        let location = location.clone();
        let storage = storage.clone();
        let set_value = set_value.clone();
        let gate = gate.clone();

        handles.push(thread::spawn(move|| {
            gate.wait();
            Component::set::<u64>(location, set_value, &storage);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let v = Component::get::<u64>(location, &storage).unwrap();
    assert!(v == 4 || v == 3 || v == 2 || v == 1 || v == 0); //will change depending on thread execution order due to the thread gate.
}

#[test]
fn test_expected_indices_on_insert() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices).unwrap();
    assert!(i == 0);
}

#[test]
fn test_reusing_indices() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices).unwrap();
    Component::empty::<u64>(i, &storage, &indices);
    let j = Component::insert::<u64>(23u64, &storage, &indices).unwrap();
    assert!(i == j, "i: {}, j: {}", i, j);
}

#[test]
fn test_insert() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices).unwrap();
}

#[test]
fn test_is_empty() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices).unwrap();
    assert!(!Component::is_empty::<u64>(i, &storage).unwrap());
    Component::empty::<u64>(i, &storage, &indices);
    assert!(Component::is_empty::<u64>(i, &storage).unwrap());
}

#[test]
#[should_panic]
fn test_get() {
    let (storage, indices) = Component::new();
    match Component::get::<u64>(0, &storage) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
#[should_panic]
fn test_empty() {
    let (storage, indices) = Component::new();
    match Component::empty::<u64>(0, &storage, &indices) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
#[should_panic]
fn test_set() {
    let (storage, indices) = Component::new();
    match Component::set::<u64>(0, 32u64, &storage) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
fn test_len() {
    let (storage, indices) = Component::new();
    for i in 0..50 {
        Component::insert::<u64>(i as u64, &storage, &indices);
    }
    assert!(Component::len::<u64>(&storage).unwrap() == 50);
}

#[test]
fn test_insert_get() {
    let (storage, indices) = Component::new();
    let input: u64 = 32;
    let i = Component::insert::<u64>(input, &storage, &indices).unwrap();
    let output = Component::get::<u64>(i, &storage).unwrap();
    assert!(input == output, "input: {}, output: {:#?}", input, output);
}

#[test]
fn test_insert_multiple_get_multiple() {
    let (storage, indices) = Component::new();
    for i in 0..100 {
        Component::insert::<usize>(i, &storage, &indices);
    }
    for i in 0..100 {
        let output = Component::get::<usize>(i, &storage).unwrap();
        assert!(i == output, "i: {}, output: {:#?}", i, output);
    }
}

#[test]
fn test_insert_different_get_different() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices).unwrap();
    let i2 = Component::insert::<bool>(true, &storage, &indices).unwrap();
    assert!(Component::get::<bool>(i2, &storage).unwrap());
    assert!(Component::get::<u64>(i, &storage).unwrap() == 32)
}

#[test]
fn test_insert_empty() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(0, &storage, &indices).unwrap();
    Component::empty::<u64>(i, &storage, &indices);
}

#[test]
fn test_insert_remove_insert() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices).unwrap();
    match Component::empty::<u64>(i, &storage, &indices) {
        Ok(_) => {
            let i = Component::insert::<u64>(31u64, &storage, &indices).unwrap();
            assert!(Component::get::<u64>(i, &storage).unwrap() == 31u64);
        },
        Err(_) => panic!()
    }
}

#[test]
fn test_insert_set() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices).unwrap();
    match Component::set::<u64>(i, 100u64, &storage) {
        Ok(_) => assert!(Component::get::<u64>(i, &storage).unwrap() == 100u64),
        Err(_) => panic!()
    }
}

#[test]
fn test_insert_empty_insert_set() {
    let (storage, indices) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices).unwrap();
    match Component::empty::<u64>(i, &storage, &indices) {
        Ok(_) => {
            let i = Component::insert::<u64>(31u64, &storage, &indices).unwrap();
            match Component::set::<u64>(i, 30u64, &storage) {
                Ok(_) => assert!(Component::get::<u64>(i, &storage).unwrap() == 30u64),
                Err(_) => panic!()
            }
        },
        Err(_) => panic!()
    }
}
