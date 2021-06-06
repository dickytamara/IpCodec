
use pj_sys::*;
use std::os::raw::c_void;

use crate::prelude::AutoCreate;
use crate::prelude::AutoDefault;
use crate::utils::check_status;

use super::PJPool;


pub struct PJGrpLockConfig { pub ctx: Box<pj_grp_lock_config> }

impl AutoDefault<PJGrpLockConfig> for PJGrpLockConfig {
    // void 	pj_grp_lock_config_default (pj_grp_lock_config *cfg)
    fn default() -> PJGrpLockConfig {
        unsafe {
            let mut ret = Box::new(pj_grp_lock_config::new());

            pj_grp_lock_config_default(ret.as_mut() as *mut _);

            PJGrpLockConfig::from(ret)
        }
    }
}

impl From<Box<pj_grp_lock_config>> for PJGrpLockConfig {
    fn from (ptr: Box<pj_grp_lock_config>) -> Self {
        Self { ctx: ptr }
    }
 }

pub struct PJGrpLock { pub ctx: Box<*mut pj_grp_lock_t> }

impl From<Box<*mut pj_grp_lock_t>> for PJGrpLock {
    fn from (ptr: Box<*mut pj_grp_lock_t>) -> Self {
        Self { ctx: ptr }
    }
}



impl PJGrpLock {


    // pj_status_t 	pj_grp_lock_create (pj_pool_t *pool, const pj_grp_lock_config *cfg, pj_grp_lock_t **p_grp_lock)
    pub fn create(pool: &PJPool, cfg: &mut PJGrpLockConfig) -> Result<PJGrpLock, i32> {
        unsafe {
            let grplock = Box::new(std::ptr::null_mut());
            let status = check_status(pj_grp_lock_create(*pool.ctx, cfg.ctx.as_mut() as *const _, *grplock as *mut _));

            match status {
                Ok(()) => return Ok(PJGrpLock::from(grplock)),
                Err(e) => return Err(e),
            }

        }
    }

    // pj_status_t 	pj_grp_lock_create_w_handler (pj_pool_t *pool, const pj_grp_lock_config *cfg, void *member, void(*handler)(void *member), pj_grp_lock_t **p_grp_lock)
    pub fn create_w_handler() -> Result<(), i32> {
        Ok(())
    }

    // pj_status_t 	pj_grp_lock_destroy (pj_grp_lock_t *grp_lock)
    pub fn destroy(self) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_destroy(*self.ctx)) }
    }

    // pj_status_t 	pj_grp_lock_replace (pj_grp_lock_t *old_lock, pj_grp_lock_t *new_lock)
    pub fn pj_grp_lock_replace(&self) -> Result<(), i32> {
        unsafe { Ok(()) }
    }


    // pj_status_t 	pj_grp_lock_acquire (pj_grp_lock_t *grp_lock)
    pub fn acquire(&self) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_acquire(*self.ctx)) }
    }

    // pj_status_t 	pj_grp_lock_tryacquire (pj_grp_lock_t *grp_lock)
    pub fn tryacquire(&self) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_tryacquire(*self.ctx)) }
    }

    // pj_status_t 	pj_grp_lock_release (pj_grp_lock_t *grp_lock)
    pub fn release(&self) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_release(*self.ctx)) }
    }

    // pj_status_t 	pj_grp_lock_add_handler (pj_grp_lock_t *grp_lock, pj_pool_t *pool, void *member, void(*handler)(void *member))
    pub fn add_handler(&self, pool: &PJPool, member: &mut Box<*mut c_void>, handler: Option<unsafe extern "C" fn(*mut c_void)>) -> Result<(), i32> {
        unsafe {
            check_status(pj_grp_lock_add_handler(*self.ctx, *pool.ctx, **member, handler))
        }
    }


    // pj_status_t 	pj_grp_lock_del_handler (pj_grp_lock_t *grp_lock, void *member, void(*handler)(void *member))
    pub fn del_handler(&self, member: &mut Box<*mut c_void>, handler: Option<unsafe extern "C" fn(*mut c_void)>) -> Result<(), i32> {
        unsafe {
            check_status(pj_grp_lock_del_handler(*self.ctx, **member, handler))
        }
    }

    // pj_status_t 	pj_grp_lock_add_ref (pj_grp_lock_t *grp_lock)
    pub fn add_ref(&self) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_add_ref(*self.ctx)) }
    }

    // pj_status_t 	pj_grp_lock_dec_ref (pj_grp_lock_t *grp_lock)
    pub fn dec_ref(&self) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_dec_ref(*self.ctx)) }
    }

    // int 	pj_grp_lock_get_ref (pj_grp_lock_t *grp_lock)
    pub fn get_ref(&self) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_get_ref(*self.ctx)) }
    }

    // void 	pj_grp_lock_dump (pj_grp_lock_t *grp_lock)
    pub fn dump(&self) {
        unsafe { pj_grp_lock_dump(*self.ctx) }
    }

    // pj_status_t 	pj_grp_lock_chain_lock (pj_grp_lock_t *grp_lock, pj_lock_t *ext_lock, int pos)
    pub fn chain_lock(&self, ext_lock: &mut Box<*mut pj_lock_t>, pos: i32) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_chain_lock(*self.ctx, **ext_lock, pos)) }
    }

    // pj_status_t 	pj_grp_lock_unchain_lock (pj_grp_lock_t *grp_lock, pj_lock_t *ext_lock)
    pub fn unchain_lock(&self, ext_lock: &mut Box<*mut pj_lock_t>) -> Result<(), i32> {
        unsafe { check_status(pj_grp_lock_unchain_lock(*self.ctx, **ext_lock)) }
    }

}