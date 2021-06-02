

use std::ffi::CString;

use pjlib_util_sys::pj_dns_resolver;
use utils::{FromString, ToString};

use crate::utils::{boolean_to_pjbool, check_boolean, check_status};

use super::*;


pub struct SIPEndpoint { ctx: Box<*mut pjsip_sys::pjsip_endpoint> }

impl From<*mut pjsip_endpoint> for SIPEndpoint {
    fn from(ptr: *mut pjsip_endpoint) -> Self {
        Self {
            ctx: Box::new(ptr),
        }
    }
}

impl SIPEndpoint {

    pub fn new(pf: &mut Box<*mut pj_pool_factory>, name: String) -> Self {
        SIPEndpoint::create(pf, name).unwrap()
    }

    /// pjsip_endpt_create
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
                    return Ok(SIPEndpoint::from(*endpt))
                }
                Err(e) => return Err(e)
            }
        }
    }

    /// pjsip_endpt_destroy
    pub fn endpt_destroy(self) {
        unsafe { pjsip_endpt_destroy(*self.ctx); }
    }

    // pjsip_endpt_name
    pub fn endpt_name(&self) -> String {
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

    // pj_status_t 	pjsip_endpt_schedule_timer_w_grp_lock (SIPEndpoint *endpt, pj_timer_entry *entry, const pj_time_val *delay, int id_val, pj_grp_lock_t *grp_lock)

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
        unsafe {
            Box::new(pjsip_endpt_get_timer_heap(*self.ctx))
        }
    }

    pub fn endpt_register_module(&self, module: &mut pjsip_module) -> Result<(), i32> {
        unsafe { check_status( pjsip_endpt_register_module(*self.ctx, module as *mut _)) }
    }

    pub fn endpt_unregister_module(&self, module: &mut pjsip_module) -> Result<(), i32> {
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
    pub fn endpt_create_tdata(&self, p_tdata: &mut Box<*mut pjsip_tx_data> ) -> Result<(), i32> {
        unsafe {
            check_status( pjsip_endpt_create_tdata(*self.ctx, **p_tdata as *mut _))
        }
    }

    // pjsip_endpt_create_resolver
    pub fn endpt_create_resolver(&self, p_resv: &mut Box<*mut pjsip_tx_data>) -> Result<(), i32> {
        unsafe {
            check_status( pjsip_endpt_create_resolver(*self.ctx, **p_resv as *mut _))
        }
    }

    // pj_status_t 	pjsip_endpt_set_resolver (SIPEndpoint *endpt, pj_dns_resolver *resv)
    pub fn endpt_set_resolver(&self, resv: &mut Box<*mut pj_dns_resolver>) -> Result<(), i32> {
        unsafe { check_status(pjsip_endpt_set_resolver(*self.ctx, **resv)) }
    }

    // pj_status_t 	pjsip_endpt_set_ext_resolver (SIPEndpoint *endpt, pjsip_ext_resolver *ext_res)
    pub fn endpt_set_ext_resolver(&self, ext_res: &mut Box<*mut pjsip_ext_resolver>) -> Result<(), i32> {
        unsafe { check_status(pjsip_endpt_set_ext_resolver(*self.ctx, **ext_res)) }
    }

    // pj_dns_resolver * 	pjsip_endpt_get_resolver (pjsip_endpoint *endpt)
    pub fn endpt_get_resolver(&self) -> Box<*mut pj_dns_resolver> {
        unsafe { Box::new(pjsip_endpt_get_resolver(*self.ctx)) }
    }

    // void 	pjsip_endpt_resolve (pjsip_endpoint *endpt, pj_pool_t *pool, pjsip_host_info *target, void *token, pjsip_resolver_callback *cb)

    // pjsip_tpmgr * 	pjsip_endpt_get_tpmgr (pjsip_endpoint *endpt)
    pub fn endpt_get_tpmgr(&self) -> Box<*mut pjsip_tpmgr> {
        unsafe { Box::new(pjsip_endpt_get_tpmgr(*self.ctx)) }
    }

    // pj_ioqueue_t * 	pjsip_endpt_get_ioqueue (pjsip_endpoint *endpt)
    pub fn endpt_get_ioqueue(&self) -> Box<*mut pj_ioqueue_t> {
        unsafe { Box::new(pjsip_endpt_get_ioqueue(*self.ctx)) }
    }

    // pj_status_t 	pjsip_endpt_acquire_transport (pjsip_endpoint *endpt, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_transport **p_tp)
    // pj_status_t 	pjsip_endpt_acquire_transport2 (pjsip_endpoint *endpt, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_tx_data *tdata, pjsip_transport **p_tp)

    // const pjsip_hdr * 	pjsip_endpt_get_capability (pjsip_endpoint *endpt, int htype, const pj_str_t *hname)
    pub fn endpt_get_capability(&self, htype: i32, hname: String) -> Box<*const pjsip_hdr> {
        unsafe {
            let mut hname = pj_str_t::from_string(hname);
            Box::new(pjsip_endpt_get_capability(*self.ctx, htype, &mut hname as *const _))
        }
    }

    // pj_bool_t 	pjsip_endpt_has_capability (pjsip_endpoint *endpt, int htype, const pj_str_t *hname, const pj_str_t *token)
    pub fn endpt_has_capability(&self, htype: i32, hname: String, token: String) -> bool {
        unsafe {
            let hname = &mut pj_str_t::from_string(hname) as *const _;
            let token = &mut pj_str_t::from_string(token) as *const _;

            check_boolean(pjsip_endpt_has_capability(*self.ctx, htype, hname, token))
        }
    }

    // pj_status_t 	pjsip_endpt_add_capability (pjsip_endpoint *endpt, pjsip_module *mod, int htype, const pj_str_t *hname, unsigned count, const pj_str_t tags[])

    // pjsip_endpt_get_request_headers
    pub fn endpt_get_request_headers(&self) -> Box<*const pjsip_hdr> {
        unsafe { Box::new(pjsip_endpt_get_request_headers(*self.ctx)) }
    }

    // pjsip_endpt_dump (pjsip_endpoint *endpt, pj_bool_t detail)
    pub fn endpt_dump(&self, detail: bool) {
        unsafe { pjsip_endpt_dump(*self.ctx, boolean_to_pjbool(detail)) }
    }

    // pjsip_endpt_atexit
    pub fn endpt_atexit(&self, func: pjsip_endpt_exit_callback) -> Result<(), i32> {
        unsafe { check_status(pjsip_endpt_atexit(*self.ctx, func)) }
    }

}
