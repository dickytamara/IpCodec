
use std::ffi::c_void;

use pj_sys::*;

use crate::{prelude::AutoCreate, utils::{boolean_to_pjbool, check_boolean, check_status}};

use super::PJPool;

pub struct PJTimerHeap { pub ctx: Box<*mut pj_timer_heap_t> }
pub struct PJTimerEntry{ pub ctx: Box<*mut pj_timer_entry> }

impl From<Box<*mut pj_timer_heap_t>> for PJTimerHeap {
    fn from(ptr: Box<*mut pj_timer_heap_t>) -> Self {
        Self { ctx: ptr }
    }
}

impl From<Box<*mut pj_timer_entry>> for PJTimerEntry {
    fn from (ptr: Box<*mut pj_timer_entry>) -> Self {
        Self { ctx: ptr }
    }
}

impl PJTimerHeap {

    // pj_size_t 	pj_timer_heap_mem_size (pj_size_t count)
    pub fn mem_size(count: u64) -> u64 {
        unsafe { pj_timer_heap_mem_size(count) }
    }

    // pj_status_t 	pj_timer_heap_create (pj_pool_t *pool, pj_size_t count, pj_timer_heap_t **ht)
    pub fn create(pool: &PJPool, count: u64) -> Result<Self, i32> {
        unsafe {
            let heap: Box<*mut pj_timer_heap_t> = Box::new(std::ptr::null_mut());
            let status = check_status(pj_timer_heap_create(*pool.ctx, count, *heap as *mut _));

            match status {
                Ok(()) => return Ok(PJTimerHeap::from(heap)),
                Err(e) => return Err(e),
            }
        }
    }

    // void 	pj_timer_heap_destroy (pj_timer_heap_t *ht)
    pub fn destroy(self) {
        unsafe { pj_timer_heap_destroy(*self.ctx ) }
    }

    // void 	pj_timer_heap_set_lock (pj_timer_heap_t *ht, pj_lock_t *lock, pj_bool_t auto_del)
    pub fn set_lock(&self, lock: &mut Box<*mut pj_lock_t>, auto_del: bool) {
        unsafe {
            pj_timer_heap_set_lock(*self.ctx, **lock, boolean_to_pjbool(auto_del));
        }
    }

    // unsigned 	pj_timer_heap_set_max_timed_out_per_poll (pj_timer_heap_t *ht, unsigned count)
    pub fn set_max_timed_out_per_poll(&self, count: u32) -> u32 {
        unsafe { pj_timer_heap_set_max_timed_out_per_poll(*self.ctx, count) }
    }

    // pj_status_t 	pj_timer_heap_schedule (pj_timer_heap_t *ht, pj_timer_entry *entry, const pj_time_val *delay)
    pub fn schedule(&self, entry: &mut Box<*mut pj_timer_entry>, delay: &mut Box<*const pj_time_val>) -> Result<(), i32> {
        unsafe {
            check_status(pj_timer_heap_schedule_dbg(*self.ctx, **entry, **delay, std::ptr::null_mut(), 0))
        }
    }

    // pj_status_t 	pj_timer_heap_schedule_w_grp_lock (pj_timer_heap_t *ht, pj_timer_entry *entry, const pj_time_val *delay, int id_val, pj_grp_lock_t *grp_lock)

    // int 	pj_timer_heap_cancel (pj_timer_heap_t *ht, pj_timer_entry *entry)
    pub fn cancel(&self, entry: &mut Box<*mut pj_timer_entry>) -> i32 {
        unsafe { pj_timer_heap_cancel(*self.ctx, **entry) }
    }

    // int 	pj_timer_heap_cancel_if_active (pj_timer_heap_t *ht, pj_timer_entry *entry, int id_val)
    pub fn cancel_if_active(&self, entry: &mut Box<*mut pj_timer_entry>, id_val: i32) -> i32 {
        unsafe { pj_timer_heap_cancel_if_active(*self.ctx, **entry, id_val) }
    }

    // pj_size_t 	pj_timer_heap_count (pj_timer_heap_t *ht)
    pub fn count(&self) -> u64 {
        unsafe { pj_timer_heap_count(*self.ctx) }
    }

    // pj_status_t 	pj_timer_heap_earliest_time (pj_timer_heap_t *ht, pj_time_val *timeval)
    pub fn earliest_time(&self, timeval: &mut pj_time_val) -> Result<(), i32> {
        unsafe { check_status(pj_timer_heap_earliest_time(*self.ctx, timeval as *mut _)) }
    }

    // unsigned 	pj_timer_heap_poll (pj_timer_heap_t *ht, pj_time_val *next_delay)
    pub fn poll(&self, next_delay: &mut pj_time_val) -> u32 {
        unsafe { pj_timer_heap_poll(*self.ctx, next_delay as *mut _) }
    }

}

impl PJTimerEntry {

    // pj_timer_entry * 	pj_timer_entry_init (pj_timer_entry *entry, int id, void *user_data, pj_timer_heap_callback *cb)
    pub fn init(&self, id: i32, user_data: &mut Box<*mut c_void>, cb: pj_timer_heap_callback) -> Self {
        unsafe {
            let mut entry = pj_timer_entry::new();

            pj_timer_entry_init(
                &mut entry as *mut _,
                id, **user_data, cb
            );

            PJTimerEntry::from(Box::new(&mut entry as *mut _))
        }
    }

    // pj_bool_t 	pj_timer_entry_running (pj_timer_entry *entry)
    pub fn running(&self) -> bool {
        unsafe { check_boolean(pj_timer_entry_running(*self.ctx)) }
    }
}