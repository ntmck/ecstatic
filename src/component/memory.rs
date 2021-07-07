use std::any::{Any, TypeId};

use crate::component::{CManager, COwnership};
use crate::entity::Entity;
use crate::ErrEcs;

//Memory management for component storage.
//Memory MUST know which entity owns each index. Try to use COwnership to iterate?
pub struct Memory {}

impl Memory {
    pub fn new() -> Memory {
        Memory {}
    }

    pub fn compress<T: Any>(&self, cmanager: &mut CManager, cownership: &mut COwnership) -> Result<(), ErrEcs> {
        //map of *cloned* index->entity_id for ownership reference.
        let index_owned_by_entity_map = cownership.get_index_entity_map::<T>();
        for i in index_owned_by_entity_map.keys() {
            let j = cmanager.unsafe_swap_with_free::<T>(*i);
            let eid = index_owned_by_entity_map.get(&i).unwrap();
            cownership.update_index_by_id(*eid, &TypeId::of::<T>(), j);
        }

        let new_len = cmanager.len::<T>();
        print!("newlen: {}, size of map: {}, plen: {}, pcap: {}\n", new_len, index_owned_by_entity_map.len(), cmanager.plen::<T>(), cmanager.pcapacity::<T>());
        let new_len = cmanager.reset_free::<T>()?;
        cmanager.cresize::<T>(new_len)?;
        Ok(())
    }
}
