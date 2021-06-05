

use std::ffi::{CString, c_void};

use pjlib_util_sys::pj_dns_resolver;
use utils::{FromString, ToString};

use crate::{pj::PJPool, utils::{boolean_to_pjbool, check_boolean, check_status}};

use super::*;


pub struct SIPEndpoint { ctx: Box<*mut pjsip_sys::pjsip_endpoint> }

impl From<Box<*mut pjsip_endpoint>> for SIPEndpoint {
    fn from(ptr: Box<*mut pjsip_endpoint>) -> Self {
        Self { ctx: ptr }
    }
}

impl SIPEndpoint {

    pub fn new(pf: &mut Box<*mut pj_pool_factory>, name: String) -> Self {
        SIPEndpoint::create(pf, name).unwrap()
    }

    // pjsip_endpt_create
    pub fn create(pf: &mut Box<*mut pj_pool_factory>, name: String) -> Result<SIPEndpoint, i32> {
        unsafe {
            let name = CString::new(name.as_str()).expect("pjsip_endpt_create").into_raw();
            let endpt: Box<*mut pjsip_sys::pjsip_endpoint> = Box::new(std::ptr::null_mut());

            let status = check_status( pjsip_endpt_create(
                **pf,
                name as *const _,
                *endpt as *mut _)
            );

            match status {
                Ok(()) => {
                    return Ok(SIPEndpoint::from(endpt))
                }
                Err(e) => return Err(e)
            }
        }
    }

    // pjsip_endpt_destroy
    pub fn destroy(self) {
        unsafe { pjsip_endpt_destroy(*self.ctx); }
    }

    // pjsip_endpt_name
    pub fn name(&self) -> String {
        unsafe {
            let res = *pjsip_endpt_name(*self.ctx);
            res.to_string().clone()
        }
    }

    // pjsip_endpt_handle_events
    pub fn handle_events(&self, max_timeout: &mut pj_time_val) -> Result<(), i32> {
        unsafe {
            check_status(
                pjsip_endpt_handle_events(*self.ctx,
                    max_timeout as *const _)
            )
        }
    }

    // pjsip_endpt_handle_events2
    pub fn handle_events2(&self, max_timeout: &mut pj_time_val, count: &mut u32) -> Result<(), i32> {
        unsafe {
            check_status(
                pjsip_endpt_handle_events2(
                    *self.ctx,
                    max_timeout as *const _,
                    count as *mut _
                )
            )
         }
    }

    // pjsip_endpt_schedule_timer
    pub fn sechedule_timer(&self, entry: &mut pj_timer_entry, delay: &mut pj_time_val) -> Result<(), i32> {
        unsafe {
            check_status(
                pjsip_endpt_schedule_timer_dbg(
                    *self.ctx,
                    entry as *mut _,
                    delay as *mut _,
                    std::ptr::null(),
                    0)
            )
        }
    }

    // pjsip_endpt_schedule_timer_w_grp_lock
    pub fn schedule_timer_w_grp_lock(
        &self,
        entry: &mut Box<*mut pj_timer_entry>,
        delay: &mut Box<*const pj_time_val>,
        id_val: i32,
        grp_lock: &mut Box<*mut pj_grp_lock_t>
    ) -> Result<(), i32> {
        unsafe {
            check_status(pjsip_endpt_schedule_timer_w_grp_lock_dbg(
                *self.ctx, **entry, **delay, id_val, **grp_lock,
                std::ptr::null(), 0
            ))
        }
    }

    // pjsip_endpt_cancel_timer
    pub fn cancel_timer(&self, entry: &mut pj_timer_entry) {
        unsafe {
            pjsip_endpt_cancel_timer(
                *self.ctx,
                entry as *mut _)
            }
    }

    // pjsip_endpt_get_timer_heap
    pub fn get_timer_heap(&self) -> Box<*mut pj_timer_heap_t> {
        unsafe { Box::new(pjsip_endpt_get_timer_heap(*self.ctx)) }
    }

    // pjsip_endpt_register_module
    pub fn register_module(&self, module: &mut pjsip_module) -> Result<(), i32> {
        unsafe { check_status( pjsip_endpt_register_module(*self.ctx, module as *mut _)) }
    }

    // pjsip_enpt_unregister_module
    pub fn unregister_module(&self, module: &mut pjsip_module) -> Result<(), i32> {
        unsafe { check_status(pjsip_endpt_unregister_module(*self.ctx, module as *mut _)) }
    }

    // pjsip_process_rdata_param_default
    pub fn process_rdata_param_default(p: &mut Box<*mut pjsip_process_rdata_param>) {
        unsafe { pjsip_process_rdata_param_default(**p);
        }
    }

    // pjsip_endpt_process_rx_data
    pub fn process_rx_data(&self, rdata: &mut Box<*mut pjsip_rx_data>, p: Option<Box<*mut pjsip_process_rdata_param>>, p_handled: &mut Option<bool>) -> Result<(), i32> {
        unsafe {
            let p = match p {
                Some(param) => {
                    *param
                },
                None => std::ptr::null_mut()
            };

            let tmp = match p_handled {
                Some(ref mut handled) => {
                    boolean_to_pjbool(handled.clone()) as *mut _
                },
                None => std::ptr::null_mut()
            };

            let status = pjsip_endpt_process_rx_data(*self.ctx, **rdata, p, tmp);

            if p_handled.is_some() {
                p_handled.as_mut().map(|val| *val = check_boolean(*tmp.clone()));
            }

            check_status(status)
        }
    }

    // pjsip_endpt_create_pool
    pub fn create_pool(&self, pool_name: String, initial: u64, increment: u64) -> Box<*mut pj_pool_t> {
        unsafe {
            let pool_name = CString::new(pool_name.as_str()).expect("pjsip_endpt_create_pool").into_raw();
            Box::new(pjsip_endpt_create_pool(*self.ctx, pool_name, initial, increment))
        }
    }

    // pjsip_endpt_release_pool
    pub fn release_pool(&self, pool: Box<*mut pj_pool_t>) {
        unsafe {
            pjsip_endpt_release_pool(*self.ctx, *pool);
        }
    }

    // this 3 function bellow not imported from dynamic library
    // pjsip_transaction * 	pjsip_endpt_find_tsx (SIPEndpoint *endpt, const pj_str_t *key)
    // pub fn find_tsx(&self, key: String) -> Box<*mut pjsip_transaction> {
    //     unsafe {
    //         let key = pj_str_t::from_string(key);
    //         Box::new(pjsip_endpt_find_tsx(*self.ctx, key as *const _))
    //     }
    // }
    // void 	pjsip_endpt_register_tsx (SIPEndpoint *endpt, pjsip_transaction *tsx)
    // void 	pjsip_endpt_destroy_tsx (SIPEndpoint *endpt, pjsip_transaction *tsx)

    // pjsip_endpt_create_tdata
    pub fn create_tdata(&self, p_tdata: &mut Box<*mut pjsip_tx_data> ) -> Result<(), i32> {
        unsafe {
            check_status( pjsip_endpt_create_tdata(*self.ctx, **p_tdata as *mut _))
        }
    }

    // pjsip_endpt_create_resolver
    pub fn create_resolver(&self, p_resv: &mut Box<*mut pjsip_tx_data>) -> Result<(), i32> {
        unsafe {
            check_status( pjsip_endpt_create_resolver(*self.ctx, **p_resv as *mut _))
        }
    }

    // pjsip_endpt_set_resolver
    pub fn set_resolver(&self, resv: &mut Box<*mut pj_dns_resolver>) -> Result<(), i32> {
        unsafe { check_status(pjsip_endpt_set_resolver(*self.ctx, **resv)) }
    }

    // pjsip_endpt_set_ext_resolver
    pub fn set_ext_resolver(&self, ext_res: &mut Box<*mut pjsip_ext_resolver>) -> Result<(), i32> {
        unsafe { check_status(pjsip_endpt_set_ext_resolver(*self.ctx, **ext_res)) }
    }

    // pjsip_endpt_get_resolver
    pub fn get_resolver(&self) -> Box<*mut pj_dns_resolver> {
        unsafe { Box::new(pjsip_endpt_get_resolver(*self.ctx)) }
    }

    // pjsip_endpt_resolve
    pub fn pjsip_endpt_resolve(
        &self, pool: &PJPool,
        target: &mut Box<*mut pjsip_host_info>,
        token: &mut Box<*mut c_void>,
        cb: pjsip_resolver_callback
    ) {
        unsafe { pjsip_endpt_resolve(*self.ctx, *pool.ctx, **target, **token, cb); }
    }

    // pjsip_endpt_get_tpmgr
    pub fn get_tpmgr(&self) -> Box<*mut pjsip_tpmgr> {
        unsafe { Box::new(pjsip_endpt_get_tpmgr(*self.ctx)) }
    }

    // pjsip_endpt_get_ioqueue
    pub fn get_ioqueue(&self) -> Box<*mut pj_ioqueue_t> {
        unsafe { Box::new(pjsip_endpt_get_ioqueue(*self.ctx)) }
    }

    // pjsip_endpt_acquire_transport
    pub fn acquire_transport(&self,
        type_: SIPTransportType,
        remote: &mut Box<*const pj_sockaddr_t>,
        addr_len: i32,
        sel: &mut Box<*const pjsip_tpselector>,
    ) -> Result<Box<*mut pjsip_transport>, i32> {
        unsafe {
            let tp: Box<*mut pjsip_transport> = Box::new(std::ptr::null_mut());
            let status = check_status(pjsip_endpt_acquire_transport(
                *self.ctx,
                type_.into(),
                **remote,
                addr_len,
                **sel,
                *tp as *mut _
            ));

            match status {
                Ok(()) => {
                    return Ok(tp)
                },
                Err(e) => return Err(e)
            }
        }
    }

    // pjsip_endpt_acquire_transport2
    pub fn acquire_transport2(
        &self,
        type_: SIPTransportType,
        remote: &mut Box<*const pj_sockaddr_t>,
        addr_len: i32,
        sel: &mut Box<*const pjsip_tpselector>,
        tdata: &mut Box<*mut pjsip_tx_data>
    ) -> Result<Box<*mut pjsip_transport>, i32> {
        unsafe {
            let tp: Box<*mut pjsip_transport> = Box::new(std::ptr::null_mut());
            let status = check_status(pjsip_endpt_acquire_transport2(
                *self.ctx, type_.into(), **remote, addr_len,
                **sel, **tdata, *tp as *mut _
            ));

            match status {
                Ok(()) => {
                    return Ok(tp)
                },
                Err(e) => return Err(e)
            }
        }
    }

    // pjsip_endpt_get_capability
    pub fn get_capability(&self, htype: i32, hname: String) -> Box<*const pjsip_hdr> {
        unsafe {
            let mut hname = pj_str_t::from_string(hname);
            Box::new(pjsip_endpt_get_capability(*self.ctx, htype, &mut hname as *const _))
        }
    }

    // pjsip_endpt_has_capability
    pub fn has_capability(&self, htype: i32, hname: String, token: String) -> bool {
        unsafe {
            let hname = &mut pj_str_t::from_string(hname) as *const _;
            let token = &mut pj_str_t::from_string(token) as *const _;

            check_boolean(pjsip_endpt_has_capability(*self.ctx, htype, hname, token))
        }
    }

    // pj_status_t 	pjsip_endpt_add_capability (pjsip_endpoint *endpt, pjsip_module *mod, int htype, const pj_str_t *hname, unsigned count, const pj_str_t tags[])

    // pjsip_endpt_get_request_headers
    pub fn get_request_headers(&self) -> Box<*const pjsip_hdr> {
        unsafe { Box::new(pjsip_endpt_get_request_headers(*self.ctx)) }
    }

    // pjsip_endpt_dump (pjsip_endpoint *endpt, pj_bool_t detail)
    pub fn dump(&self, detail: bool) {
        unsafe { pjsip_endpt_dump(*self.ctx, boolean_to_pjbool(detail)) }
    }

    // pjsip_endpt_atexit
    pub fn atexit(&self, func: pjsip_endpt_exit_callback) -> Result<(), i32> {
        unsafe { check_status(pjsip_endpt_atexit(*self.ctx, func)) }
    }

}
