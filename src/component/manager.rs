use std::vec::Vec;
use std::any::{Any, TypeId};
use std::collections::{HashSet, HashMap, VecDeque};

pub struct Components {
    //component_mask(type of component)->components
    storage: HashMap<TypeId, Vec<Box<dyn Any>>>,
    //component_mask(type of component)->active_component_indices
    packed: HashMap<TypeId, Vec<usize>>,
    //component_mask(type of component)->(next_free_index, freed_indices_for_type)
    free: HashMap<TypeId, (u64, VecDeque<usize>)>,
}

pub trait TComponentManager {
    //Component get
    fn cget<T: Any>(&self, i: usize) -> T;

    //Component set
    fn cset<T: Any>(&mut self, i: usize, comp: T);

    //Component insert -- returns index it was inserted into.
    fn cinsert<T: Any>(&mut self, comp: T) -> usize;

    //Component remove -- returns component that was removed.
    fn cremove<T: Any>(&mut self, i: usize) -> T;
}

pub struct CManager {
    components: Components,
}

impl CManager {
    pub fn new() -> CManager {
        CManager {
            components: Components {
                storage: HashMap::new(),
                packed: HashMap::new(),
                free: HashMap::new(),
            }
        }
    }
}

impl TComponentManager for CManager {
    fn cget<T: Any>(&self, i: usize) -> T {
        if let Some(vec) = self.components.storage.get(&TypeId::of::<T>()) {
            *vec[i].downcast_ref::<T>().unwrap()
        } else {
            panic!("Error handling unimplemented.");
        }
    }

    fn cset<T: Any>(&mut self, i: usize, comp: T) {

    }

    fn cinsert<T: Any>(&mut self, comp: T) -> usize {

    }

    fn cremove<T: Any>(&mut self, i: usize) -> T {

    }
}
