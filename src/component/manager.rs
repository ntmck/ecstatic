use std::vec::Vec;
use std::any::{Any, TypeId};
use std::collections::{HashSet, HashMap, VecDeque};

pub struct Components {
    //component_mask(type of component)->components
    storage: HashMap<TypeId, Vec<Box<dyn Any>>>,
    //component_mask(type of component)->active_component_indices
    packed: HashMap<TypeId, HashSet<usize>>,
    //component_mask(type of component)->(next_free_index, freed_indices_for_type)
    free: HashMap<TypeId, (usize, VecDeque<usize>)>,
}

pub trait TComponentManager {
    //Component get
    fn cget<T: Any>(&self, i: usize) -> &T;

    //Component set
    fn cset<T: Any>(&mut self, i: usize, comp: T);

    //Component insert -- returns index it was inserted into.
    fn cinsert<T: Any>(&mut self, comp: T) -> usize;

    //Component remove
    fn cremove<T: Any>(&mut self, i: usize);
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
    //Packs a new index for a component in the packed array.
    fn pack<T: Any>(&mut self, i: usize) {
        loop {
            if let Some(set) = self.components.packed.get_mut(&TypeId::of::<T>()) {
                set.insert(i);
                break;
            } else {
                self.components.packed.insert(TypeId::of::<T>(), HashSet::new());
            }
        }
    }

    //Unpacks index from packed array for component.
    fn unpack<T: Any>(&mut self, i: usize) {
        if !self.components.packed.get_mut(&TypeId::of::<T>()).unwrap().remove(&i) {
            panic!("Error handling not implemented. Attempt to unpack non-existent element from packed.");
        }
    }

    //Frees an index for use later.
    fn free_index<T: Any>(&mut self, i: usize) {

    }

    //Returns an available index for insertion.
    fn find_available_index<T: Any>(&mut self) -> usize {
        let i;
        if let Some(nextAndVecdq) = self.components.free.get_mut(&TypeId::of::<T>()) {
            if let Some(dq) = nextAndVecdq.1.pop_front() {
                i = dq;
            } else {
                i = nextAndVecdq.0;
                nextAndVecdq.0 += 1;
            }
        } else {
            panic!("Error handling unimplemented.");
        }
        i
    }
}

impl TComponentManager for CManager {
    fn cget<T: Any>(&self, i: usize) -> &T {
        if let Some(vec) = self.components.storage.get(&TypeId::of::<T>()) {
            &*vec[i].downcast_ref::<T>().unwrap()
        } else {
            panic!("Error handling unimplemented.");
        }
    }

    fn cset<T: Any>(&mut self, i: usize, comp: T) {
        if let Some(vec) = self.components.storage.get_mut(&TypeId::of::<T>()) {
            vec[i] = Box::new(comp);
        } else {
            panic!("Error handling unimplemented.");
        }
    }

    fn cinsert<T: Any>(&mut self, comp: T) -> usize {
        let i = self.find_available_index::<T>();
        loop {
            if let Some(vec) = self.components.storage.get_mut(&TypeId::of::<T>()) {
                if i >= vec.capacity() {
                    vec.reserve((i - vec.capacity()) + 1);
                }
                vec.insert(i, Box::new(comp));
                break;
            } else {
                self.components.storage.insert(TypeId::of::<T>(), vec![]);
            }
        }
        self.pack::<T>(i);
        i
    }

    fn cremove<T: Any>(&mut self, i: usize) {
        self.unpack::<T>(i);
        self.free_index::<T>(i);
        if let Some(vec) = self.components.storage.get_mut(&TypeId::of::<T>()) {
            vec[i] = Box::new(-1);
        } else {
            panic!("Error handling unimplemented.");
        }
    }
}
