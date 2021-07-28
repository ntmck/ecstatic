use std::any::{Any, TypeId};

use crate::component::{CManager, COwnership};
use crate::ErrEcs;

pub struct Memory;

impl Memory {
    pub fn compress<T>(cmanager: &mut CManager, cownership: &mut COwnership) -> Result<(), ErrEcs>
    where T: Any + Send + Sync {
        crossbeam::scope(|s| -> Result<(), ErrEcs> {
            let t = s.spawn(|_| -> Result<(), ErrEcs> {
                let index_owned_by_entity_map = cownership.get_index_entity_map::<T>();
                let mut keys_len = 0;
                for i in index_owned_by_entity_map.keys() {
                    let j = cmanager.unsafe_swap_with_free::<T>(*i);
                    let eid = index_owned_by_entity_map.get(&i).unwrap();
                    cownership.update_index_by_id(*eid, &TypeId::of::<T>(), j)?;
                    cmanager.unpack::<T>(*i)?;
                    cmanager.pack::<T>(j);
                    keys_len += 1;
                }
                cmanager.reset_free::<T>(keys_len + 1)?;
                cmanager.cresize::<T>(keys_len + 1)?;
                Ok(())
            });
            t.join().unwrap()
        }).unwrap()
    }
}
