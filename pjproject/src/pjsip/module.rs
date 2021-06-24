
use utils::{FromString, ToString};
use std::convert::TryFrom;
use super::*;


pub trait SIPModuleExt {
    // To allow chaining of modules in the endpoint.
    // pub prev: *mut pjsip_module,
    // pub next: *mut pjsip_module,

    /// Module name to identify the module.
    ///
    /// This field MUST be initialized before registering the module.
    fn set_name(&mut self, value: String);
    fn get_name(&self) -> String;
    /// Module ID. Application must initialize this field with -1 before registering the
    /// module to PJSIP. After the module is registered, this field will contain a unique ID
    /// to identify the module.
    fn set_id(&mut self, value: i32);
    fn get_id(&self) -> i32;
    /// Integer number to identify module initialization and start order with regard to other
    /// modules. Higher number will make the module gets initialized later.
    ///
    /// This field MUST be initialized before registering the module.
    fn set_priority(&mut self, value: SIPModulePriority);
    fn get_priority(&self) -> SIPModulePriority;
    /// Optional function to be called to initialize the module. This function will be called
    /// by endpoint during module registration. If the value is NULL, then it's equal to
    /// returning PJ_SUCCESS.
    ///
    /// # Parameters
    /// endpt The endpoint instance.
    fn connect_load(&mut self, callback: Option<unsafe extern "C" fn(endpt: *mut pjsip_sys::pjsip_endpoint) -> i32>);
    /// Optional function to be called to start the module. This function will be called by
    /// endpoint during module registration. If the value is NULL, then it's equal to
    /// returning PJ_SUCCESS.
    ///
    /// # Returns
    /// Module should return zero to indicate success.
    fn connect_start(&mut self, callback: Option<unsafe extern "C" fn() -> i32>);
    /// Optional function to be called to deinitialize the module before it is unloaded. This
    /// function will be called by endpoint during module unregistration. If the value is
    /// NULL, then it's equal to returning PJ_SUCCESS.
    ///
    /// # Returns
    /// Module should return PJ_SUCCESS to indicate success.
    fn connect_stop(&mut self, callback: Option<unsafe extern "C" fn() -> i32>);
    /// Optional function to be called to deinitialize the module before it is unloaded. This
    /// function will be called by endpoint during module unregistration. If the value is NULL,
    /// then it's equal to returning PJ_SUCCESS.
    ///
    /// # Parameters
    /// mod	The module.
    fn connect_unload(&mut self, callback: Option<unsafe extern "C" fn() -> i32>);
    /// Optional function to be called to process incoming request message.
    ///
    /// # Parameters
    /// rdata	The incoming message.
    ///
    /// # Returns
    ///     Module should return PJ_TRUE if it handles the request, or otherwise it should
    /// return PJ_FALSE to allow other modules to handle the request.
    fn connect_on_rx_request(&mut self, callback: Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> i32>);
    /// Optional function to be called to process incoming response message.
    ///
    /// # Parameters
    /// rdata    The incoming message.
    ///
    /// # Returns
    ///      Module should return PJ_TRUE if it handles the response, or otherwise it should
    /// return PJ_FALSE to allow other modules to handle the response.
    fn connect_on_rx_response(&mut self, callback: Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> i32>);
    /// Optional function to be called when transport layer is about to transmit outgoing request message.
    ///
    /// # Parameters
    /// tdata	The outgoing request message.
    ///
    /// # Returns
    ///     Module should return PJ_SUCCESS in all cases. If non-zero (or PJ_FALSE) is returned,
    /// the message will not be sent.
    fn connect_on_tx_request(&mut self, callback: Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> i32>);
    /// Optional function to be called when transport layer is about to transmit outgoing response message.
    ///
    /// # Parameters
    /// tdata	The outgoing response message.
    ///
    /// # Returns
    ///     Module should return PJ_SUCCESS in all cases. If non-zero (or PJ_FALSE) is returned, the message will not be sent.
    fn connect_on_tx_response(&mut self, callback: Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> i32>);
    /// Optional function to be called when this module is acting as transaction user for the specified
    /// transaction, when the transaction's state has changed.
    ///
    /// # Parameters
    /// - tsx	The transaction.
    /// - event	The event which has caused the transaction state to change.
    fn connect_on_tsx_state(&mut self, callback: Option<unsafe extern "C" fn(tsx: *mut pjsip_transaction, event: *mut pjsip_event)>);
}

impl SIPModuleExt for SIPModule {
    fn set_name(&mut self, value: String) {
        self.name = pj_str_t::from_string(value);
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_id(&mut self, value: i32) {
        self.id = value;
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn set_priority(&mut self, value: SIPModulePriority) {
        let val: u32 = value.into();
        self.priority = val as i32;
    }

    fn get_priority(&self) -> SIPModulePriority {
        SIPModulePriority::try_from(self.priority as u32)
        .expect("Error SIPModule as get_priority")
    }

    fn connect_load(&mut self, callback: Option<unsafe extern "C" fn(endpt: *mut pjsip_endpoint) -> pj_status_t>) {
        self.load = callback;
    }

    fn connect_start(&mut self, callback: Option<unsafe extern "C" fn() -> pj_status_t>) {
        self.start = callback;
    }

    fn connect_stop(&mut self, callback: Option<unsafe extern "C" fn() -> pj_status_t>) {
        self.stop = callback;
    }

    fn connect_unload(&mut self, callback: Option<unsafe extern "C" fn() -> pj_status_t>) {
        self.unload = callback;
    }

    fn connect_on_rx_request(&mut self, callback: Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> pj_bool_t>) {
        self.on_rx_request = callback;
    }

    fn connect_on_rx_response(&mut self, callback: Option<unsafe extern "C" fn(rdata: *mut pjsip_rx_data) -> pj_bool_t>) {
        self.on_rx_response = callback;
    }

    fn connect_on_tx_request(&mut self, callback: Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> pj_status_t>) {
        self.on_tx_request = callback;
    }

    fn connect_on_tx_response(&mut self, callback: Option<unsafe extern "C" fn(tdata: *mut pjsip_tx_data) -> pj_status_t>) {
        self.on_tx_response = callback;
    }

    fn connect_on_tsx_state(&mut self, callback: Option<unsafe extern "C" fn(tsx: *mut pjsip_transaction, event: *mut pjsip_event)>) {
        self.on_tsx_state = callback;
    }
}

