

use pj_sys::*;
use std::ffi::CString;

use crate::utils::{boolean_to_pjbool, check_status};

use super::PJPool;


pub struct PJEvent { pub ctx: Box<*mut pj_event_t> }

impl From<Box<*mut pj_event_t>> for PJEvent {
    fn from(ptr: Box<*mut pj_event_t>) -> Self {
        Self { ctx: ptr }
    }
}


impl PJEvent {

    // pj_status_t 	pj_event_create (pj_pool_t *pool, const char *name, pj_bool_t manual_reset, pj_bool_t initial, pj_event_t **event)
    pub fn create(pool: &PJPool, name: String, manual_reset: bool, initial: bool) -> Result<PJEvent, i32> {
        unsafe {
            let event = Box::new(std::ptr::null_mut());
            let name = CString::new(name.as_str()).unwrap().to_owned().as_ptr();
            let status = check_status(pj_event_create(
                *pool.ctx,
                name,
                boolean_to_pjbool(manual_reset),
                boolean_to_pjbool(initial),
                *event as *mut _
            ));

            match status {
                Ok(()) => return Ok(Self::from(event)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_event_wait (pj_event_t *event)
    pub fn wait(&self) -> Result<(), i32> {
        unsafe { check_status(pj_event_wait(*self.ctx)) }
    }

    // pj_status_t 	pj_event_trywait (pj_event_t *event)
    pub fn trywait(&self) -> Result<(), i32> {
        unsafe { check_status(pj_event_trywait(*self.ctx)) }
    }

    // pj_status_t 	pj_event_set (pj_event_t *event)
    pub fn set(&self) -> Result<(), i32> {
        unsafe { check_status(pj_event_set(*self.ctx)) }
    }

    // pj_status_t 	pj_event_pulse (pj_event_t *event)
    pub fn pulse(&self) -> Result<(), i32> {
        unsafe { check_status(pj_event_pulse(*self.ctx)) }
    }

    // pj_status_t 	pj_event_reset (pj_event_t *event)
    pub fn reset(&self) -> Result<(), i32> {
        unsafe { check_status(pj_event_pulse(*self.ctx)) }
    }

   // pj_status_t 	pj_event_destroy (pj_event_t *event)
    pub fn destroy(self) -> Result<(), i32> {
        unsafe { check_status(pj_event_destroy(*self.ctx)) }
    }
}