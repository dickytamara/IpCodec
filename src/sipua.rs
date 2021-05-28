use pjproject::pjsua::{CredentialInfo, UAAccConfig, UAConfig, UALoggingConfig, UAMediaConfig};

use super::sipcore::*;

use std::cell::RefMut;
use std::ops::Drop;

// SIPUserAgent
#[derive(Clone)]
pub struct SIPUserAgent {}


impl SIPUserAgent {

    // create sip user sip user agent with default ivalue
    pub fn new() -> Self {
        unsafe {
            SIP_CORE = Some(SIPCore::new());
        }

        Self {}
    }

    /// start user agent
    pub fn start(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().init();
        }
    }

    pub fn restart(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().restart();
        }
    }

    pub fn get_context(&self) -> &SIPCore {
        unsafe {
            SIP_CORE.as_ref().unwrap()
        }
    }

    pub fn log_config(&self) -> RefMut<UALoggingConfig> {
        unsafe {
            SIP_CORE.as_ref().unwrap().log_config.borrow_mut()
        }
    }

    pub fn ua_config(&self) -> RefMut<UAConfig> {
        unsafe {
            SIP_CORE.as_ref().unwrap().ua_config.borrow_mut()
        }
    }

    pub fn media_config(&self) -> RefMut<UAMediaConfig> {
        unsafe {
            SIP_CORE.as_ref().unwrap().media_config.borrow_mut()
        }
    }

    pub fn acc_config(&self) -> RefMut<Box<UAAccConfig>> {
        unsafe {
            SIP_CORE.as_ref().unwrap().default_acc_config.borrow_mut()
        }
    }

    pub fn acc_cred(&self) -> RefMut<CredentialInfo> {
        unsafe {
            SIP_CORE.as_ref().unwrap().default_acc_cred.borrow_mut()
        }
    }

}


impl Drop for SIPUserAgent {
    fn drop(&mut self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().stop();
        }
    }
}

