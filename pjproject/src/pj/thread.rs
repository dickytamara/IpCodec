
use std::ffi::{CStr, CString, c_void};

use pj_sys::*;

use crate::utils::{check_boolean, check_status};

use super::PJPool;

pub struct PJThread { pub ctx: Box<*mut pj_thread_t> }

impl From<Box<*mut pj_thread_t>> for PJThread {
    fn from(thread: Box<*mut pj_thread_t>) -> Self {
        Self { ctx: thread }
    }
}

impl PJThread {

    // pj_thread_create
    pub fn create(
        pool: &PJPool,
        thread_name: String,
        proc_: pj_thread_proc,
        arg: &mut Box<*mut c_void>,
        stack_size: u64,
        flags: u32
    ) -> Result<PJThread, i32> {
        unsafe {
            let thread: Box<*mut pj_thread_t> = Box::new(std::ptr::null_mut());
            let thread_name = CString::new(thread_name.as_str()).unwrap().into_raw();

            let status = check_status(pj_thread_create(
                *pool.ctx, thread_name, proc_, **arg,
                stack_size, flags, *thread as *mut _
            ));

            match status {
                Ok(()) => {
                    return Ok(PJThread::from(thread))
                },
                Err(e) => return Err(e)
            }
        }
    }

    pub fn register(thread_name: Option<String>, desc: &mut pj_thread_desc) -> Result<Self, i32> {
        unsafe {
            let mut thread = Self { ctx: Box::new(std::ptr::null_mut()) };

            let thread_name = match thread_name {
                Some(value) => std::ffi::CString::new(value.as_str())
                    .expect("Error::pj_thread_register").into_raw(),
                None => std::ptr::null_mut()
            };

            let status = pj_thread_register( thread_name, desc.as_mut_ptr(), thread.ctx.as_mut() as *mut _);

            match check_status(status) {
                Ok(()) => {
                    if !(*thread.ctx).is_null() {
                        return Ok(thread);
                    } else {
                        return Err(-1);
                    }
                },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn is_registered() -> bool {
        unsafe { check_boolean(pj_thread_is_registered()) }
    }

    pub fn get_prio(&self) -> i32 {
        unsafe { pj_thread_get_prio(*self.ctx) }
    }

    pub fn set_prio(&self, prio: i32) -> Result<(), i32> {
        unsafe {
            check_status(pj_thread_set_prio(*self.ctx, prio))
        }
    }

    pub fn get_prio_min(&self) -> i32 {
        unsafe { pj_thread_get_prio_min(*self.ctx) }
    }

    pub fn get_prio_max(&self) -> i32 {
        unsafe { pj_thread_get_prio_max(*self.ctx) }
    }

    pub fn get_os_handle(&self) -> Box<*mut c_void> {
        unsafe { Box::new(pj_thread_get_os_handle(*self.ctx)) }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let cstr = pj_thread_get_name(*self.ctx);
            CStr::from_ptr(cstr).to_str().unwrap().to_string()
        }
    }

    pub fn thread_this() -> Self {
        unsafe { Self::from(Box::new(pj_thread_this())) }
    }

    pub fn resume(&self) -> Result<(), i32> {
        unsafe { check_status(pj_thread_resume(*self.ctx)) }
    }

    pub fn join(&self) -> Result<(), i32> {
        unsafe { check_status(pj_thread_join(*self.ctx)) }
    }

    pub fn destroy(self) -> Result<(), i32> {
        unsafe { check_status(pj_thread_destroy(*self.ctx)) }
    }

    pub fn sleep(msec: u32) -> Result<(), i32> {
        unsafe { check_status(pj_thread_sleep(msec)) }
    }

    pub fn getpid() -> u32 {
        unsafe { pj_getpid() }
    }

}
