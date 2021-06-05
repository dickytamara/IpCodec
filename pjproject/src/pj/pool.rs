

use std::ffi::{CStr, CString, c_void};

use pj_sys::*;

pub struct PJPool { pub ctx: Box<*mut pj_pool_t>, }

impl From<Box<*mut pj_pool_t>> for PJPool {
    fn from(ptr: Box<*mut pj_pool_t>) -> Self {
        Self{ ctx: ptr }
    }
}

pub struct PJPoolBlock { pub ctx: Box<pj_pool_block> }

impl From<pj_pool_block> for PJPoolBlock {
    fn from (type_: pj_pool_block) -> Self {
        Self { ctx: Box::new(type_)}
    }
}

impl PJPool {

    // pj_pool_create
    pub fn create(
        factory: &mut Box<*mut pj_pool_factory>,
        name: String,
        initial_size: u64,
        increment_size: u64,
        callback: pj_pool_callback
    ) -> Self {
        unsafe {
            let name = CString::new(name.as_str()).unwrap().into_raw();
            let pool = pj_pool_create(
                **factory,
                name,
                initial_size,
                increment_size,
                callback
            );

            PJPool::from(Box::new(pool))
        }
    }

    // pj_pool_release
    pub fn release(&self) {
        unsafe { pj_pool_release(*self.ctx) }
    }

    // pj_pool_safe_release
    pub fn safe_release(self) {
        unsafe { pj_pool_safe_release(*self.ctx as *mut _) }
    }

    // pj_pool_secure_release
    pub fn secure_release(self) {
        unsafe { pj_pool_secure_release(*self.ctx as *mut _) }
    }

    // pj_pool_getobjname
    pub fn getobjname(&self) -> String {
        unsafe {
            let result = pj_pool_getobjname(*self.ctx);
            CStr::from_ptr(result).to_str().unwrap().to_string()
        }
    }

    // pj_pool_reset
    pub fn reset(&self) {
        unsafe { pj_pool_reset(*self.ctx) }
    }

    // pj_pool_get_capacity
    pub fn get_capacity(&self) -> u64 {
        unsafe { pj_pool_get_capacity(*self.ctx) }
    }

    // pj_pool_get_used_size
    pub fn get_used_size(&self) -> u64 {
        unsafe { pj_pool_get_used_size(*self.ctx) }
    }

    // pj_pool_alloc
    pub fn alloc(&self, size: u64) -> Box<*mut c_void> {
        unsafe { Box::new(pj_pool_alloc(*self.ctx, size)) }
    }

    // pj_pool_calloc
    pub fn calloc(&self, count: u64, elem: u64) -> Box<*mut c_void> {
        unsafe { Box::new(pj_pool_calloc(*self.ctx, count, elem)) }
    }

    // this function is only availabe on windows platform
    // void * 	pj_pool_zalloc (pj_pool_t *pool, pj_size_t size)
    // pub fn zalloc(&self, size: u64) -> Box<*mut c_void> {
    //     unsafe { Box::new(pj_pool_zalloc(*self.ctx, size)) }
    // }

    // pj_pool_alloc_from_block
    pub fn alloc_from_block(block: &mut PJPoolBlock, size: u64) -> Box<*mut c_void> {
        unsafe { Box::new(pj_pool_alloc_from_block(block.ctx.as_mut() as *mut _, size)) }
    }
    // pj_pool_allocate_find
    pub fn allocate_find(&self, size: u64) -> Box<*mut c_void> {
        unsafe { Box::new(pj_pool_allocate_find(*self.ctx, size)) }
    }
}


