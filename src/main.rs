#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
extern crate pjproject;

extern crate gtk;
extern crate gio;
extern crate glib;
extern crate gdk;

extern crate systemstat;
extern crate configparser;

// gui module
mod helper;
mod about;
mod audio_line;
mod codec;
mod dialpad;
mod maintab;
mod header;
mod status;
mod account;
mod settings;
mod settings_audio;
mod settings_ua;
mod settings_ice;
mod settings_media;
mod settings_stun;
mod settings_turn;
mod settings_proxy;
mod settings_dns;

// sipua module
mod sipua;
mod sipcore;

mod settings_tls;


use gtk::prelude::*;
use gio::prelude::*;
use helper::{HelperFileSettings, application_config_path};
use pjproject::{pjnath::{IceSessTrickle, TurnTpType}, pjsip_ua::SIPInvState, pjsua::{CredentialInfoType, ua::CredentialInfoExt}, prelude::*};
use pjproject::pjsua::media::UASound;
use systemstat::Duration;

use pjproject::pj;
// use pjnath_sys::*;

use std::{cell::RefCell, env, rc::Rc, thread};
use std::include_str;


use gtk::{Application, Builder};

use dialpad::{DialpadWidget};
use audio_line::AudioLineWidget;
use maintab::MaintabWidget;
use header::HeaderWidget;
use status::StatusbarWidget;
use account::AccountWidget;
use codec::CodecWidget;
use settings::{SettingsCurrentActivePage, SettingsWidget};
// use helper::{HelperFileSettings, application_config_path};

// use sipua::SIPInviteState;
use sipua::*;

use crate::dialpad::CallButtonState;
enum SignalLevel { Level( (u32, u32, u32, u32)) }

/// update receive transmit level bar
fn thread_update_level_bar(rx_widget_clone: AudioLineWidget, tx_widget_clone: AudioLineWidget) {

    // sender, receiver more clear to read
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    thread::spawn(move || {
        let mut desc= [0i64;64usize];

        // register main thread
        if !pj::thread::PJThread::thread_is_registered() {
            pj::thread::PJThread::thread_register(None, &mut desc).unwrap();
        }

        loop {
            thread::sleep(Duration::from_millis(40));

            let _ = sender.send(SignalLevel::Level(UASound::default().get_msignal_level().unwrap()));
        }
    });

    // TODO: fix thread attach.
    receiver.attach(None, move |level| {
        match level {
            SignalLevel::Level((tx_l,tx_r, rx_l, rx_r)) => {
                rx_widget_clone.set_level_bar(rx_l, rx_r);
                tx_widget_clone.set_level_bar(tx_l, tx_r);
            }
        }

        glib::Continue(true)
    });
}

// calback audio line transmit receive
fn callback_audio_line_widget(rx_level: Rc<RefCell<i32>>, tx_level: Rc<RefCell<i32>>, rx_widget: &AudioLineWidget, tx_widget: &AudioLineWidget) {

    // update output device list
    for item in UASound::enum_aud_devs().unwrap().iter() {
        let name = item.get_name();
        rx_widget.add_device_text(&name.unwrap());
    }

    // update input device list
    for item in UASound::enum_aud_devs().unwrap().iter() {
        let name = item.get_name();
        tx_widget.add_device_text(&name.unwrap());
    }

    // slider level change
    let level = rx_level.clone();
    rx_widget.on_scale_changed_value (move |v| {
        level.replace(v);
        UASound::default().adjust_rx_level((level.borrow().clone() as f32) / 100_f32).unwrap();
    });

    let level = tx_level.clone();
    tx_widget.on_scale_changed_value (move |v| {
        level.replace(v);
        UASound::default().adjust_tx_level((level.borrow().clone() as f32) / 100_f32).unwrap();
    });

    let level = rx_level.clone();
    rx_widget.on_button_mute_clicked(move |state| {
        if state {
            UASound::default().adjust_rx_level(0_f32).unwrap();
        } else {
            UASound::default()
            .adjust_rx_level((level.borrow().clone() as f32) / 100_f32).unwrap();
        }
    });

    let level = tx_level.clone();
    tx_widget.on_button_mute_clicked(move |state| {
        if state {
            UASound::default().adjust_tx_level(0_f32).unwrap();
        } else {
            UASound::default().adjust_tx_level((level.borrow().clone() as f32) / 100_f32).unwrap();
        }
    });
}

// callback dialpad widget
fn callback_dialpad_widget(sipua: &mut SIPUserAgent, dialpad: &DialpadWidget) {

    // button call clicked
    let sip = sipua.clone();
    dialpad.on_button_call_clicked(move | sip_address, state | {
        println!("sip_call_addres : {}", sip_address);

        match state {
            CallButtonState::Call => sip.call(sip_address),
            CallButtonState::Hangup => sip.call_hangup(),
            CallButtonState::Abort => sip.call_hangup(),
            CallButtonState::Answer => sip.call_answer(),
            _ => ()
        }

    });

    // callback inv state
    let dialpad = dialpad.clone();
    sipua.connect_invite( move | state | {
        let dialpad = dialpad.clone();
        glib::source::idle_add( move || {
            match state {
                SIPInvState::Null => dialpad.update_state_normal(),
                SIPInvState::Calling => dialpad.update_state_outgoing(),
                SIPInvState::Incoming => dialpad.update_state_incoming(),
                SIPInvState::Early => dialpad.update_state_normal(),
                SIPInvState::Connecting => dialpad.update_state_normal(),
                SIPInvState::Confirmed => dialpad.update_state_oncall(),
                SIPInvState::Disconnected => dialpad.update_state_normal()
            }

            glib::Continue(false)
        });

    });
}

// callback account widget
fn callback_account_widget(sipua: &mut SIPUserAgent, account: &AccountWidget) {

    let account_clone = account.clone();
    account.save_connect_clicked(move ||{
        account_clone.save(application_config_path());
    });

    let account_clone = account.clone();
    let sipua_clone = sipua.clone();
    account.on_btn_connect_clicked(move || {
        sipua_clone.acc_config().set_id(account_clone.get_sip_url());
        sipua_clone.acc_config().set_reg_uri(account_clone.get_registrar_url());
        sipua_clone.acc_cred().set_data_type(CredentialInfoType::PlainText);
        sipua_clone.acc_cred().set_realm(account_clone.get_realm());
        sipua_clone.acc_cred().set_username(account_clone.get_username());
        sipua_clone.acc_cred().set_data(account_clone.get_password());
        sipua_clone.account_connect();
    });
}

fn callback_settings_widget(sipua: &mut SIPUserAgent, settings: &SettingsWidget) {

    // save setting for each tab
    let settings_clone = settings.clone();
    settings.save_connect_clicked(move |page| {
        let config_path = application_config_path();
        match page.unwrap() {
            SettingsCurrentActivePage::Ua => settings_clone.call.save(config_path),
            SettingsCurrentActivePage::Stun => settings_clone.stun.save(config_path),
            SettingsCurrentActivePage::Turn => settings_clone.turn.save(config_path),
            SettingsCurrentActivePage::Ice => settings_clone.ice.save(config_path),
            SettingsCurrentActivePage::Audio => settings_clone.audio.save(config_path),
            SettingsCurrentActivePage::Media => settings_clone.media.save(config_path),
            SettingsCurrentActivePage::Proxy => settings_clone.proxy.save(config_path),
            SettingsCurrentActivePage::Dns => settings_clone.dns.save(config_path),
        };
    });

    // reset configuration for each tab
    let settings_clone = settings.clone();
    settings.reset_connect_clicked(move |page| {
        match page.unwrap() {
            SettingsCurrentActivePage::Ua => settings_clone.call.reset(),
            SettingsCurrentActivePage::Stun => settings_clone.stun.reset(),
            SettingsCurrentActivePage::Turn => settings_clone.turn.reset(),
            SettingsCurrentActivePage::Ice => settings_clone.ice.reset(),
            SettingsCurrentActivePage::Audio => settings_clone.audio.reset(),
            SettingsCurrentActivePage::Media => settings_clone.media.reset(),
            SettingsCurrentActivePage::Proxy => settings_clone.proxy.reset(),
            SettingsCurrentActivePage::Dns => settings_clone.dns.reset(),
        };
    });

    // apply configuration for each tab
    let settings_clone = settings.clone();
    let ua = sipua.clone();
    settings.apply_connect_clicked(move |page| {
        match page.unwrap() {
            SettingsCurrentActivePage::Ua => {
                ua.set_autoanswer(settings_clone.call.get_autoanswer());
                ua.set_force_lr(settings_clone.call.get_no_forcelr());
            },
            SettingsCurrentActivePage::Stun => {
                ua.ua_config().set_stun_srv(
                    settings_clone.stun.get_server1(),
                    settings_clone.stun.get_server2(),
                    settings_clone.stun.get_server3(),
                    settings_clone.stun.get_server4()
                );
            },
            SettingsCurrentActivePage::Turn => {
                if settings_clone.turn.get_use_turn() {
                    if !settings_clone.turn.get_server().is_empty() {

                        ua.media_config().set_enable_turn(true);
                        ua.media_config().set_turn_server(settings_clone.turn.get_server());
                        ua.media_config().set_turn_auth_cred(
                            Some("*".to_string()),
                            Some(settings_clone.turn.get_username()),
                            Some(CredentialInfoType::PlainText),
                            Some(settings_clone.turn.get_password()),
                            None);

                        // TODO: check code bellow
                        let tp_type = match settings_clone.turn.get_transport() {
                            1 => TurnTpType::Udp,
                            2 => TurnTpType::Tcp,
                            3 => TurnTpType::Tls,
                            _ => TurnTpType::Udp
                        };

                        ua.media_config().set_turn_conn_type(tp_type);

                    } else {
                        ua.media_config().set_enable_turn(false);
                    }
                }
            },
            SettingsCurrentActivePage::Ice => {
                if settings_clone.ice.get_use_ice() {

                } else {
                    ua.media_config().set_enable_ice(settings_clone.ice.get_use_ice());
                    ua.media_config().set_ice_no_rtcp(settings_clone.ice.get_no_rtcp());
                    ua.media_config().set_ice_max_host_cands(settings_clone.ice.get_max_hosts() as i32);

                    let trickle_method = match settings_clone.ice.get_trickle_method() {
                        1 => IceSessTrickle::Disabled,
                        2 => IceSessTrickle::Half,
                        3 => IceSessTrickle::Full,
                        _ => IceSessTrickle::Disabled
                    };

                    ua.media_config().set_ice_opt(
                        Some(settings_clone.ice.get_aggressive()),
                        None,
                        None,
                        Some(trickle_method)
                    );
                }
            },
            SettingsCurrentActivePage::Audio => {

            },
            SettingsCurrentActivePage::Media => {

            },
            SettingsCurrentActivePage::Proxy => {
                ua.ua_config().set_outbound_proxy(
                    settings_clone.proxy.get_proxy1(),
                    settings_clone.proxy.get_proxy2(),
                    None ,
                    None
                );
            },
            SettingsCurrentActivePage::Dns => {
                ua.ua_config().set_nameserver(
                    settings_clone.dns.get_nameserver1(),
                    settings_clone.dns.get_nameserver2(),
                    settings_clone.dns.get_nameserver3(),
                    settings_clone.dns.get_nameserver4()
                )
            }
        }

        ua.restart();
    });
}

fn callback_codec_widget(sipua: &mut SIPUserAgent, settings: &CodecWidget) {

}

fn main() {
    gtk::init()
    .expect("Cant initalize gtk");

    let application = Application::new(
        Some("com.tamara.open_ip_audio_codec"),
        gio::ApplicationFlags::FLAGS_NONE
    ).expect("GTK app fail to initialize.");

    let mut sipua = SIPUserAgent::new();
    let tx_level = Rc::new(RefCell::new(100));
    let rx_level = Rc::new(RefCell::new(100));

    sipua.log_config().set_level(5);
    sipua.start();

    // builder
    let builder= Builder::from_string(include_str!("../glade/main_ui.glade"));
    let main_window: gtk::ApplicationWindow = builder.get_object("main_ui").unwrap();

    let rx_widget = audio_line::create_transmit_widget(&builder);
    let tx_widget = audio_line::create_receive_widget(&builder);
    let maintab_widget= MaintabWidget::new(&builder);
    let statusbar_widget = StatusbarWidget::new(&builder);
    let headerbar_widget = HeaderWidget::new(&builder);
    let dialpad_widget = DialpadWidget::new(&builder);
    let account_widget = AccountWidget::new(&builder);
    let settings_widget= SettingsWidget::new(&builder);
    let codec_widget = CodecWidget::new(&builder);

    // set callback
    callback_audio_line_widget(rx_level, tx_level, &rx_widget, &tx_widget);
    callback_dialpad_widget(&mut sipua, &dialpad_widget);
    callback_account_widget(&mut sipua, &account_widget);
    callback_settings_widget(&mut sipua, &settings_widget);
    callback_codec_widget(&mut sipua, &codec_widget);

    // test call data
    dialpad_widget.set_call_address_text(String::from("sip://@27.50.19.174"));
    dialpad_widget.add_call_log("sip://@27.50.19.174");
    dialpad_widget.add_call_log("*888#");
    dialpad_widget.add_call_log("*363#");

    // account test
    account_widget.set_sip_url("sip:ipcodec01@27.50.19.174");
    account_widget.set_registrar_url("sip:27.50.19.174");
    account_widget.set_realm("asterisk");
    account_widget.set_username("ipcodec01");
    account_widget.set_password("12345678");

    // test google stun server
    settings_widget.stun.set_server1("stun1.l.google.com:19302".to_string());
    settings_widget.stun.set_server2("stun2.l.google.com:19302".to_string());
    settings_widget.stun.set_server3("stun3.l.google.com:19302".to_string());
    settings_widget.stun.set_server4("stun4.l.google.com:19302".to_string());


    // test public dns nameserver
    settings_widget.dns.set_nameserver1("8.8.8.8".to_string()); // google main dns
    settings_widget.dns.set_nameserver2("8.8.4.4".to_string()); // google backup dns
    settings_widget.dns.set_nameserver3("1.1.1.1".to_string()); // cloudflare main dns
    settings_widget.dns.set_nameserver4("1.0.0.1".to_string()); // cloudflare backup dns

    // init application
    application.connect_activate(move |app| {
        // input
        main_window.set_application(Some(app));
        main_window.show_all();
    });

    // thread procedure to update level bar
    // Transmit and Receive
    thread_update_level_bar(rx_widget.clone(), tx_widget.clone());

    // sub testing gui
    application.run(&env::args().collect::<Vec<_>>());

}
