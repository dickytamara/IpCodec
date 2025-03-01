use pjproject_rs::{pjsip, pjsip_ua::SIPInvState, prelude::*};
use pjproject_rs::prelude::AutoDefault;
use pjproject_rs::pj::*;
use pjproject_rs::pjsip::*;
use pjproject_rs::pjsua::*;
use pjproject_rs::pjmedia::*;

use pjproject_rs::pjsua;

use std::{cell::RefCell, rc::Rc};

// thread safe use Arc<Mutex<SIPCore>>
pub static mut SIP_CORE: Option<SIPCore> = None;
pub static mut CURRENT_CALL: Option<i32> = None;

pub struct SIPCore {
    pub ua_config: Box<UAConfig>,
    pub log_config: Box<UALoggingConfig>,
    pub media_config: Box<UAMediaConfig>,
    pub default_transport_config: UATransportConfig,
    pub default_rtp_config: UATransportConfig,
    pub default_acc_config: Box<UAAccConfig>,
    pub default_acc_cred: Box<CredentialInfo>,
    pub default_acc: Option<UAAccount>,
    pub default_call_setting: UACallSetting,
    def_pool: Option<PJPool>, // this pool mostlly unused.
    module: SIPModule,
    no_udp: bool,
    no_tcp: bool,
    use_ipv6: bool,
    transport_udp: Option<UATransport>,
    transport_tcp: Option<UATransport>,
    transport_udp6: Option<UATransport>,
    transport_tcp6: Option<UATransport>,
    redir_op: SIPRedirectOp,
    pub input_dev: i32,
    pub output_dev: i32,
    current_call: Option<UACall>,
    events: Rc<RefCell<SIPCoreEvents>>,
    pub auto_answer: bool,
    pub compact_form: bool,
}

// struct to hold invite event function
struct SIPCoreEvents {
    invite: Box<dyn Fn(SIPInvState)>,
    incoming_call: Box<dyn FnMut()>
}

impl SIPCoreEvents {
    fn new() -> SIPCoreEvents {
        SIPCoreEvents {
            invite: Box::new(|s| {}),
            incoming_call: Box::new(|| {})
        }
    }
}


impl SIPCore {

    pub fn new() -> Self {
        // create instance
        pjsua::create().unwrap();
        let mut sipcore = Self {
            ua_config: Box::new(UAConfig::default()),
            log_config: Box::new(UALoggingConfig::default()),
            media_config: Box::new(UAMediaConfig::default()),
            default_transport_config: UATransportConfig::default(),
            default_rtp_config: UATransportConfig::default(),
            default_acc_config: Box::new(UAAccConfig::default()),
            default_acc_cred: Box::new(CredentialInfo::new()),
            default_acc: None,
            default_call_setting: UACallSetting::default(),
            def_pool: None,
            module: SIPModule::new(),
            transport_udp: None,
            transport_tcp: None,
            transport_udp6: None,
            transport_tcp6: None,
            redir_op: SIPRedirectOp::AcceptReplace,
            input_dev: -1,
            output_dev: -2,
            current_call: None,
            auto_answer: false,
            events: Rc::new(RefCell::new(SIPCoreEvents::new())),
            compact_form: false,
            no_udp: false,
            no_tcp: false,
            use_ipv6: false,
        };

        // default ua config
        sipcore.ua_config.set_max_calls(1);
        sipcore.ua_config.set_force_lr(true);
        sipcore.ua_config.set_user_agent(String::from("IpCodec"));

        // set default event
        sipcore.ua_config.cb.on_call_state = Some(on_call_state);
        sipcore.ua_config.cb.on_call_media_state = Some(on_call_media_state);
        sipcore.ua_config.cb.on_incoming_call = Some(on_incoming_call);
        sipcore.ua_config.cb.on_reg_state = Some(on_reg_state);


        // default media config
        sipcore.media_config.set_channel_count(ConfigChannel::Stereo);
        sipcore.media_config.set_clock_rate(ClockRate::ClockRate48000);
        sipcore.media_config.set_snd_clock_rate(ClockRate::ClockRate48000);
        sipcore.media_config.set_quality(EncodingQuality::Level10);
        sipcore.media_config.set_ec_options(MediaEchoFlag::Default);
        sipcore.media_config.set_ec_tail_len(0);

        // default log level
        sipcore.log_config.set_level(0);

        // default transport config
        sipcore.default_transport_config.set_port(5060);
        sipcore.default_rtp_config.set_port(4000);

        sipcore.default_call_setting.aud_cnt = 1;
        sipcore.default_call_setting.vid_cnt = 0;

        sipcore
    }

    pub fn start(&mut self) {

        // check if pjsua has created or not.
        if pjsua::get_state() != UAState::Created {
            pjsua::create().unwrap();
        }

        // default application pool
        self.def_pool = Some(pjsua::pool_create("ipcodec_app"));

        // register sub module for unhandeled error
        // in version 2.10 we dont need to register any module

        // self.module.set_priority(SIPModulePriority::Application);
        // self.module.set_name(String::from("mod-default-handler"));
        // self.module.connect_on_rx_request(Some(on_rx_request));
        // let endpt = pjsua::get_pjsip_endpt();
        // endpt.register_module(&mut self.module).unwrap();
        //SIPModule::register(&mut self.module);

        pjsua::init(
            &mut self.ua_config,
            &mut self.log_config,
            &mut self.media_config)
        .unwrap();

        // check if setting not have at least 1 protocol
        if self.no_udp & self.no_tcp {
            self.no_udp = false;
        }

        // Initialize UDP Transport and local account
        if !self.no_udp {
            let tp = UATransport::new(SIPTransportType::Udp, &self.default_transport_config);
            let local = UAAccount::new_local(&tp, true).unwrap();
            let mut config = local.get_config().unwrap();

            config.rtp_cfg.set_port(self.default_rtp_config.get_port());
            local.modify(&mut config).unwrap();
            local.set_online_status(true).unwrap();

            self.transport_udp = Some(tp);
        }

        // initialize TCP transport and local account
        if !self.no_tcp {
            let tp = UATransport::new(SIPTransportType::Tcp, &self.default_transport_config);
            let local = UAAccount::new_local(&tp, true).unwrap();
            let mut config = local.get_config().unwrap();

            config.rtp_cfg.set_port(self.default_rtp_config.get_port());
            local.modify(&mut config).unwrap();
            local.set_online_status(true).unwrap();

            self.transport_tcp = Some(tp);
        }

        // initialize UDPv6 and local account
        if self.use_ipv6 & !self.no_udp {
            let tp = UATransport::new(SIPTransportType::UdpV6, &self.default_transport_config);
            let local = UAAccount::new_local(&tp, true).unwrap();
            let mut config = local.get_config().unwrap();

            config.rtp_cfg.set_port(self.default_rtp_config.get_port());
            config.set_ipv6_media_use(self.use_ipv6);
            local.modify(&mut config).unwrap();
            local.set_online_status(true).unwrap();

            self.transport_udp6 = Some(tp);
        }

        // initialize TCPv6 and local account
        if self.use_ipv6 & !self.no_tcp {
            let tp = UATransport::new(SIPTransportType::TcpV6, &self.default_transport_config);
            let local = UAAccount::new_local(&tp, true).unwrap();
            let mut config = local.get_config().unwrap();

            config.rtp_cfg.set_port(self.default_rtp_config.get_port());
            config.set_ipv6_media_use(self.use_ipv6);
            local.modify(&mut config).unwrap();
            local.set_online_status(true).unwrap();

            self.transport_tcp6 = Some(tp);
        }

        self.account_connect();

        // self.media_config.init();
        UASound::default().set_snd_dev(self.input_dev, self.output_dev).unwrap();
        pjsua::start().unwrap();
    }

    pub fn restart(&mut self) {

        if self.def_pool.is_some() {
            self.def_pool.as_ref().unwrap().release();
            self.def_pool = None
        }

        self.stop();
        self.start();
    }

    pub fn stop(&self) {
        pjsua::destroy().unwrap();
    }

    pub fn account_connect(&mut self) {
        // remove default current account
        if self.default_acc.is_some() {
            self.default_acc = None;
        }

        if ! self.default_acc_config.get_id().is_empty() {
            let cred = self.default_acc_cred.clone();
            self.default_acc_config.set_cred_info(*cred).unwrap();
            let acc = UAAccount::new(&self.default_acc_config, true).unwrap();
            self.default_acc = Some(acc);
        }
    }

    pub fn call(&mut self, call_addr: &str) {
        let account = UAAccount::default();
        if account.is_valid() {
            // Some(&mut self.default_call_setting.borrow_mut())
            let mut msg_data = Box::new(UAMsgData::default());
            match account.call(call_addr.to_string(),
            Some(&mut self.default_call_setting),
            Some(&mut msg_data)
        ) {
                Ok(call) => self.current_call = Some(call),
                Err(e) => ()
            }
        }
    }

    pub fn call_hangup(&mut self) {
        match self.current_call {
            Some(ref call) => {
                if call.is_active() {
                    call.hangup(SIPStatusCode::Ok, None, None).unwrap();
                } else {
                    self.current_call = None;
                }
            },
            None => ()
        }
    }

    pub fn call_answer(&self) {
        if self.current_call.is_some() {
            self.current_call
            .unwrap()
            .answer(SIPStatusCode::Ok, None, None).unwrap();
        }
    }

    pub fn on_call_audio_state(&self, ci: &UACallInfo, mi: u32, has_error: &mut bool) {

        let media = &ci.media[mi as usize];

        match media.get_status() {
            CallMediaStatus::None => {}
            CallMediaStatus::LocalHold => {}
            CallMediaStatus::Error => {}
            CallMediaStatus::Active |
            CallMediaStatus::RemoteHold => {
                unsafe {
                    let call_conf_slot = media.stream.aud.as_ref().conf_slot;

                    for call in UACall::enum_calls().unwrap().iter() {
                        if ci.get_id() == call.get_info().unwrap().get_id() { continue; }
                        if call.has_media() {
                            UAConf::connect(call_conf_slot, call.get_info().unwrap().get_id()).unwrap();
                            UAConf::connect(call.get_info().unwrap().get_id(), call_conf_slot).unwrap();
                        }
                    }

                    UAConf::connect(call_conf_slot, 0).unwrap();
                    UAConf::connect(0, call_conf_slot).unwrap();
                }
            }
        };
    }

    pub fn callback_on_call_state(&mut self, call: UACall, e: *mut SIPEvent) {
        // call event for non internal
        (self.events.borrow().invite)(call.get_info().unwrap().get_state());

        if call.get_info().unwrap().get_state() == SIPInvState::Disconnected {
            match self.current_call {
                Some(current_call) => {

                    let current_call_id: i32 = current_call.into();
                    let call_id: i32 = call.into();

                    if current_call_id == call_id {
                        self.current_call = None;
                    } else {
                        UACall::from(call_id)
                        .hangup(SIPStatusCode::Null, None, None)
                        .unwrap();
                    }
                },
                None => (),
            }
        } else {
            if self.current_call.is_some() {
                if !self.current_call.unwrap().is_active() {
                    self.current_call = None;
                }
            }
        }
    }

    pub fn callback_on_call_media_state(&self, call: UACall) {

        let mut has_error = false;
        let call_info = call.get_info().unwrap();

        for mi in 0..call_info.media_cnt {

            let call_media_info = &call_info.media[mi as usize];
            match call_media_info.get_type_() {
                MediaType::Audio => {
                    self.on_call_audio_state(&call_info, mi, &mut has_error);
                },
                _ => has_error = true
            }

            // println!("sipua.rs Call {} media {} [type={}], status is {}",
            //     call_info.id, mi, media_type, status_name);
        }

        if has_error {
            let reason = format!("Media failed");
            call.hangup(SIPStatusCode::IntenalServerError, Some(reason), None).unwrap();
        }
    }

    pub fn callback_on_incomming_call(&mut self, account: UAAccount, call: UACall, rdata: SIPRxData) {

        self.current_call = Some(call);
        (self.events.borrow().invite)(SIPInvState::Incoming);

        let mut opt = UACallSetting::default();
        if self.auto_answer {
            call.answer2(&mut opt, SIPStatusCode::Ok, None, None).unwrap();
        }
    }

    pub fn callback_on_reg_state(&self, acc: UAAccount) {
        // println!("reg_state {}", acc_id);
    }

}

pub trait SIPCoreEventsExt {
    fn connect_invite<F: Fn(SIPInvState) + 'static> (&self, f: F);
    fn connect_incoming_call<F: Fn() + 'static> (&self, f: F);
}

// trait to hold external event
impl SIPCoreEventsExt for SIPCore {

    fn connect_invite <F: Fn(SIPInvState) + 'static> (&self, f: F) {
        self.events.borrow_mut().invite = Box::new(f);
    }

    fn connect_incoming_call <F: Fn() + 'static> (&self, f: F) {
        self.events.borrow_mut().incoming_call = Box::new(f);
    }
}

// on_call_state(pjsua_call_id, pjsip_event)
unsafe extern "C" fn on_call_state(call_id: i32, e: *mut SIPEvent) {
    SIP_CORE.as_mut().unwrap()
    .callback_on_call_state(UACall::from(call_id), e);
}

// on_incoming_call(acc_id, call_id, rdata)
unsafe extern "C" fn on_incoming_call( acc_id: i32, call_id: i32, rdata: *mut pjproject_rs::pjsip::pjsip_rx_data) {
    SIP_CORE.as_mut().unwrap()
    .callback_on_incomming_call(UAAccount::from(acc_id), UACall::from(call_id), SIPRxData::from(Box::new(rdata)))
}

// on_call_media_state(call_id)
unsafe extern "C" fn on_call_media_state(call_id: i32) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_call_media_state(UACall::from(call_id));
}

/// on_reg_state(acc_id)
unsafe extern "C" fn on_reg_state(acc_id: i32) {
    SIP_CORE.as_ref().unwrap()
    .callback_on_reg_state(UAAccount::from(acc_id));
}
