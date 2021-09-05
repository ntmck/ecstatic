use std::any::{Any, TypeId};
use std::any::type_name;
use std::marker::{Send, Sync};
use std::collections::{HashMap, BTreeSet};
use std::sync::{Arc, RwLock};
use std::panic;

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

pub type ALLengths = Arc<RwLock<HashMap<TypeId, RwLock<usize>>>>;

//Required to downcast_ref after obtaining the lock.
pub type Modify<T> = fn(&mut T);

//An empty type for emptying component memory.
enum Empty { Empty }

pub struct Component;
impl Component {
    const STORAGE_LOCK_ERROR_MSG: &'static str = "Failed to acquire storage lock.";
    const TYPE_NOT_FOUND_ERROR_MSG: &'static str = "Failed to find component of type:";
    const VECTOR_LOCK_ERROR_MSG: &'static str = "Failed to acquire vector lock.";
    const INDEX_OUT_OF_BOUNDS_ERROR_MSG: &'static str = "Index out of bounds for index and type ->";
    const ELEMENT_LOCK_ERROR_MSG: &'static str = "Failed to acquire element lock.";
    const DOWNCAST_ERROR_MSG: &'static str = "Failed to downcast value for type ->";
    const LEN_NOT_FOUND_ERROR_MSG: &'static str = "Failed to find length corresponding to type ->";

    pub fn new() -> (ALComponentStorage, ALIndices, ALLengths) {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        let indices = Arc::new(RwLock::new([HashMap::new(), HashMap::new()]));
        let lengths = Arc::new(RwLock::new(HashMap::new()));
        (storage, indices, lengths)
    }

    //inserts a component into storage by type and returns the index it was inserted into.
    pub fn insert<T>(component: T, storage: &ALComponentStorage, indices: &ALIndices, lengths: &ALLengths) -> Result<usize, ErrEcs>
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        Component::check_initialized_component_vector::<T>(storage);
        Component::check_initialized_lengths::<T>(lengths);
        let len = Component::len::<T>(lengths).unwrap();
        let i = Component::get_index::<T>(indices, len);
        //if i > len { Component::allocate::<T>(i - len, storage); } //NOTE: This may be necessary later.
        if let Err(e) = panic::catch_unwind(|| {
            storage
                .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::TYPE_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                .write().expect(Component::VECTOR_LOCK_ERROR_MSG)
                .insert(i, RwLock::new(Box::new(component)));

            //add 1 to length.
            match Component::change_len::<T>(1, true, lengths) {
                Ok(_) => (),
                Err(e) => panic!("{:#?}", e),
            }
        }) {
            Component::free_index::<T>(i, indices);
            return Err(ErrEcs::ComponentInsert(format!("{:#?}", e)))
        }
        Ok(i)
    }

    //replaces a component at the given index with the given component.
    pub fn set<T>(i: usize, component: T, storage: &ALComponentStorage) -> Result<(), ErrEcs>
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        Component::check_initialized_component_vector::<T>(storage);
        if let Err(e) = panic::catch_unwind(|| {
            *storage
                .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::TYPE_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                .read().expect(Component::VECTOR_LOCK_ERROR_MSG)
                .get(i).expect(&format!("{} index: {}, type: {}", Component::INDEX_OUT_OF_BOUNDS_ERROR_MSG, i, type_name::<T>()))
                .write().expect(Component::ELEMENT_LOCK_ERROR_MSG) = Box::new(component);
        }) {
            return Err(ErrEcs::ComponentSet(format!("{:#?}", e)))
        }
        Ok(())
    }

    //modifies the component using the provided function.
    pub fn modify<T>(i: usize, storage: &ALComponentStorage, modify: Modify<T>) -> Result<(), ErrEcs>
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        Component::check_initialized_component_vector::<T>(storage);
        if !Component::is_empty::<T>(i, storage)? {
            if let Err(e) = panic::catch_unwind(|| {
                match storage
                    .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                    .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::TYPE_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                    .read().expect(Component::VECTOR_LOCK_ERROR_MSG)
                    .get(i).expect(&format!("{} index: {}, type: {}", Component::INDEX_OUT_OF_BOUNDS_ERROR_MSG, i, type_name::<T>()))
                    .write() {
                        Ok(mut value) => modify(value.downcast_mut::<T>().expect(&format!("{} type: {}", Component::DOWNCAST_ERROR_MSG, type_name::<T>()))),
                        Err(e) => panic!("{}", Component::ELEMENT_LOCK_ERROR_MSG)
                    }
            }) {
                return Err(ErrEcs::ComponentModify(format!("{:#?}", e)))
            }
            Ok(())
        } else {
            return Err(ErrEcs::ComponentEmpty(format!("Component::modify {}", type_name::<T>())))
        }
    }

    //reads the component at given index.
    pub fn read<T>(i: usize, storage: &ALComponentStorage) -> Result<T, ErrEcs>
    where T: Any + Send + Sync + Copy + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        Component::check_initialized_component_vector::<T>(storage);
        if !Component::is_empty::<T>(i, storage)? {
            match panic::catch_unwind(|| -> T {
                *storage
                    .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                    .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::TYPE_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                    .read().expect(Component::VECTOR_LOCK_ERROR_MSG)
                    .get(i).expect(&format!("{} index: {}, type: {}", Component::INDEX_OUT_OF_BOUNDS_ERROR_MSG, i, type_name::<T>()))
                    .read().expect(Component::ELEMENT_LOCK_ERROR_MSG)
                    .downcast_ref::<T>().expect(&format!("{} type: {}", Component::DOWNCAST_ERROR_MSG, type_name::<T>()))

            }) {
                Ok(v) => Ok(v),
                Err(e) => Err(ErrEcs::ComponentRead(format!("{:#?}", e)))
            }
        } else {
            return Err(ErrEcs::ComponentEmpty(format!("Component::read {}", type_name::<T>())))
        }
    }

    //Empties, but doesn't deallocate, memory at index for a component type.
    pub fn empty<T>(i: usize, storage: &ALComponentStorage, indices: &ALIndices, lengths: &ALLengths) -> Result<(), ErrEcs>
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        Component::check_initialized_component_vector::<T>(storage);
        if let Err(e) = panic::catch_unwind(|| {
            match storage
                .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::TYPE_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                .read().expect(Component::VECTOR_LOCK_ERROR_MSG)
                .get(i).expect(&format!("{} index: {}, type: {}", Component::INDEX_OUT_OF_BOUNDS_ERROR_MSG, i, type_name::<T>()))
                .write() {
                    Ok(mut value) => {
                        *value = Box::new(Empty::Empty);

                        //sub 1 to length
                        match Component::change_len::<T>(1, false, lengths) {
                            Ok(_) => (),
                            Err(e) => panic!("{:#?}", e),
                        }
                    },
                    Err(e) => panic!("{}", Component::ELEMENT_LOCK_ERROR_MSG)
                }
        }) {
            return Err(ErrEcs::ComponentEmpty(format!("{:#?}", e)))
        }
        Component::free_index::<T>(i, indices);
        Ok(())
    }


    pub fn capacity<T>(storage: &ALComponentStorage) -> Result<usize, ErrEcs>
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        match panic::catch_unwind(|| -> usize {
            storage
                .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::TYPE_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                .read().expect(Component::VECTOR_LOCK_ERROR_MSG)
                .len() //capacity = len because emptying a component does not remove the allocation from the vector.

        }) {
            Ok(v) => Ok(v),
            Err(e) => Err(ErrEcs::ComponentRead(format!("{:#?}", e)))
        }
    }

    //returns the count of non-empty components of the underlying component vector by type.
    pub fn len<T>(lengths: &ALLengths) -> Result<usize, ErrEcs>
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        match panic::catch_unwind(|| -> usize {
            *lengths
                .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::LEN_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                .read().expect(&format!("{}", Component::ELEMENT_LOCK_ERROR_MSG))
        }) {
            Ok(v) => Ok(v),
            Err(e) => Err(ErrEcs::ComponentLen(format!("{:#?}", e)))
        }
    }

    fn change_len<T>(amount: usize, add: bool, lengths: &ALLengths) -> Result<(), ErrEcs>
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        match panic::catch_unwind(|| {
            match lengths
                .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::LEN_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                .write() {
                    Ok(mut v) => { if add { *v += amount; } else { *v -= amount; } },
                    Err(e) => panic!("{}", e),
                }
        }) {
            Ok(v) => Ok(()),
            Err(e) => Err(ErrEcs::ComponentLen(format!("{:#?}", e)))
        }
    }

    //checks if the value of a type is empty.
    fn is_empty<T>(i: usize, storage: &ALComponentStorage) -> Result<bool, ErrEcs>
    where T: Any + Send + Sync + std::panic::UnwindSafe + std::panic::RefUnwindSafe
    {
        Component::check_initialized_component_vector::<T>(storage);
        match panic::catch_unwind(|| -> bool {
            match storage
                .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
                .get(&TypeId::of::<T>()).expect(&format!("{} {}", Component::TYPE_NOT_FOUND_ERROR_MSG, type_name::<T>()))
                .read().expect(Component::VECTOR_LOCK_ERROR_MSG)
                .get(i).expect(&format!("{} index: {}, type: {}", Component::INDEX_OUT_OF_BOUNDS_ERROR_MSG, i, type_name::<T>()))
                .read().expect(Component::ELEMENT_LOCK_ERROR_MSG)
                .downcast_ref::<Empty>() {
                    Some(_) => return true,
                    None => return false,
                }
        }) {
            Ok(v) => Ok(v),
            Err(e) => Err(ErrEcs::ComponentEmpty(format!("Panic on is_empty: {:#?}", e)))
        }
    }

    //allocates space in the component vector.
    /*fn allocate<T: Any + Send + Sync>(to: usize, storage: &ALComponentStorage) {
        unimplemented!()
    }*/

    fn check_initialized_lengths<T: Any + Send + Sync>(lengths: &ALLengths) {
        let mut initialized = false;
        if let Some(_) = lengths
            .read().expect(Component::STORAGE_LOCK_ERROR_MSG)
            .get(&TypeId::of::<T>()) {
                initialized = true;
            }

        if !initialized {
            lengths
            .write().expect(Component::STORAGE_LOCK_ERROR_MSG)
            .insert(TypeId::of::<T>(), RwLock::new(0));
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
    let (storage, indices, lengths) = Component::new();
    let (tx, rx) = channel::<u64>();
    let loc = Component::insert::<u64>(372u64, &storage, &indices, &lengths).unwrap();
    for i in 0..5 {
        let tx = tx.clone();
        let loc = loc.clone();
        let storage = storage.clone();
        thread::spawn(move|| {
            tx.send(Component::read::<u64>(loc, &storage).unwrap()).unwrap();
        });
    }
    for _ in 0..5 {
        let value = rx.recv().unwrap();
        assert!(value == 372u64);
    }
}

#[test]
fn test_multi_thread_set() {
    let (storage, indices, lengths) = Component::new();
    let location = Component::insert::<u64>(372u64, &storage, &indices, &lengths).unwrap();
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

    let v = Component::read::<u64>(location, &storage).unwrap();
    assert!(v == 4 || v == 3 || v == 2 || v == 1 || v == 0); //will change depending on thread execution order due to the thread gate.
}

#[test]
fn test_expected_indices_on_insert() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices, &lengths).unwrap();
    assert!(i == 0);
}

#[test]
fn test_reusing_indices() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices, &lengths).unwrap();
    Component::empty::<u64>(i, &storage, &indices, &lengths);
    let j = Component::insert::<u64>(23u64, &storage, &indices, &lengths).unwrap();
    assert!(i == j, "i: {}, j: {}", i, j);
}

#[test]
fn test_insert() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices, &lengths).unwrap();
}

#[test]
fn test_is_empty() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices, &lengths).unwrap();
    assert!(!Component::is_empty::<u64>(i, &storage).unwrap());
    Component::empty::<u64>(i, &storage, &indices, &lengths);
    assert!(Component::is_empty::<u64>(i, &storage).unwrap());
}

#[test]
#[should_panic]
fn test_get() {
    let (storage, indices, lengths) = Component::new();
    match Component::read::<u64>(0, &storage) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
#[should_panic]
fn test_empty() {
    let (storage, indices, lengths) = Component::new();
    match Component::empty::<u64>(0, &storage, &indices, &lengths) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
#[should_panic]
fn test_set() {
    let (storage, indices, lengths) = Component::new();
    match Component::set::<u64>(0, 32u64, &storage) {
        Ok(_) => (),
        Err(_) => panic!()
    }
}

#[test]
fn test_len() {
    let (storage, indices, lengths) = Component::new();
    for i in 0..50 {
        Component::insert::<u64>(i as u64, &storage, &indices, &lengths);
    }
    assert!(Component::len::<u64>(&lengths).unwrap() == 50);
}

#[test]
fn test_insert_get() {
    let (storage, indices, lengths) = Component::new();
    let input: u64 = 32;
    let i = Component::insert::<u64>(input, &storage, &indices, &lengths).unwrap();
    let output = Component::read::<u64>(i, &storage).unwrap();
    assert!(input == output, "input: {}, output: {:#?}", input, output);
}

#[test]
fn test_insert_multiple_get_multiple() {
    let (storage, indices, lengths) = Component::new();
    for i in 0..100 {
        Component::insert::<usize>(i, &storage, &indices, &lengths);
    }
    for i in 0..100 {
        let output = Component::read::<usize>(i, &storage).unwrap();
        assert!(i == output, "i: {}, output: {:#?}", i, output);
    }
}

#[test]
fn test_insert_different_get_different() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices, &lengths).unwrap();
    let i2 = Component::insert::<bool>(true, &storage, &indices, &lengths).unwrap();
    assert!(Component::read::<bool>(i2, &storage).unwrap());
    assert!(Component::read::<u64>(i, &storage).unwrap() == 32)
}

#[test]
fn test_insert_empty() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(0, &storage, &indices, &lengths).unwrap();
    Component::empty::<u64>(i, &storage, &indices, &lengths);
}

#[test]
fn test_insert_remove_insert() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices, &lengths).unwrap();
    match Component::empty::<u64>(i, &storage, &indices, &lengths) {
        Ok(_) => {
            let i = Component::insert::<u64>(31u64, &storage, &indices, &lengths).unwrap();
            assert!(Component::read::<u64>(i, &storage).unwrap() == 31u64);
        },
        Err(_) => panic!()
    }
}

#[test]
fn test_insert_set() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices, &lengths).unwrap();
    match Component::set::<u64>(i, 100u64, &storage) {
        Ok(_) => assert!(Component::read::<u64>(i, &storage).unwrap() == 100u64),
        Err(_) => panic!()
    }
}

#[test]
fn test_insert_empty_insert_set() {
    let (storage, indices, lengths) = Component::new();
    let i = Component::insert::<u64>(32u64, &storage, &indices, &lengths).unwrap();
    match Component::empty::<u64>(i, &storage, &indices, &lengths) {
        Ok(_) => {
            let i = Component::insert::<u64>(31u64, &storage, &indices, &lengths).unwrap();
            match Component::set::<u64>(i, 30u64, &storage) {
                Ok(_) => assert!(Component::read::<u64>(i, &storage).unwrap() == 30u64),
                Err(_) => panic!()
            }
        },
        Err(_) => panic!()
    }
}
