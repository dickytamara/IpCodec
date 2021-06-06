
use pj_sys::*;

use crate::utils::check_status;

use super::PJPool;

pub struct PJAtomic { pub ctx: Box<*mut pj_atomic_t> }

impl From<Box<*mut pj_atomic_t>> for PJAtomic {
    fn from(ptr: Box<*mut pj_atomic_t>) -> Self {
        Self { ctx: ptr }
    }
}


impl PJAtomic {

    // pj_status_t 	pj_atomic_create (pj_pool_t *pool, pj_atomic_value_t initial, pj_atomic_t **atomic)
    pub fn create(pool: &PJPool, initial: i64) -> Result<Self, i32> {
        unsafe {
            let atomic = Box::new(std::ptr::null_mut());
            let status = check_status(pj_atomic_create(*pool.ctx, initial, *atomic as *mut _));

            match status {
                Ok(()) => return Ok(PJAtomic::from(atomic)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_atomic_destroy (pj_atomic_t *atomic_var)
    pub fn destroy(&self) -> Result<(), i32> {
        unsafe { check_status(pj_atomic_destroy(*self.ctx)) }
    }

    // void 	pj_atomic_set (pj_atomic_t *atomic_var, pj_atomic_value_t value)
    pub fn set(&self, value: i64) {
        unsafe { pj_atomic_set(*self.ctx, value) }
    }

    // pj_atomic_value_t 	pj_atomic_get (pj_atomic_t *atomic_var)
    pub fn get(&self) -> i64 {
        unsafe { pj_atomic_get(*self.ctx) }
    }

    // void 	pj_atomic_inc (pj_atomic_t *atomic_var)
    pub fn inc(&self) {
        unsafe { pj_atomic_inc(*self.ctx) }
    }

    // pj_atomic_value_t 	pj_atomic_inc_and_get (pj_atomic_t *atomic_var)
    pub fn inc_and_get(&self) -> i64 {
        unsafe { pj_atomic_inc_and_get(*self.ctx) }
    }

    // void 	pj_atomic_dec (pj_atomic_t *atomic_var)
    pub fn dec(&self) {
        unsafe { pj_atomic_dec(*self.ctx) }
    }

    // pj_atomic_value_t 	pj_atomic_dec_and_get (pj_atomic_t *atomic_var)
    pub fn dec_and_get(&self) -> i64 {
        unsafe { pj_atomic_dec_and_get(*self.ctx) }
    }

    // void 	pj_atomic_add (pj_atomic_t *atomic_var, pj_atomic_value_t value)
    pub fn add(&self, value: i64) {
        unsafe { pj_atomic_add(*self.ctx, value) }
    }

    // pj_atomic_value_t 	pj_atomic_add_and_get (pj_atomic_t *atomic_var, pj_atomic_value_t value)
    pub fn add_and_get(&self, value: i64) -> i64 {
        unsafe { pj_atomic_add_and_get(*self.ctx, value) }
    }
}