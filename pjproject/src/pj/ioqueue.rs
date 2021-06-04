

use std::ffi::{CStr, c_void};

use pj_sys::*;

use crate::utils::{boolean_to_pjbool, check_boolean, check_status};

use super::PJPool;

pub struct PJIoQueue { pub ctx: Box<*mut pj_ioqueue_t>}
pub struct PJIoQueueKey { pub ctx: Box<*mut pj_ioqueue_key_t> }

impl From<Box<*mut pj_ioqueue_t>> for PJIoQueue {
    fn from(ptr: Box<*mut pj_ioqueue_t>) -> Self {
        Self { ctx: ptr }
    }
}

impl From<Box<*mut pj_ioqueue_key_t>> for PJIoQueueKey {
    fn from(ptr: Box<*mut pj_ioqueue_key_t>) -> Self {
        Self { ctx: ptr }
    }
}


impl PJIoQueue {

    // const char * 	pj_ioqueue_name (void)
    pub fn name() -> String {
        unsafe {
            let cstr = pj_ioqueue_name();
            CStr::from_ptr(cstr).to_string_lossy().into_owned()
        }
    }

    // pj_status_t 	pj_ioqueue_create (pj_pool_t *pool, pj_size_t max_fd, pj_ioqueue_t **ioqueue)
    pub fn create(pool: &PJPool, max_fd: u64) -> Result<Self, i32> {
        unsafe {
            let ioqueue = Box::new(std::ptr::null_mut());
            let status = check_status(pj_ioqueue_create(*pool.ctx, max_fd, *ioqueue as *mut _));

            match status {
                Ok(()) => return Ok(Self::from(ioqueue)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_ioqueue_destroy (pj_ioqueue_t *ioque)
    pub fn destroy(self) -> Result<(), i32> {
        unsafe { check_status(pj_ioqueue_destroy(*self.ctx)) }
    }

    // pj_status_t 	pj_ioqueue_set_lock (pj_ioqueue_t *ioque, pj_lock_t *lock, pj_bool_t auto_delete)
    pub fn set_lock(&self, lock: &mut Box<*mut pj_lock_t>, auto_delete: bool) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_set_lock(*self.ctx, **lock, boolean_to_pjbool(auto_delete)))
         }
    }

    // pj_status_t 	pj_ioqueue_set_default_concurrency (pj_ioqueue_t *ioqueue, pj_bool_t allow)
    pub fn set_default_concurrency(&self, allow: bool) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_set_default_concurrency(*self.ctx, boolean_to_pjbool(allow)))
        }
    }

    // int 	pj_ioqueue_poll (pj_ioqueue_t *ioque, const pj_time_val *timeout)
    pub fn poll(&self, timeout: &pj_time_val) -> i32 {
        unsafe { pj_ioqueue_poll(*self.ctx, timeout as *const _) }
    }


}

impl PJIoQueueKey {

    // pj_status_t 	pj_ioqueue_register_sock (pj_pool_t *pool, pj_ioqueue_t *ioque, pj_sock_t sock, void *user_data, const pj_ioqueue_callback *cb, pj_ioqueue_key_t **key)
    pub fn register_sock(
        pool: &PJPool,
        ioque: &PJIoQueue,
        sock: pj_sock_t,
        user_data: &mut Box<*mut c_void>,
        cb: Box<*const pj_ioqueue_callback>
    ) -> Result<PJIoQueueKey, i32> {
        unsafe {
            let ioqueue_key = Box::new(std::ptr::null_mut());
            let status = check_status(pj_ioqueue_register_sock(
                *pool.ctx,
                *ioque.ctx,
                sock,
                **user_data,
                *cb,
                *ioqueue_key as *mut _
            ));

            match status {
                Ok(()) => return Ok(PJIoQueueKey::from(ioqueue_key)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_ioqueue_register_sock2 (pj_pool_t *pool, pj_ioqueue_t *ioque, pj_sock_t sock, pj_grp_lock_t *grp_lock, void *user_data, const pj_ioqueue_callback *cb, pj_ioqueue_key_t **key)
    pub fn register_sock2(
        pool: &PJPool,
        ioque: &PJIoQueue,
        sock: pj_sock_t,
        grp_lock: &mut Box<*mut pj_grp_lock_t>,
        user_data: &mut Box<*mut c_void>,
        cb: Box<*const pj_ioqueue_callback>
    ) -> Result<PJIoQueueKey, i32> {
        unsafe {
            let ioqueue_key = Box::new(std::ptr::null_mut());
            let status = check_status(pj_ioqueue_register_sock2(
                *pool.ctx,
                *ioque.ctx,
                sock,
                **grp_lock,
                **user_data,
                *cb,
                *ioqueue_key as *mut _
            ));

            match status {
                Ok(()) => return Ok(PJIoQueueKey::from(ioqueue_key)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_ioqueue_unregister (pj_ioqueue_key_t *key)
    pub fn unregister(self) -> Result<(), i32> {
        unsafe { check_status(pj_ioqueue_unregister(*self.ctx)) }
    }

    // void * 	pj_ioqueue_get_user_data (pj_ioqueue_key_t *key)
    pub fn get_user_data(&self) -> Box<*mut c_void> {
        unsafe { Box::new(pj_ioqueue_get_user_data(*self.ctx)) }
    }

    // pj_status_t 	pj_ioqueue_set_user_data (pj_ioqueue_key_t *key, void *user_data, void **old_data)


    // pj_status_t 	pj_ioqueue_set_concurrency (pj_ioqueue_key_t *key, pj_bool_t allow)
    pub fn set_concurrency(&self, allow: bool) -> Result<(), i32> {
        unsafe { check_status(pj_ioqueue_set_concurrency(*self.ctx, boolean_to_pjbool(allow))) }
    }

    // pj_status_t 	pj_ioqueue_lock_key (pj_ioqueue_key_t *key)
    pub fn lock_key(&self) -> Result<(), i32> {
        unsafe { check_status(pj_ioqueue_lock_key(*self.ctx)) }
    }

    // pj_status_t 	pj_ioqueue_trylock_key (pj_ioqueue_key_t *key)
    pub fn trylock_key(&self) -> Result<(), i32> {
        unsafe { check_status(pj_ioqueue_trylock_key(*self.ctx)) }
    }

    // pj_status_t 	pj_ioqueue_unlock_key (pj_ioqueue_key_t *key)
    pub fn unlock_key(&self) -> Result<(), i32> {
        unsafe { check_status(pj_ioqueue_unlock_key(*self.ctx)) }
    }

    // void 	pj_ioqueue_op_key_init (pj_ioqueue_op_key_t *op_key, pj_size_t size)


    // pj_bool_t 	pj_ioqueue_is_pending (pj_ioqueue_key_t *key, pj_ioqueue_op_key_t *op_key)
    pub fn pending(&self, op_key: &mut Box<*mut pj_ioqueue_op_key_t>) -> bool {
        unsafe { check_boolean(pj_ioqueue_is_pending(*self.ctx, **op_key)) }
    }

    // pj_status_t 	pj_ioqueue_post_completion (pj_ioqueue_key_t *key, pj_ioqueue_op_key_t *op_key, pj_ssize_t bytes_status)
    pub fn post_completion(&self, op_key: &mut Box<*mut pj_ioqueue_op_key_t>, bytes_status: i64) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_post_completion(
                *self.ctx, **op_key, bytes_status
            ))
        }
    }

    // pj_status_t 	pj_ioqueue_accept (pj_ioqueue_key_t *key, pj_ioqueue_op_key_t *op_key, pj_sock_t *new_sock, pj_sockaddr_t *local, pj_sockaddr_t *remote, int *addrlen)
    pub fn accept(
        &self, op_key: &mut Box<*mut pj_ioqueue_op_key_t>,
        new_sock: &mut Box<*mut pj_sock_t>,
        local: &mut Box<*mut pj_sockaddr_t>,
        remote: &mut Box<*mut pj_sockaddr_t>,
        addrlen: &mut Box<*mut i32>
    ) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_accept(
                *self.ctx,
                **op_key,
                **new_sock,
                **local,
                **remote,
                **addrlen
            ))
        }
    }

    // pj_status_t 	pj_ioqueue_connect (pj_ioqueue_key_t *key, const pj_sockaddr_t *addr, int addrlen)
    pub fn connect(&self, addr: &mut Box<*const pj_sockaddr_t>, addrlen: i32) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_connect(
                *self.ctx,
                **addr,
                addrlen
            ))
        }
    }

    // pj_status_t 	pj_ioqueue_recv (pj_ioqueue_key_t *key, pj_ioqueue_op_key_t *op_key, void *buffer, pj_ssize_t *length, pj_uint32_t flags)
    pub fn recv(&self, op_key: &mut Box<*mut pj_ioqueue_op_key_t>, buffer: &mut Box<*mut c_void>, length: &mut Box<*mut i64>, flags: u32) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_recv(*self.ctx, **op_key, **buffer, **length, flags))
        }
    }

    // pj_status_t 	pj_ioqueue_recvfrom (pj_ioqueue_key_t *key, pj_ioqueue_op_key_t *op_key, void *buffer, pj_ssize_t *length, pj_uint32_t flags, pj_sockaddr_t *addr, int *addrlen)
    pub fn recvfrom(
        &self,
        op_key: &mut Box<*mut pj_ioqueue_op_key_t>,
        buffer: &mut Box<*mut c_void>,
        length: &mut Box<*mut i64>,
        flags: u32,
        addr: &mut Box<*mut pj_sockaddr_t>,
        addrlen: &mut Box<*mut i32>
    ) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_recvfrom(
                *self.ctx,
                **op_key,
                **buffer,
                **length,
                flags,
                **addr,
                **addrlen
            ))
        }
    }


    // pj_status_t 	pj_ioqueue_send (pj_ioqueue_key_t *key, pj_ioqueue_op_key_t *op_key, const void *data, pj_ssize_t *length, pj_uint32_t flags)
    pub fn send(&self, op_key: &mut Box<*mut pj_ioqueue_op_key_t>, data: &mut Box<*const c_void>, length: &mut Box<*mut i64>, flags: u32) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_send(
                *self.ctx,
                **op_key,
                **data,
                **length,
                flags
            ))
        }
    }

    // pj_status_t 	pj_ioqueue_sendto (pj_ioqueue_key_t *key, pj_ioqueue_op_key_t *op_key, const void *data, pj_ssize_t *length, pj_uint32_t flags, const pj_sockaddr_t *addr, int addrlen)
    pub fn sendto(
        &self,
        op_key: &mut Box<*mut pj_ioqueue_op_key_t>,
        data: &mut Box<*const c_void>,
        length: &mut Box<*mut i64>,
        flags: u32,
        addr: &mut Box<*const pj_sockaddr_t>,
        addrlen: i32
    ) -> Result<(), i32> {
        unsafe {
            check_status(pj_ioqueue_sendto(
                *self.ctx,
                **op_key,
                **data,
                **length,
                flags,
                **addr,
                addrlen
            ))
        }
    }

}
