use std::vec::Vec;
use std::any::{Any, TypeId};
use std::any::type_name;
use std::collections::{HashSet, HashMap, VecDeque};
use crate::ErrEcs;

pub struct Components {
    //component_mask(type of component)->components
    storage: HashMap<TypeId, Vec<Box<dyn Any>>>,
    //component_mask(type of component)->active_component_indices
    packed: HashMap<TypeId, HashSet<usize>>,
    //component_mask(type of component)->(next_free_index, freed_indices_for_type)
    free: HashMap<TypeId, (usize, VecDeque<usize>)>,
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

    pub fn cget<T: Any>(&self, i: usize) -> Result<&T, ErrEcs> {
        if let Some(vec) = self.components.storage.get(&TypeId::of::<T>()) {
            if let Some(cmp) = vec[i].downcast_ref::<T>() {
                Ok(cmp)
            } else { Err(ErrEcs::CManagerComponentNotFound(format!("cget type: {} index: {}", type_name::<T>(), i))) }
        } else { Err(ErrEcs::CManagerComponentTypeNotFound(format!("cget type: {}", type_name::<T>()))) }
    }

    pub fn cset<T: Any>(&mut self, i: usize, comp: T) -> Result<(), ErrEcs>{
        if let Some(vec) = self.components.storage.get_mut(&TypeId::of::<T>()) {
            vec[i] = Box::new(comp);
            Ok(())
        } else { Err(ErrEcs::CManagerComponentTypeNotFound(format!("cset type: {}", type_name::<T>()))) }
    }

    pub fn cinsert<T: Any>(&mut self, comp: T) -> usize {
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

    pub fn cremove<T: Any>(&mut self, i: usize) -> Result<(), ErrEcs> {
        self.cremove_by_id(TypeId::of::<T>(), i)
    }

    pub fn cremove_by_id(&mut self, key: TypeId, i: usize) -> Result<(), ErrEcs> {
        self.unpack_by_id(key, i)?;
        self.free_index_by_id(key, i);
        if let Some(vec) = self.components.storage.get_mut(&key) {
            vec[i] = Box::new(0u8);
            Ok(())
        } else { Err(ErrEcs::CManagerComponentTypeNotFound(format!("cremove type_id: {:#?}", key))) }
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
    fn unpack<T: Any>(&mut self, i: usize) -> Result<(), ErrEcs> {
        self.unpack_by_id(TypeId::of::<T>(), i)
    }

    fn unpack_by_id(&mut self, key: TypeId, i: usize) -> Result<(), ErrEcs> {
        if !self.components.packed.get_mut(&key).unwrap().remove(&i) {
            return Err(ErrEcs::CManagerUnpackIndexNotFound(
                format!("unpack attempt to unpack non-existent element from packed. index: {}", i)
            ))
        }
        Ok(())
    }

    //Inserts a freed an index for use later.
    fn free_index<T: Any>(&mut self, i: usize) {
        self.free_index_by_id(TypeId::of::<T>(), i);
    }

    fn free_index_by_id(&mut self, key: TypeId, i: usize) {
        loop {
            if let Some(next_and_vecdq) = self.components.free.get_mut(&key) {
                next_and_vecdq.1.push_back(i);
                break;
            } else {
                self.components.free.insert(key, (0, VecDeque::new()));
            }
        }
    }

    //Returns an available index for insertion.
    fn find_available_index<T: Any>(&mut self) -> usize {
        let i;
        loop {
            if let Some(next_and_vecdq) = self.components.free.get_mut(&TypeId::of::<T>()) {
                if let Some(dq) = next_and_vecdq.1.pop_front() {
                    i = dq;
                } else {
                    i = next_and_vecdq.0;
                    next_and_vecdq.0 += 1;
                }
                break;
            } else {
                self.components.free.insert(TypeId::of::<T>(), (0, VecDeque::new()));
            }
        }
        i
    }
}
