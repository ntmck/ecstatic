use std::any::{Any, TypeId};
use crate::component::{CManager, COwnership};
use crate::entity::Entity;

//Memory management for component storage.
//Memory MUST know which entity owns each index. Try to use COwnership to iterate?
pub struct Memory {}

impl Memory {
    pub fn new() -> Memory {
        Memory {}
    }

    /*pub fn compress(&self, cmanager: &mut CManager, cownership: &mut COwnership) {
        //i need a way of knowing that a component index belongs to an entity at packed component level.

        //maybe edit packed so that it is a Vec of struct(usize, entity.id)? (change packed to Vec!)
            //Side note: probably change Free's tuple to a named struct for simplicity...
        //that way we iterate through cmanager.packed, where i = packed index
        //move storage[i] to cmanager.free.pop_front
        //change current packed index to next free using popped free index,
        //use entity.id@type to update cownership's index to popped free index.
        //repeat until all moved to free space, then truncate all extra free space.
        //update next_free to storage.len()? +1? and ensure that the freed VecDeque is empty.

        //foreach type we swap a packed index for a freed index and update the packed index to free.

        let mut type_ids: Vec<TypeId> = vec![];
        for (type_id, _) in cmanager.components.storage.iter() {
            type_ids.push(*type_id);
        }

        for type_id in type_ids.iter() {
            let free_index = cmanager.find_available_free_index_by_id(type_id);
            if let Some(packed_index) =
        }
    }*/
}
