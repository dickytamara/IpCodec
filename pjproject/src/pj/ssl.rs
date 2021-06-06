use std::ffi::c_void;

use pj_sys::*;

use crate::utils::check_status;

use super::PJPool;



pub struct PJSslSock { pub ctx: Box<*mut pj_ssl_sock_t> }

impl From<Box<*mut pj_ssl_sock_t>> for PJSslSock {
    fn from(ptr: Box<*mut pj_ssl_sock_t>) -> Self {
        Self { ctx: ptr }
    }
}



// pj_status_t 	pj_ssl_cert_load_from_files (pj_pool_t *pool, const pj_str_t *CA_file, const pj_str_t *cert_file, const pj_str_t *privkey_file, const pj_str_t *privkey_pass, pj_ssl_cert_t **p_cert)

// pj_status_t 	pj_ssl_cert_load_from_files2 (pj_pool_t *pool, const pj_str_t *CA_file, const pj_str_t *CA_path, const pj_str_t *cert_file, const pj_str_t *privkey_file, const pj_str_t *privkey_pass, pj_ssl_cert_t **p_cert)

// pj_status_t 	pj_ssl_cert_load_from_buffer (pj_pool_t *pool, const pj_ssl_cert_buffer *CA_buf, const pj_ssl_cert_buffer *cert_buf, const pj_ssl_cert_buffer *privkey_buf, const pj_str_t *privkey_pass, pj_ssl_cert_t **p_cert)

// pj_ssize_t 	pj_ssl_cert_info_dump (const pj_ssl_cert_info *ci, const char *indent, char *buf, pj_size_t buf_size)

// pj_status_t 	pj_ssl_cert_get_verify_status_strings (pj_uint32_t verify_status, const char *error_strings[], unsigned *count)

// void 	pj_ssl_cert_wipe_keys (pj_ssl_cert_t *cert)

// pj_status_t 	pj_ssl_cipher_get_availables (pj_ssl_cipher ciphers[], unsigned *cipher_num)

// pj_bool_t 	pj_ssl_cipher_is_supported (pj_ssl_cipher cipher)

// const char * 	pj_ssl_cipher_name (pj_ssl_cipher cipher)

// pj_ssl_cipher 	pj_ssl_cipher_id (const char *cipher_name)

// pj_status_t 	pj_ssl_curve_get_availables (pj_ssl_curve curves[], unsigned *curve_num)

// pj_bool_t 	pj_ssl_curve_is_supported (pj_ssl_curve curve)

// const char * 	pj_ssl_curve_name (pj_ssl_curve curve)

// pj_ssl_curve 	pj_ssl_curve_id (const char *curve_name)

// void 	pj_ssl_sock_param_default (pj_ssl_sock_param *param)

// void 	pj_ssl_sock_param_copy (pj_pool_t *pool, pj_ssl_sock_param *dst, const pj_ssl_sock_param *src)


impl PJSslSock {

    // pj_status_t 	pj_ssl_sock_create (pj_pool_t *pool, const pj_ssl_sock_param *param,  pj_ssl_sock_t **p_ssock)
    pub fn create(pool: &PJPool, param: &mut pj_ssl_sock_param) -> Result<PJSslSock, i32> {
        unsafe {
            let p_ssock = Box::new(std::ptr::null_mut());
            let status = check_status(pj_ssl_sock_create(*pool.ctx, param as *const _, *p_ssock as *mut _));

            match status {
                Ok(()) => return Ok(PJSslSock::from(p_ssock)),
                Err(e) => return Err(e),
            }
        }
    }

    // pj_status_t 	pj_ssl_sock_set_certificate (pj_ssl_sock_t *ssock, pj_pool_t *pool, const pj_ssl_cert_t *cert)
    pub fn set_certificate(&self, pool: &PJPool, cert: &mut Box<*mut pj_ssl_cert_t>) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_set_certificate(*self.ctx, *pool.ctx, **cert)) }
    }

    // pj_status_t 	pj_ssl_sock_close (pj_ssl_sock_t *ssock)
    pub fn close(&self) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_close(*self.ctx)) }
    }

    // pj_status_t 	pj_ssl_sock_set_user_data (pj_ssl_sock_t *ssock, void *user_data)
    pub fn set_user_data(&self, user_data: &mut Box<*mut c_void>) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_set_user_data(*self.ctx, **user_data)) }
    }

    // void * 	pj_ssl_sock_get_user_data (pj_ssl_sock_t *ssock)
    pub fn get_user_data(&self) -> Box<*mut c_void> {
        unsafe { Box::new(pj_ssl_sock_get_user_data(*self.ctx)) }
    }

    // pj_status_t 	pj_ssl_sock_get_info (pj_ssl_sock_t *ssock, pj_ssl_sock_info *info)
    pub fn get_info(&self, info: &mut Box<*mut pj_ssl_sock_info>) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_get_info(*self.ctx, **info)) }
    }

    // pj_status_t 	pj_ssl_sock_start_read (pj_ssl_sock_t *ssock, pj_pool_t *pool, unsigned buff_size, pj_uint32_t flags)
    pub fn start_read(&self, pool: &PJPool, buff_size: u32, flags: u32) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_start_read(*self.ctx, *pool.ctx, buff_size, flags)) }
    }

    // pj_status_t 	pj_ssl_sock_start_read2 (pj_ssl_sock_t *ssock, pj_pool_t *pool, unsigned buff_size, void *readbuf[], pj_uint32_t flags)

    // pj_status_t 	pj_ssl_sock_start_recvfrom (pj_ssl_sock_t *ssock, pj_pool_t *pool, unsigned buff_size, pj_uint32_t flags)
    pub fn start_recvfrom(&self, pool: &PJPool, buff_size: u32, flags: u32) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_start_recvfrom(*self.ctx, *pool.ctx, buff_size, flags)) }
    }

    // pj_status_t 	pj_ssl_sock_start_recvfrom2 (pj_ssl_sock_t *ssock, pj_pool_t *pool, unsigned buff_size, void *readbuf[], pj_uint32_t flags)

    // pj_status_t 	pj_ssl_sock_send (pj_ssl_sock_t *ssock, pj_ioqueue_op_key_t *send_key, const void *data, pj_ssize_t *size, unsigned flags)
    pub fn send(&self, send_key: &mut pj_ioqueue_op_key_t, data: &mut Box<*const c_void>, size: &mut i64, flags: u32) -> Result<(), i32> {
        unsafe {
            check_status(pj_ssl_sock_send(
                *self.ctx,
                send_key as *mut _,
                **data,
                size as *mut _,
                flags
            ))
        }
    }

    // pj_status_t 	pj_ssl_sock_sendto (pj_ssl_sock_t *ssock, pj_ioqueue_op_key_t *send_key, const void *data, pj_ssize_t *size, unsigned flags, const pj_sockaddr_t *addr, int addr_len)
    pub fn sendto(
        &self,
        send_key: &mut pj_ioqueue_op_key_t,
        data: &mut Box<*const c_void>,
        size: &mut i64,
        flags: u32,
        addr: &mut Box<*mut pj_sockaddr_t>,
        addr_len: i32
    ) -> Result<(), i32> {
        unsafe {
            check_status(pj_ssl_sock_sendto(
                *self.ctx,
                send_key as *mut _,
                **data,
                size as *mut _,
                flags,
                **addr,
                addr_len
            ))
        }
    }

    // pj_status_t 	pj_ssl_sock_start_accept (pj_ssl_sock_t *ssock, pj_pool_t *pool, const pj_sockaddr_t *local_addr, int addr_len)
    pub fn start_accept(&self, pool: &PJPool, local_addr: &mut Box<*mut pj_sockaddr_t>, addr_len: i32) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_start_accept(*self.ctx, *pool.ctx, **local_addr, addr_len)) }
    }

    // pj_status_t 	pj_ssl_sock_start_accept2 (pj_ssl_sock_t *ssock, pj_pool_t *pool, const pj_sockaddr_t *local_addr, int addr_len, const pj_ssl_sock_param *newsock_param)
    pub fn start_accept2(&self, pool: &PJPool, local_addr: &mut Box<*mut pj_sockaddr_t>, addr_len: i32, newsock_param: &mut Box<*const pj_ssl_sock_param>) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_start_accept2(*self.ctx, *pool.ctx, **local_addr, addr_len, **newsock_param)) }
    }

    // pj_status_t 	pj_ssl_sock_start_connect (pj_ssl_sock_t *ssock, pj_pool_t *pool, const pj_sockaddr_t *localaddr, const pj_sockaddr_t *remaddr, int addr_len)
    pub fn start_connect(&self, pool: &PJPool, localaddr: &mut Box<*mut pj_sockaddr_t>, remaddr: &mut Box<*mut pj_sockaddr_t>, addr_len: i32) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_start_connect(*self.ctx, *pool.ctx, **localaddr, **remaddr, addr_len)) }
    }

    // pj_status_t 	pj_ssl_sock_start_connect2 (pj_ssl_sock_t *ssock, pj_ssl_start_connect_param *connect_param)
    pub fn start_connect2(&self, connect_param: &mut Box<*mut pj_ssl_start_connect_param>) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_start_connect2(*self.ctx, **connect_param)) }
    }

    // pj_status_t 	pj_ssl_sock_renegotiate (pj_ssl_sock_t *ssock)
    pub fn renegotiate(&self) -> Result<(), i32> {
        unsafe { check_status(pj_ssl_sock_renegotiate(*self.ctx)) }
    }
}
