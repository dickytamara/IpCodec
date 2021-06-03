

use std::ffi::{CStr, c_void};

use pj_sys::*;

pub struct PJPool { ctx: Box<*mut pj_pool_t> }

impl From<*mut pj_pool_t> for PJPool {
    fn from(ptr: *mut pj_pool_t) -> Self {
        Self{ ctx: Box::new(ptr) }
    }
}

impl PJPool {

    // pool_t * 	pj_pool_create (pj_pool_factory *factory, const char *name, pj_size_t initial_size, pj_size_t increment_size, pj_pool_callback *callback)
    // pub fn create() -> Self {

    // }

    // void 	pj_pool_release (pj_pool_t *pool)
    pub fn release(self) {
        unsafe { pj_pool_release(*self.ctx) }
    }

    // void 	pj_pool_safe_release (pj_pool_t **ppool)
    pub fn safe_release(self) {
        unsafe { pj_pool_safe_release(*self.ctx as *mut _) }
    }

    // void 	pj_pool_secure_release (pj_pool_t **ppool)
    pub fn secure_release(self) {
        unsafe { pj_pool_secure_release(*self.ctx as *mut _) }
    }

    // const char * 	pj_pool_getobjname (const pj_pool_t *pool)
    pub fn getobjname(&self) -> String {
        unsafe {
            let result = pj_pool_getobjname(*self.ctx);
            CStr::from_ptr(result).to_str().unwrap().to_string()
        }
    }

    // void 	pj_pool_reset (pj_pool_t *pool)
    pub fn reset(&self) {
        unsafe { pj_pool_reset(*self.ctx) }
    }

    // pj_size_t 	pj_pool_get_capacity (pj_pool_t *pool)
    pub fn get_capacity(&self) -> u64 {
        unsafe { pj_pool_get_capacity(*self.ctx) }
    }

    // pj_size_t 	pj_pool_get_used_size (pj_pool_t *pool)
    pub fn get_used_size(&self) -> u64 {
        unsafe { pj_pool_get_used_size(*self.ctx) }
    }

    // void * 	pj_pool_alloc (pj_pool_t *pool, pj_size_t size)
    pub fn alloc(&self, size: u64) -> Box<*mut c_void> {
        unsafe { Box::new(pj_pool_alloc(*self.ctx, size)) }
    }

    // void * 	pj_pool_calloc (pj_pool_t *pool, pj_size_t count, pj_size_t elem)
    pub fn calloc(&self, count: u64, elem: u64) -> Box<*mut c_void> {
        unsafe { Box::new(pj_pool_calloc(*self.ctx, count, elem)) }
    }

    // this function is only availabe on windows platform
    // void * 	pj_pool_zalloc (pj_pool_t *pool, pj_size_t size)
    // pub fn zalloc(&self, size: u64) -> Box<*mut c_void> {
    //     unsafe { Box::new(pj_pool_zalloc(*self.ctx, size)) }
    // }

    // void * 	pj_pool_alloc_from_block (pj_pool_block *block, pj_size_t size)
    pub fn alloc_from_block(block: &mut Box<*mut pj_pool_block>, size: u64) -> Box<*mut c_void> {
        unsafe { Box::new(pj_pool_alloc_from_block(**block, size)) }
    }
    // void * 	pj_pool_allocate_find (pj_pool_t *pool, pj_size_t size)
    pub fn allocate_find(&self, size: u64) -> Box<*mut c_void> {
        unsafe { Box::new(pj_pool_allocate_find(*self.ctx, size)) }
    }
}


