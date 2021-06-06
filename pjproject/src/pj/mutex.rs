
use pj_sys::*;

use crate::utils::check_status;

use super::PJPool;
use std::ffi::CString;

pub struct PJMutex { pub ctx: Box<*mut pj_mutex_t> }
pub struct PJRwMutex { pub ctx: Box<*mut pj_rwmutex_t> }

impl From<Box<*mut pj_mutex_t>> for PJMutex {
    fn from(ptr: Box<*mut pj_mutex_t>) -> Self {
        Self { ctx: ptr }
    }
}

impl From<Box<*mut pj_rwmutex_t>> for PJRwMutex {
    fn from(ptr: Box<*mut pj_rwmutex_t>) -> Self {
        Self { ctx: ptr }
    }
}



impl PJMutex {

    // pj_status_t 	pj_mutex_create (pj_pool_t *pool, const char *name, int type, pj_mutex_t **mutex)
    pub fn create(pool: &PJPool, name: String, type_: i32) -> Result<PJMutex, i32> {
        unsafe {
            let mutex = Box::new(std::ptr::null_mut());
            let name = CString::new(name.as_str()).unwrap().to_owned().as_ptr();
            let status = check_status(pj_mutex_create(*pool.ctx, name, type_, *mutex as *mut _));

            match status {
                Ok(()) => return Ok(PJMutex::from(mutex)),
                Err(e) => return Err(e),
            }
         }
    }

    // pj_status_t 	pj_mutex_create_simple (pj_pool_t *pool, const char *name, pj_mutex_t **mutex)
    pub fn create_simple(pool: &PJPool, name: String) -> Result<PJMutex, i32> {
        unsafe {
            let mutex = Box::new(std::ptr::null_mut());
            let name = CString::new(name.as_str()).unwrap().to_owned().as_ptr();
            let status = check_status(pj_mutex_create_simple(*pool.ctx, name, *mutex as *mut _));

            match status {
                Ok(()) => return Ok(PJMutex::from(mutex)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_mutex_create_recursive (pj_pool_t *pool, const char *name, pj_mutex_t **mutex)
    pub fn create_recursive(pool: &PJPool, name: String) -> Result<PJMutex, i32> {
        unsafe {
            let mutex = Box::new(std::ptr::null_mut());
            let name = CString::new(name.as_str()).unwrap().to_owned().as_ptr();
            let status = check_status(pj_mutex_create_recursive(*pool.ctx, name, *mutex as *mut _));

            match status {
                Ok(()) => return Ok(PJMutex::from(mutex)),
                Err(e) => return Err(e)
            }
        }
    }

    // pj_status_t 	pj_mutex_lock (pj_mutex_t *mutex)
    pub fn lock(&self) -> Result<(), i32> {
        unsafe { check_status(pj_mutex_lock(*self.ctx)) }
    }

    // pj_status_t 	pj_mutex_unlock (pj_mutex_t *mutex)
    pub fn unlock(&self) -> Result<(), i32> {
        unsafe { check_status(pj_mutex_unlock(*self.ctx)) }
    }

    // pj_status_t 	pj_mutex_trylock (pj_mutex_t *mutex)
    pub fn trylock(&self) -> Result<(), i32> {
        unsafe { check_status(pj_mutex_trylock(*self.ctx)) }
    }

    // pj_status_t 	pj_mutex_destroy (pj_mutex_t *mutex)
    pub fn destroy(self) -> Result<(), i32> {
        unsafe { check_status(pj_mutex_destroy(*self.ctx)) }
    }

    // pj_bool_t 	pj_mutex_is_locked (pj_mutex_t *mutex)
    pub fn is_locked(&self) -> Result<(), i32> {
        unsafe { check_status(pj_mutex_is_locked(*self.ctx)) }
    }

}

impl PJRwMutex {

    // pj_status_t 	pj_rwmutex_create (pj_pool_t *pool, const char *name, pj_rwmutex_t **mutex)
    pub fn create(pool: &PJPool, name: String) -> Result<PJRwMutex, i32> {
        unsafe {
            let mutex = Box::new(std::ptr::null_mut());
            let name = CString::new(name.as_str()).unwrap().to_owned().as_ptr();
            let status = check_status(pj_rwmutex_create(*pool.ctx, name, *mutex as *mut _));

            match status {
                Ok(()) => return Ok(PJRwMutex::from(mutex)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_rwmutex_lock_read (pj_rwmutex_t *mutex)
    pub fn lock_read(&self) -> Result<(), i32> {
        unsafe { check_status(pj_rwmutex_lock_read(*self.ctx)) }
    }

    // pj_status_t 	pj_rwmutex_lock_write (pj_rwmutex_t *mutex)
    pub fn locak_write(&self) -> Result<(), i32> {
        unsafe { check_status(pj_rwmutex_lock_write(*self.ctx)) }
    }

    // pj_status_t 	pj_rwmutex_unlock_read (pj_rwmutex_t *mutex)
    pub fn unlock_read(&self) -> Result<(), i32> {
        unsafe { check_status(pj_rwmutex_unlock_read(*self.ctx)) }
    }

    // pj_status_t 	pj_rwmutex_unlock_write (pj_rwmutex_t *mutex)
    pub fn unlock_write(&self) -> Result<(), i32> {
        unsafe { check_status(pj_rwmutex_unlock_write(*self.ctx)) }
    }

    // pj_status_t 	pj_rwmutex_destroy (pj_rwmutex_t *mutex)
    pub fn destroy(self) -> Result<(), i32> {
        unsafe { check_status(pj_rwmutex_destroy(*self.ctx)) }
    }
}

