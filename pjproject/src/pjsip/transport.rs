


use crate::pj::PJPool;
use crate::pjsip::endpoint::SIPEndpoint;
use crate::utils::check_boolean;
use std::{convert::TryFrom, ffi::{CStr, CString}};

use pj_sys::pj_str_t;
use pjsip_sys::*;

use crate::{prelude::FromString, utils::{boolean_to_pjbool, check_status}};

use super::SIPTransportType;


pub struct SIPTransport {
    pub ctx: Box<*mut pjsip_transport>
}

impl From<Box<*mut pjsip_transport>> for SIPTransport {
    fn from(ptr: Box<*mut pjsip_transport>) -> Self {
        Self { ctx: ptr }
    }
}

pub struct SIPRxData {
    pub ctx: Box<*mut pjsip_rx_data>
}

impl From<Box<*mut pjsip_rx_data>> for SIPRxData {
    fn from(ptr: Box<*mut pjsip_rx_data>) -> Self {
        Self { ctx: ptr }
    }
}

pub struct SIPTxData {
    pub ctx: Box<*mut pjsip_tx_data>
}

impl From<Box<*mut pjsip_tx_data>> for SIPTxData {
    fn from(ptr: Box<*mut pjsip_tx_data>) -> Self {
        Self { ctx: ptr }
    }
}

pub struct SIPTpMgr {
    pub ctx: Box<*mut pjsip_tpmgr>
}

impl From<Box<*mut pjsip_tpmgr>> for SIPTpMgr {
    fn from(ptr: Box<*mut pjsip_tpmgr>) -> Self {
        Self { ctx: ptr }
    }
}

pub struct SIPTpSelector {
    pub ctx: Box<*mut pjsip_tpselector>
}

impl From<Box<*mut pjsip_tpselector>> for SIPTpSelector {
    fn from(ptr: Box<*mut pjsip_tpselector>) -> Self {
        Self { ctx: ptr }
    }
}


pub struct SIPTpFactory {
    pub ctx: Box<*mut pjsip_tpfactory>
}

impl From<Box<*mut pjsip_tpfactory>> for SIPTpFactory {
    fn from(ptr: Box<*mut pjsip_tpfactory>) -> Self {
        Self { ctx: ptr }
    }
}


impl SIPTransport {

    // pj_status_t 	pjsip_transport_register_type (unsigned tp_flag, const char *tp_name, int def_port, int *p_tp_type)
    pub fn register_type(tp_flag: u32, name: String, def_port: i32, p_tp_type: &mut i32) -> Result<(), i32> {
        unsafe {
            let name = CString::new(name.as_str()).unwrap().to_owned().as_ptr();
            check_status(pjsip_transport_register_type(
                tp_flag,
                name,
                def_port,
                p_tp_type as *mut _
            ))
        }
    }

    // pjsip_transport_type_e 	pjsip_transport_get_type_from_name (const pj_str_t *name)
    pub fn get_type_from_name(name: String) -> SIPTransportType {
        unsafe {
            let mut name = pj_str_t::from_string(name);
            SIPTransportType::try_from(pjsip_transport_get_type_from_name(&mut name as *const _) as u32).unwrap()
        }
    }

    // pjsip_transport_type_e 	pjsip_transport_get_type_from_flag (unsigned flag)
    pub fn get_type_from_flag(flag: u32) -> SIPTransportType {
        unsafe {
            SIPTransportType::try_from(pjsip_transport_get_type_from_flag(flag)).unwrap()
        }
    }

    // int 	pjsip_transport_type_get_af (pjsip_transport_type_e type)
    pub fn type_get_af(type_: SIPTransportType) -> i32 {
        unsafe {
            pjsip_transport_type_get_af(type_.into())
        }
    }

    // unsigned 	pjsip_transport_get_flag_from_type (pjsip_transport_type_e type)
    pub fn get_flag_from_type(type_: SIPTransportType) -> u32 {
        unsafe {
            pjsip_transport_get_flag_from_type(type_.into())
        }
    }

    // int 	pjsip_transport_get_default_port_for_type (pjsip_transport_type_e type)
    pub fn get_default_port_for_type(type_: SIPTransportType) -> i32 {
        unsafe {
            pjsip_transport_get_default_port_for_type(type_.into())
        }
    }

    // const char * 	pjsip_transport_get_type_name (pjsip_transport_type_e t)
    pub fn get_type_name(t: SIPTransportType) -> String {
        unsafe {
            let ret = pjsip_transport_get_type_name(t.into());
            CStr::from_ptr(ret).to_owned().into_string().unwrap()
        }
    }

    // const char * 	pjsip_transport_get_type_desc (pjsip_transport_type_e t)
    pub fn get_type_desc(t: SIPTransportType) -> String {
        unsafe {
            let ret = pjsip_transport_get_type_desc(t.into());
            CStr::from_ptr(ret).to_owned().into_string().unwrap()
        }
    }

    // TODO fix this
    // pj_status_t 	pjsip_transport_register (pjsip_tpmgr *mgr, pjsip_transport *tp)
    pub fn register(mgr: &mut SIPTpMgr, tp: &mut SIPTransport) -> Result<(), i32> {
        unsafe { check_status(
            pjsip_transport_register(*mgr.ctx, *tp.ctx)
        )}
    }

    // pj_status_t 	pjsip_transport_shutdown (pjsip_transport *tp)
    pub fn shutdown(&mut self) -> Result<(), i32> {
        unsafe { check_status(
            pjsip_transport_shutdown(*self.ctx)
        ) }
    }

    // pj_status_t 	pjsip_transport_shutdown2 (pjsip_transport *tp, pj_bool_t force)
    pub fn shutdown2(&mut self, force: bool) -> Result<(), i32> {
        unsafe {
            let force = boolean_to_pjbool(force);
            check_status(pjsip_transport_shutdown2(
                *self.ctx, force
            ))
        }
    }

    // pj_status_t 	pjsip_transport_destroy (pjsip_transport *tp)
    pub fn destroy(self) -> Result<(), i32> {
        unsafe { check_status(
            pjsip_transport_destroy(*self.ctx)
        )}
    }

    // pj_status_t 	pjsip_transport_add_ref (pjsip_transport *tp)
    pub fn add_ref(&mut self) -> Result<(), i32> {
        unsafe { check_status(
            pjsip_transport_add_ref(*self.ctx)
        )}
    }

    // pj_status_t 	pjsip_transport_dec_ref (pjsip_transport *tp)
    pub fn dec_ref(&mut self) -> Result<(), i32> {
        unsafe { check_status (
            pjsip_transport_dec_ref(*self.ctx)
        )}
    }

    // pj_status_t 	pjsip_transport_add_state_listener (pjsip_transport *tp, pjsip_tp_state_callback cb, void *user_data, pjsip_tp_state_listener_key **key)
    // pub fn add_state_listener(&mut self, cb: pjsip_tp_state_callback, user_data: Box<*mut void>, ) -> 

    // pj_status_t 	pjsip_transport_remove_state_listener (pjsip_transport *tp, pjsip_tp_state_listener_key *key, const void *user_data)

    // pj_status_t 	pjsip_transport_send (pjsip_transport *tr, pjsip_tx_data *tdata, const pj_sockaddr_t *addr, int addr_len, void *token, pjsip_tp_send_callback cb)


}

impl SIPRxData {
    // char * 	pjsip_rx_data_get_info (pjsip_rx_data *rdata)
    pub fn get_info(&mut self) -> String {
        unsafe {
            let res = pjsip_rx_data_get_info(*self.ctx);
            CStr::from_ptr(res).to_owned().into_string().unwrap()
        }
    }

    // pj_status_t 	pjsip_rx_data_clone (const pjsip_rx_data *src, unsigned flags, pjsip_rx_data **p_rdata)
    pub fn data_clone(&mut self, flags: u32) -> Result<Self, i32> {
        unsafe {
            let tx_data = Box::new(std::ptr::null_mut());

            let status = check_status(pjsip_rx_data_clone(*self.ctx, flags, *tx_data as *mut _));
            match status {
                Ok(()) => { return Ok(SIPRxData::from(tx_data)); },
                Err(e) => { return Err(e); }
            }
        }
    }

    // pj_status_t 	pjsip_rx_data_free_cloned (pjsip_rx_data *rdata)
    pub fn free_cloned(&mut self) -> Result<(), i32> {
        unsafe {
            check_status(pjsip_rx_data_free_cloned(*self.ctx))
        }
    }
}

impl SIPTxData {

    // pj_status_t 	pjsip_tx_data_create (pjsip_tpmgr *mgr, pjsip_tx_data **tdata)
    pub fn create(mgr: &mut SIPTpMgr) -> Result <SIPTxData, i32> {
        unsafe {
            let ptr: Box<*mut pjsip_tx_data> = Box::new(std::ptr::null_mut());

            let status = check_status(pjsip_tx_data_create(*mgr.ctx, *ptr as *mut _));

            match status {
                Ok(()) => { return Ok(SIPTxData::from(ptr)); },
                Err(e) => { return Err(e); }
            }
        }
    }

    // void 	pjsip_tx_data_add_ref (pjsip_tx_data *tdata)
    pub fn add_ref(&mut self) {
        unsafe { pjsip_tx_data_add_ref(*self.ctx) }
    }

    // pj_status_t 	pjsip_tx_data_dec_ref (pjsip_tx_data *tdata)
    pub fn dec_ref(&mut self) -> Result<(), i32> {
        unsafe { check_status(pjsip_tx_data_dec_ref(*self.ctx)) }
    }


    // pj_status_t 	pjsip_tx_data_encode (pjsip_tx_data *tdata)
    pub fn encode(&mut self) -> Result<(), i32> {
        unsafe { check_status(pjsip_tx_data_encode(*self.ctx)) }
    }

    // pj_bool_t 	pjsip_tx_data_is_valid (pjsip_tx_data *tdata)
    pub fn is_valid(&mut self) -> bool {
        unsafe { check_boolean(pjsip_tx_data_is_valid(*self.ctx)) }
    }

    // void 	pjsip_tx_data_invalidate_msg (pjsip_tx_data *tdata)
    pub fn invalidate_msg(&mut self) {
        unsafe { pjsip_tx_data_invalidate_msg(*self.ctx) }
    }

    // char * 	pjsip_tx_data_get_info (pjsip_tx_data *tdata)
    pub fn get_info(&mut self) -> String {
        unsafe {
            let ptr = pjsip_tx_data_get_info(*self.ctx);
            CStr::from_ptr(ptr).to_owned().into_string().unwrap()
        }
    }

    // pj_status_t 	pjsip_tx_data_set_transport (pjsip_tx_data *tdata, const pjsip_tpselector *sel)
    pub fn set_transport(&mut self, sel: &mut SIPTpSelector) -> Result<(), i32> {
        unsafe { check_status(pjsip_tx_data_set_transport(*self.ctx, *sel.ctx)) }
    }

    // pj_status_t 	pjsip_tx_data_clone (const pjsip_tx_data *src, unsigned flags, pjsip_tx_data **p_rdata)
    pub fn clone(&mut self, flags: u32) -> Result<Self, i32> {
        unsafe {
            let ptr: Box<*mut pjsip_tx_data> = Box::new(std::ptr::null_mut()); 

            let status = check_status(pjsip_tx_data_clone(*self.ctx, flags, *ptr as *mut _));

            match status {
                Ok(()) => { return Ok(SIPTxData::from(ptr)); },
                Err(e) => { return Err(e); }
            }
        }
    }

}

impl SIPTpMgr {

    // pj_ssize_t 	pjsip_tpmgr_receive_packet (pjsip_tpmgr *mgr, pjsip_rx_data *rdata)
    pub fn receive_packet(&mut self, rdata: &mut SIPRxData) -> i64 {
        unsafe { pjsip_tpmgr_receive_packet(*self.ctx, *rdata.ctx) }
    }

    // pj_status_t 	pjsip_tpmgr_register_tpfactory (pjsip_tpmgr *mgr, pjsip_tpfactory *tpf)
    pub fn register_tpfactory(&mut self, tpf: &mut SIPTpFactory) -> Result<(), i32> {
        unsafe { check_status(pjsip_tpmgr_register_tpfactory(*self.ctx, *tpf.ctx)) }
    }

    // pj_status_t 	pjsip_tpmgr_unregister_tpfactory (pjsip_tpmgr *mgr, pjsip_tpfactory *tpf)
    pub fn unregister_factory(&mut self, tpf: &mut SIPTpFactory) -> Result<(), i32> {
        unsafe { check_status(pjsip_tpmgr_unregister_tpfactory(*self.ctx, *tpf.ctx)) }
    }

    // pj_status_t 	pjsip_tpmgr_create (pj_pool_t *pool, pjsip_endpoint *endpt, pjsip_rx_callback rx_cb, pjsip_tx_callback tx_cb, pjsip_tpmgr **p_mgr)
    pub fn create(pool: &mut PJPool, endpt: SIPEndpoint, rx_cb: pjsip_rx_callback, tx_cb: pjsip_tx_callback) -> Result<Self, i32> {
        unsafe {
            let ptr: Box<*mut pjsip_tpmgr> = Box::new(std::ptr::null_mut());

            let status = check_status(pjsip_tpmgr_create(
                    *pool.ctx,
                    *endpt.ctx,
                    rx_cb,
                    tx_cb,
                    *ptr as *mut _
                ));

            match status {
                Ok(()) => { return Ok(SIPTpMgr::from(ptr)); },
                Err(e) => { return Err(e); }
            }
        }
    }

    // pj_status_t 	pjsip_tpmgr_find_local_addr (pjsip_tpmgr *tpmgr, pj_pool_t *pool, pjsip_transport_type_e type, const pjsip_tpselector *sel, pj_str_t *ip_addr, int *port)
    pub fn find_local_addr(&mut self, pool: &mut PJPool, type_: SIPTransportType, sel: &mut SIPTpSelector, ip_addr: String, port: &mut i32) -> Result<(), i32> {
        unsafe { 

            let mut ip_addr = pj_str_t::from_string(ip_addr);

            check_status( pjsip_tpmgr_find_local_addr(
                *self.ctx, *pool.ctx, type_.into(), *sel.ctx, &mut ip_addr as *mut _, port as *mut _
            ))
        }
    }

    // void 	pjsip_tpmgr_fla2_param_default (pjsip_tpmgr_fla2_param *prm)
    pub fn fla2_param_default(prm: &mut Box<*mut pjsip_tpmgr_fla2_param>) {
        unsafe { pjsip_tpmgr_fla2_param_default(**prm) }
    }

    // pj_status_t 	pjsip_tpmgr_find_local_addr2 (pjsip_tpmgr *tpmgr, pj_pool_t *pool, pjsip_tpmgr_fla2_param *prm)
    pub fn find_local_addr2(&mut self, pool: &mut PJPool, prm: &mut Box<*mut pjsip_tpmgr_fla2_param>) -> Result<(), i32> {
        unsafe { check_status(pjsip_tpmgr_find_local_addr2(*self.ctx, *pool.ctx, **prm)) }
    }

    // unsigned 	pjsip_tpmgr_get_transport_count (pjsip_tpmgr *mgr)
    pub fn get_trasport_count(&mut self) -> u32 {
        unsafe { pjsip_tpmgr_get_transport_count(*self.ctx) }
    }

    // pj_status_t 	pjsip_tpmgr_destroy (pjsip_tpmgr *mgr)
    pub fn destroy(&mut self) -> Result<(), i32> {
        unsafe { check_status(pjsip_tpmgr_destroy(*self.ctx)) }
    }

    // void 	pjsip_tpmgr_dump_transports (pjsip_tpmgr *mgr)
    pub fn dump_transports(&mut self) {
        unsafe { pjsip_tpmgr_dump_transports(*self.ctx) }
    }

    // pj_status_t 	pjsip_tpmgr_acquire_transport (pjsip_tpmgr *mgr, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_transport **tp)
    pub fn acquire_transport(&mut self, type_: SIPTransportType, remote: &mut Box<*mut pj_sys::pj_sockaddr_t>, addr_len: i32, sel: &mut SIPTpSelector) -> Result<SIPTransport, i32> {
        unsafe { 
            let ptr: Box<*mut pjsip_transport> = Box::new(std::ptr::null_mut());

            let status = check_status( pjsip_tpmgr_acquire_transport(
                    *self.ctx, type_.into(), **remote, addr_len, *sel.ctx, *ptr as *mut _
                ));

            match status {
                Ok(()) => { return Ok(SIPTransport::from(ptr)); },
                Err(e) => { return Err(e); }
            }
        }
    }

    // pj_status_t 	pjsip_tpmgr_acquire_transport2 (pjsip_tpmgr *mgr, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_tx_data *tdata, pjsip_transport **tp)


    // pj_status_t 	pjsip_tpmgr_send_raw (pjsip_tpmgr *mgr, pjsip_transport_type_e tp_type, const pjsip_tpselector *sel, pjsip_tx_data *tdata, const void *raw_data, pj_size_t data_len, const pj_sockaddr_t *addr, int addr_len, void *token, pjsip_tp_send_callback cb)

    // pj_status_t 	pjsip_tpmgr_set_state_cb (pjsip_tpmgr *mgr, pjsip_tp_state_callback cb)

    // pjsip_tp_state_callback 	pjsip_tpmgr_get_state_cb (const pjsip_tpmgr *mgr)

    // pj_status_t 	pjsip_tpmgr_set_drop_data_cb (pjsip_tpmgr *mgr, pjsip_tp_on_rx_dropped_cb cb)
}

impl SIPTpSelector {
    // void 	pjsip_tpselector_add_ref (pjsip_tpselector *sel)
    pub fn add_ref(&mut self) {
        unsafe { pjsip_tpselector_add_ref(*self.ctx) }
    }

    // void 	pjsip_tpselector_dec_ref (pjsip_tpselector *sel)
    pub fn dec_ref(&mut self) {
        unsafe { pjsip_tpselector_dec_ref(*self.ctx) }
    }
}


