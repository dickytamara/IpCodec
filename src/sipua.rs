use pjproject_rs::pjsip_ua::SIPInvState;
use pjproject_rs::pjsua::{CredentialInfo, UAAccConfig, UAConfig, UALoggingConfig, UAMediaConfig};
use pjproject_rs::prelude::UAConfigExt;

use super::sipcore::*;
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
            SIP_CORE.as_mut().unwrap().start();
        }
    }

    pub fn restart(&self) {
        unsafe {
            SIP_CORE.as_mut().unwrap().restart();
        }
    }

    pub fn call(&self, call_addr: &str) {
        unsafe {
            SIP_CORE.as_mut().unwrap().call(call_addr);
        }
    }

    pub fn call_hangup(&self) {
        unsafe {
            SIP_CORE.as_mut().unwrap().call_hangup();
        }
    }

    pub fn call_answer(&self) {
        unsafe {
            SIP_CORE.as_ref().unwrap().call_answer();
        }
    }

    pub fn account_connect(&self) {
        unsafe {
            SIP_CORE.as_mut().unwrap().account_connect();
        }
    }

    pub fn set_autoanswer(&self, value: bool) {
        unsafe {
            SIP_CORE.as_mut().unwrap().auto_answer = value;
        }
    }

    pub fn set_force_lr(&self, value: bool) {
        unsafe {
            SIP_CORE.as_mut().unwrap().ua_config.set_force_lr(value);
        }
    }

    pub fn set_compact_form(&self, value: bool) {
        unsafe {
            SIP_CORE.as_mut().unwrap().compact_form = value;
        }
    }

    pub fn connect_invite<F: Fn(SIPInvState) +'static> (&self, f: F) {
        unsafe {
            SIP_CORE.as_ref().unwrap().connect_invite(f);
        }
    }

    pub fn log_config(&self) -> &mut Box<UALoggingConfig> {
        unsafe {
            &mut SIP_CORE.as_mut().unwrap().log_config
        }
    }

    pub fn ua_config(&self) -> &mut Box<UAConfig> {
        unsafe {
            &mut SIP_CORE.as_mut().unwrap().ua_config
        }
    }

    pub fn media_config(&self) -> &mut Box<UAMediaConfig> {
        unsafe {
            &mut SIP_CORE.as_mut().unwrap().media_config
        }
    }

    pub fn acc_config(&self) -> &mut Box<UAAccConfig> {
        unsafe {
            &mut SIP_CORE.as_mut().unwrap().default_acc_config
        }
    }

    pub fn acc_cred(&self) -> &mut Box<CredentialInfo> {
        unsafe {
            &mut SIP_CORE.as_mut().unwrap().default_acc_cred
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

