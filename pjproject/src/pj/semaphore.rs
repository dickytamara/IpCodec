
use pj_sys::*;
use std::ffi::CString;

use crate::utils::check_status;

use super::PJPool;



pub struct PJSem { pub ctx: Box<*mut pj_sem_t> }

impl From<Box<*mut pj_sem_t>> for PJSem {
    fn from(ptr: Box<*mut pj_sem_t>) -> Self {
        Self { ctx: ptr }
    }
}

impl PJSem {

    // pj_status_t 	pj_sem_create (pj_pool_t *pool, const char *name, unsigned initial, unsigned max, pj_sem_t **sem)
    pub fn create(pool: &PJPool, name: String, initial: u32, max: u32) -> Result<PJSem, i32> {
        unsafe {
            let sem = Box::new(std::ptr::null_mut());
            let name = CString::new(name.as_str()).unwrap().to_owned().as_ptr();
            let status = check_status(pj_sem_create(*pool.ctx, name, initial, max, *sem as *mut _));

            match status {
                Ok(()) => return Ok(Self::from(sem)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_sem_wait (pj_sem_t *sem)
    pub fn wait(&self) -> Result<(), i32> {
        unsafe { check_status(pj_sem_wait(*self.ctx)) }
    }

    // pj_status_t 	pj_sem_trywait (pj_sem_t *sem)
    pub fn trywait(&self) -> Result<(), i32> {
        unsafe { check_status(pj_sem_trywait(*self.ctx)) }
    }

    // pj_status_t 	pj_sem_post (pj_sem_t *sem)
    pub fn post(&self) -> Result<(), i32> {
        unsafe { check_status(pj_sem_post(*self.ctx)) }
    }

    // pj_status_t 	pj_sem_destroy (pj_sem_t *sem)
    pub fn destroy(self) -> Result<(), i32> {
        unsafe { check_status(pj_sem_destroy(*self.ctx)) }
    }

}