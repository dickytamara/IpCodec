

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

    // pj_status_t 	pjsip_transport_remove_state_listener (pjsip_transport *tp, pjsip_tp_state_listener_key *key, const void *user_data)

    // pj_status_t 	pjsip_transport_send (pjsip_transport *tr, pjsip_tx_data *tdata, const pj_sockaddr_t *addr, int addr_len, void *token, pjsip_tp_send_callback cb)


}

impl SIPRxData {
// char * 	pjsip_rx_data_get_info (pjsip_rx_data *rdata)

// pj_status_t 	pjsip_rx_data_clone (const pjsip_rx_data *src, unsigned flags, pjsip_rx_data **p_rdata)

// pj_status_t 	pjsip_rx_data_free_cloned (pjsip_rx_data *rdata)
}

impl SIPTxData {

    // pj_status_t 	pjsip_tx_data_create (pjsip_tpmgr *mgr, pjsip_tx_data **tdata)

    // void 	pjsip_tx_data_add_ref (pjsip_tx_data *tdata)

    // pj_status_t 	pjsip_tx_data_dec_ref (pjsip_tx_data *tdata)

    // pj_status_t 	pjsip_tx_data_encode (pjsip_tx_data *tdata)

    // pj_bool_t 	pjsip_tx_data_is_valid (pjsip_tx_data *tdata)

    // void 	pjsip_tx_data_invalidate_msg (pjsip_tx_data *tdata)

    // char * 	pjsip_tx_data_get_info (pjsip_tx_data *tdata)

    // pj_status_t 	pjsip_tx_data_set_transport (pjsip_tx_data *tdata, const pjsip_tpselector *sel)

    // pj_status_t 	pjsip_tx_data_clone (const pjsip_tx_data *src, unsigned flags, pjsip_tx_data **p_rdata)

}

impl SIPTpMgr {
    // pj_ssize_t 	pjsip_tpmgr_receive_packet (pjsip_tpmgr *mgr, pjsip_rx_data *rdata)

    // pj_status_t 	pjsip_tpmgr_register_tpfactory (pjsip_tpmgr *mgr, pjsip_tpfactory *tpf)

    // pj_status_t 	pjsip_tpmgr_unregister_tpfactory (pjsip_tpmgr *mgr, pjsip_tpfactory *tpf)

    // pj_status_t 	pjsip_tpmgr_create (pj_pool_t *pool, pjsip_endpoint *endpt, pjsip_rx_callback rx_cb, pjsip_tx_callback tx_cb, pjsip_tpmgr **p_mgr)

    // pj_status_t 	pjsip_tpmgr_find_local_addr (pjsip_tpmgr *tpmgr, pj_pool_t *pool, pjsip_transport_type_e type, const pjsip_tpselector *sel, pj_str_t *ip_addr, int *port)

    // void 	pjsip_tpmgr_fla2_param_default (pjsip_tpmgr_fla2_param *prm)

    // pj_status_t 	pjsip_tpmgr_find_local_addr2 (pjsip_tpmgr *tpmgr, pj_pool_t *pool, pjsip_tpmgr_fla2_param *prm)

    // unsigned 	pjsip_tpmgr_get_transport_count (pjsip_tpmgr *mgr)

    // pj_status_t 	pjsip_tpmgr_destroy (pjsip_tpmgr *mgr)

    // void 	pjsip_tpmgr_dump_transports (pjsip_tpmgr *mgr)

    // pj_status_t 	pjsip_tpmgr_acquire_transport (pjsip_tpmgr *mgr, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_transport **tp)

    // pj_status_t 	pjsip_tpmgr_acquire_transport2 (pjsip_tpmgr *mgr, pjsip_transport_type_e type, const pj_sockaddr_t *remote, int addr_len, const pjsip_tpselector *sel, pjsip_tx_data *tdata, pjsip_transport **tp)

    // pj_status_t 	pjsip_tpmgr_send_raw (pjsip_tpmgr *mgr, pjsip_transport_type_e tp_type, const pjsip_tpselector *sel, pjsip_tx_data *tdata, const void *raw_data, pj_size_t data_len, const pj_sockaddr_t *addr, int addr_len, void *token, pjsip_tp_send_callback cb)

    // pj_status_t 	pjsip_tpmgr_set_state_cb (pjsip_tpmgr *mgr, pjsip_tp_state_callback cb)

    // pjsip_tp_state_callback 	pjsip_tpmgr_get_state_cb (const pjsip_tpmgr *mgr)

    // pj_status_t 	pjsip_tpmgr_set_drop_data_cb (pjsip_tpmgr *mgr, pjsip_tp_on_rx_dropped_cb cb)
}

impl SIPTpSelector {
    // void 	pjsip_tpselector_add_ref (pjsip_tpselector *sel)

    // void 	pjsip_tpselector_dec_ref (pjsip_tpselector *sel)
}


