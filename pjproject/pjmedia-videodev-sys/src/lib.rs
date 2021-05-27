/* automatically generated by rust-bindgen 0.58.1 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate pj_sys;
extern crate pjmedia_sys;

use pj_sys::*;
use pjmedia_sys::*;

use std::os::raw::{c_uint, c_char, c_void};

extern "C" {
    pub fn pjmedia_video_format_mgr_create(pool: *mut pj_pool_t,max_fmt: c_uint,options: c_uint,p_mgr: *mut *mut pjmedia_video_format_mgr) -> pj_status_t;
    pub fn pjmedia_video_format_mgr_instance() -> *mut pjmedia_video_format_mgr;
    pub fn pjmedia_video_format_mgr_set_instance(mgr: *mut pjmedia_video_format_mgr);
    pub fn pjmedia_video_format_mgr_destroy(mgr: *mut pjmedia_video_format_mgr);
    pub fn pjmedia_videodev_strerror(status: pj_status_t, buffer: *mut c_char, bufsize: pj_size_t) -> pj_str_t;
    pub fn pjmedia_get_vid_subsys() -> *mut pjmedia_vid_subsys;
    pub fn pjmedia_vid_driver_init(drv_idx: c_uint, refresh: pj_bool_t) -> pj_status_t;
    pub fn pjmedia_vid_driver_deinit(drv_idx: c_uint);
    pub fn pjmedia_vid_dev_cap_name(cap: pjmedia_vid_dev_cap, p_desc: *mut *const c_char) -> *const c_char;
    pub fn pjmedia_vid_dev_param_set_cap(param: *mut pjmedia_vid_dev_param, cap: pjmedia_vid_dev_cap, pval: *const c_void) -> pj_status_t;
    pub fn pjmedia_vid_dev_param_get_cap(param: *const pjmedia_vid_dev_param, cap: pjmedia_vid_dev_cap, pval: *mut c_void) -> pj_status_t;
    pub fn pjmedia_vid_dev_refresh() -> pj_status_t;
    pub fn pjmedia_vid_dev_count() -> c_uint;
    pub fn pjmedia_vid_dev_get_info(id: pjmedia_vid_dev_index, info: *mut pjmedia_vid_dev_info) -> pj_status_t;
    pub fn pjmedia_vid_dev_lookup(drv_name: *const c_char, dev_name: *const c_char, id: *mut pjmedia_vid_dev_index) -> pj_status_t;
    pub fn pjmedia_vid_dev_default_param(pool: *mut pj_pool_t, id: pjmedia_vid_dev_index, param: *mut pjmedia_vid_dev_param) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_create(param: *mut pjmedia_vid_dev_param, cb: *const pjmedia_vid_dev_cb, user_data: *mut c_void, p_strm: *mut *mut pjmedia_vid_dev_stream) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_get_param(strm: *mut pjmedia_vid_dev_stream, param: *mut pjmedia_vid_dev_param) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_get_cap(strm: *mut pjmedia_vid_dev_stream, cap: pjmedia_vid_dev_cap, value: *mut c_void) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_set_cap(strm: *mut pjmedia_vid_dev_stream, cap: pjmedia_vid_dev_cap, value: *const c_void) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_start(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_is_running(strm: *mut pjmedia_vid_dev_stream) -> pj_bool_t;
    pub fn pjmedia_vid_dev_stream_get_frame(strm: *mut pjmedia_vid_dev_stream, frame: *mut pjmedia_frame) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_put_frame(strm: *mut pjmedia_vid_dev_stream, frame: *const pjmedia_frame) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_stop(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;
    pub fn pjmedia_vid_dev_stream_destroy(strm: *mut pjmedia_vid_dev_stream) -> pj_status_t;
    pub fn pjmedia_vid_codec_param_clone(pool: *mut pj_pool_t, src: *const pjmedia_vid_codec_param) -> *mut pjmedia_vid_codec_param;
    pub fn pjmedia_vid_codec_mgr_create(pool: *mut pj_pool_t, mgr: *mut *mut pjmedia_vid_codec_mgr) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_destroy(mgr: *mut pjmedia_vid_codec_mgr) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_instance() -> *mut pjmedia_vid_codec_mgr;
    pub fn pjmedia_vid_codec_mgr_set_instance(mgr: *mut pjmedia_vid_codec_mgr);
    pub fn pjmedia_vid_codec_mgr_register_factory(mgr: *mut pjmedia_vid_codec_mgr, factory: *mut pjmedia_vid_codec_factory) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_unregister_factory(mgr: *mut pjmedia_vid_codec_mgr, factory: *mut pjmedia_vid_codec_factory) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_enum_codecs(mgr: *mut pjmedia_vid_codec_mgr, count: *mut c_uint, info: *mut pjmedia_vid_codec_info, prio: *mut c_uint) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_get_codec_info(mgr: *mut pjmedia_vid_codec_mgr, pt: c_uint, info: *mut *const pjmedia_vid_codec_info) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_get_codec_info2(mgr: *mut pjmedia_vid_codec_mgr, fmt_id: pjmedia_format_id, info: *mut *const pjmedia_vid_codec_info) -> pj_status_t;
    pub fn pjmedia_vid_codec_info_to_id(info: *const pjmedia_vid_codec_info, id: *mut c_char, max_len: c_uint) -> *mut c_char;
    pub fn pjmedia_vid_codec_mgr_find_codecs_by_id(mgr: *mut pjmedia_vid_codec_mgr, codec_id: *const pj_str_t, count: *mut c_uint, p_info: *mut *const pjmedia_vid_codec_info, prio: *mut c_uint) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_set_codec_priority(mgr: *mut pjmedia_vid_codec_mgr, codec_id: *const pj_str_t, prio: pj_uint8_t) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_get_default_param(mgr: *mut pjmedia_vid_codec_mgr, info: *const pjmedia_vid_codec_info, param: *mut pjmedia_vid_codec_param) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_set_default_param(mgr: *mut pjmedia_vid_codec_mgr, info: *const pjmedia_vid_codec_info, param: *const pjmedia_vid_codec_param) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_alloc_codec(mgr: *mut pjmedia_vid_codec_mgr, info: *const pjmedia_vid_codec_info, p_codec: *mut *mut pjmedia_vid_codec) -> pj_status_t;
    pub fn pjmedia_vid_codec_mgr_dealloc_codec(mgr: *mut pjmedia_vid_codec_mgr, codec: *mut pjmedia_vid_codec) -> pj_status_t;
    pub fn pjmedia_vid_conf_setting_default(opt: *mut pjmedia_vid_conf_setting);
    pub fn pjmedia_vid_conf_create(pool: *mut pj_pool_t, opt: *const pjmedia_vid_conf_setting, p_vid_conf: *mut *mut pjmedia_vid_conf) -> pj_status_t;
    pub fn pjmedia_vid_conf_destroy(vid_conf: *mut pjmedia_vid_conf) -> pj_status_t;
    pub fn pjmedia_vid_conf_add_port(vid_conf: *mut pjmedia_vid_conf, pool: *mut pj_pool_t, port: *mut pjmedia_port, name: *const pj_str_t, opt: *mut c_void, p_slot: *mut c_uint) -> pj_status_t;
    pub fn pjmedia_vid_conf_remove_port(vid_conf: *mut pjmedia_vid_conf, slot: c_uint) -> pj_status_t;
    pub fn pjmedia_vid_conf_get_port_count(vid_conf: *mut pjmedia_vid_conf) -> c_uint;
    pub fn pjmedia_vid_conf_enum_ports(vid_conf: *mut pjmedia_vid_conf, slots: *mut c_uint, count: *mut c_uint) -> pj_status_t;
    pub fn pjmedia_vid_conf_get_port_info(vid_conf: *mut pjmedia_vid_conf, slot: c_uint, info: *mut pjmedia_vid_conf_port_info) -> pj_status_t;
    pub fn pjmedia_vid_conf_connect_port(vid_conf: *mut pjmedia_vid_conf, src_slot: c_uint, sink_slot: c_uint, opt: *mut c_void) -> pj_status_t;
    pub fn pjmedia_vid_conf_disconnect_port(vid_conf: *mut pjmedia_vid_conf, src_slot: c_uint, sink_slot: c_uint) -> pj_status_t;
    pub fn pjmedia_vid_dev_subsys_init(pf: *mut pj_pool_factory) -> pj_status_t;
    pub fn pjmedia_vid_dev_subsys_get_pool_factory() -> *mut pj_pool_factory;
    pub fn pjmedia_vid_dev_subsys_shutdown() -> pj_status_t;
    pub fn pjmedia_vid_register_factory(vdf: pjmedia_vid_dev_factory_create_func_ptr, factory: *mut pjmedia_vid_dev_factory) -> pj_status_t;
    pub fn pjmedia_vid_unregister_factory(vdf: pjmedia_vid_dev_factory_create_func_ptr, factory: *mut pjmedia_vid_dev_factory) -> pj_status_t;
    pub fn pjmedia_vid_port_param_default(prm: *mut pjmedia_vid_port_param);
    pub fn pjmedia_vid_port_create(pool: *mut pj_pool_t, prm: *const pjmedia_vid_port_param, p_vp: *mut *mut pjmedia_vid_port) -> pj_status_t;
    pub fn pjmedia_vid_port_set_cb(vid_port: *mut pjmedia_vid_port, cb: *const pjmedia_vid_dev_cb, user_data: *mut c_void);
    pub fn pjmedia_vid_port_get_stream(vid_port: *mut pjmedia_vid_port) -> *mut pjmedia_vid_dev_stream;
    pub fn pjmedia_vid_port_get_passive_port(vid_port: *mut pjmedia_vid_port) -> *mut pjmedia_port;
    pub fn pjmedia_vid_port_get_clock_src(vid_port: *mut pjmedia_vid_port) -> *mut pjmedia_clock_src;
    pub fn pjmedia_vid_port_set_clock_src(vid_port: *mut pjmedia_vid_port, clocksrc: *mut pjmedia_clock_src) -> pj_status_t;
    pub fn pjmedia_vid_port_subscribe_event(vid_port: *mut pjmedia_vid_port, port: *mut pjmedia_port) -> pj_status_t;
    pub fn pjmedia_vid_port_connect(vid_port: *mut pjmedia_vid_port, port: *mut pjmedia_port, destroy: pj_bool_t) -> pj_status_t;
    pub fn pjmedia_vid_port_disconnect(vid_port: *mut pjmedia_vid_port) -> pj_status_t;
    pub fn pjmedia_vid_port_get_connected_port( vid_port: *mut pjmedia_vid_port) -> *mut pjmedia_port;
    pub fn pjmedia_vid_port_start(vid_port: *mut pjmedia_vid_port) -> pj_status_t;
    pub fn pjmedia_vid_port_is_running(vid_port: *mut pjmedia_vid_port) -> pj_bool_t;
    pub fn pjmedia_vid_port_stop(vid_port: *mut pjmedia_vid_port) -> pj_status_t;
    pub fn pjmedia_vid_port_destroy(vid_port: *mut pjmedia_vid_port);
    pub fn pjmedia_vid_stream_info_from_sdp(si: *mut pjmedia_vid_stream_info, pool: *mut pj_pool_t, endpt: *mut pjmedia_endpt, local: *const pjmedia_sdp_session, remote: *const pjmedia_sdp_session, stream_idx: c_uint) -> pj_status_t;
    pub fn pjmedia_vid_stream_rc_config_default(cfg: *mut pjmedia_vid_stream_rc_config);
    pub fn pjmedia_vid_stream_sk_config_default(cfg: *mut pjmedia_vid_stream_sk_config);
    pub fn pjmedia_vid_stream_create(endpt: *mut pjmedia_endpt, pool: *mut pj_pool_t, info: *mut pjmedia_vid_stream_info, tp: *mut pjmedia_transport, user_data: *mut c_void, p_stream: *mut *mut pjmedia_vid_stream) -> pj_status_t;
    pub fn pjmedia_vid_stream_destroy(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    pub fn pjmedia_vid_stream_get_port(stream: *mut pjmedia_vid_stream, dir: pjmedia_dir, p_port: *mut *mut pjmedia_port) -> pj_status_t;
    pub fn pjmedia_vid_stream_get_transport(st: *mut pjmedia_vid_stream) -> *mut pjmedia_transport;
    pub fn pjmedia_vid_stream_get_stat(stream: *const pjmedia_vid_stream, stat: *mut pjmedia_rtcp_stat) -> pj_status_t;
    pub fn pjmedia_vid_stream_reset_stat(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    pub fn pjmedia_vid_stream_get_stat_jbuf(stream: *const pjmedia_vid_stream, state: *mut pjmedia_jb_state) -> pj_status_t;
    pub fn pjmedia_vid_stream_get_info(stream: *const pjmedia_vid_stream, info: *mut pjmedia_vid_stream_info) -> pj_status_t;
    pub fn pjmedia_vid_stream_start(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    pub fn pjmedia_vid_stream_is_running(stream: *mut pjmedia_vid_stream, dir: pjmedia_dir) -> pj_bool_t;
    pub fn pjmedia_vid_stream_pause(stream: *mut pjmedia_vid_stream, dir: pjmedia_dir) -> pj_status_t;
    pub fn pjmedia_vid_stream_resume(stream: *mut pjmedia_vid_stream, dir: pjmedia_dir) -> pj_status_t;
    pub fn pjmedia_vid_stream_send_keyframe(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    pub fn pjmedia_vid_stream_send_rtcp_sdes(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    pub fn pjmedia_vid_stream_send_rtcp_bye(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    pub fn pjmedia_vid_stream_send_rtcp_pli(stream: *mut pjmedia_vid_stream) -> pj_status_t;
    pub fn pjmedia_vid_stream_get_rtp_session_info(stream: *mut pjmedia_vid_stream, session_info: *mut pjmedia_stream_rtp_sess_info) -> pj_status_t;
}
