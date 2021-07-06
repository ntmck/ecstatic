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

    //for each component in storage
        //let i = active component index
        //let j = free component index
        //let e = owner of component[i]

        //get active component index: i
        //get entity: e of c[i] //This funcitonality is required at the moment...
        //get free index: j
        //swap c[i] and c[j] to move active index as far left in the vector as possible.
        //set cownership[e[type]] = j to update entity's owned component index.

    //after loop:
        //truncate all vector data after len+1
        //ensure that free_q is empty.
        //update next_free to be len+1 of compressed vector.

    pub fn compress<T: Any>(&self, cmanager: &mut CManager, cownership: &mut COwnership) {
        let eids = cownership.get_entity_ids();
        for
    }
}
