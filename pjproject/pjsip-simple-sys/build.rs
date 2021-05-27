
// extern crate bindgen;
// use std::env;

fn main () {

    // dynamic
    println!("cargo:rustc-link-lib=pjsip-simple");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rerun-if-changed=wrapper.h");

    // dynamic
    // println!("cargo:rustc-link-lib=static=pjsip-simple-x86_64-unknown-linux-gnu");
    // println!("cargo:rustc-link-search=native=/usr/local/lib");
    // println!("cargo:rerun-if-changed=wrapper.h");

    // generate lib.rs
    // create_bindgen();

}


// fn create_bindgen() {
//         // The bindgen::Builder is the main entry point
//     // to bindgen, and lets you build up options for
//     // the resulting bindings.
//     let pjsip_simple = bindgen::Builder::default().header("wrapper.h")

//         .raw_line("#![allow(non_upper_case_globals)]")
//         .raw_line("#![allow(non_camel_case_types)]")
//         .raw_line("#![allow(non_snake_case)]")
//         .raw_line("extern crate pj_sys;")
//         .raw_line("extern crate pjlib_util_sys;")
//         .raw_line("extern crate pjsip_sys;")
//         .raw_line("use pj_sys::*;")
//         .raw_line("use pjlib_util_sys::*;")
//         .raw_line("use pjsip_sys::*;")
//         // .parse_callbacks(Box::new(bindgen::CargoCallbacks))



//         .allowlist_function("pjpidf_create")
//         .allowlist_function("pjpidf_parse")
//         .allowlist_function("pjpidf_pres_add_note")
//         .allowlist_function("pjpidf_pres_add_tuple")
//         .allowlist_function("pjpidf_pres_construct")
//         .allowlist_function("pjpidf_pres_find_tuple")
//         .allowlist_function("pjpidf_pres_get_first_note")
//         .allowlist_function("pjpidf_pres_get_first_tuple")
//         .allowlist_function("pjpidf_pres_get_next_note")
//         .allowlist_function("pjpidf_pres_get_next_tuple")
//         .allowlist_function("pjpidf_pres_remove_tuple")
//         .allowlist_function("pjpidf_print")
//         .allowlist_function("pjpidf_status_construct")
//         .allowlist_function("pjpidf_status_is_basic_open")
//         .allowlist_function("pjpidf_status_set_basic_open")
//         .allowlist_function("pjpidf_tuple_add_note")
//         .allowlist_function("pjpidf_tuple_construct")
//         .allowlist_function("pjpidf_tuple_get_contact")
//         .allowlist_function("pjpidf_tuple_get_contact_prio")
//         .allowlist_function("pjpidf_tuple_get_first_note")
//         .allowlist_function("pjpidf_tuple_get_id")
//         .allowlist_function("pjpidf_tuple_get_next_note")
//         .allowlist_function("pjpidf_tuple_get_status")
//         .allowlist_function("pjpidf_tuple_get_timestamp")
//         .allowlist_function("pjpidf_tuple_set_contact")
//         .allowlist_function("pjpidf_tuple_set_contact_prio")
//         .allowlist_function("pjpidf_tuple_set_id")
//         .allowlist_function("pjpidf_tuple_set_timestamp")
//         .allowlist_function("pjpidf_tuple_set_timestamp_np")
//         .allowlist_type("PJPIDF_.*")
//         .allowlist_type("pjpidf_.*")
//         .allowlist_var("PJPIDF_.*")
//         .allowlist_var("pjpidf_.*")

//         .allowlist_function("pjrpid_add_element")
//         .allowlist_function("pjrpid_element_dup")
//         .allowlist_function("pjrpid_get_element")
//         .allowlist_type("PJRPID_.*")
//         .allowlist_type("pjrpid_.*")
//         .allowlist_var("PJRPID_.*")
//         .allowlist_var("pjrpid_.*")

//         .allowlist_function("pjsip_allow_events_hdr_create")
//         .allowlist_function("pjsip_event_hdr_create")
//         .allowlist_type("PJSIP_ALLOW_.*")
//         .allowlist_type("pjsip_allow_.*")
//         .allowlist_var("PJSIP_ALLOW_.*")
//         .allowlist_var("pjsip_allow_.*")


//         .allowlist_function("pjsip_evsub_accept")
//         .allowlist_function("pjsip_evsub_add_header")
//         .allowlist_function("pjsip_evsub_add_ref")
//         .allowlist_function("pjsip_evsub_create_uac")
//         .allowlist_function("pjsip_evsub_create_uas")
//         .allowlist_function("pjsip_evsub_current_notify")
//         .allowlist_function("pjsip_evsub_dec_ref")
//         .allowlist_function("pjsip_evsub_get_allow_events_hdr")
//         .allowlist_function("pjsip_evsub_get_mod_data")
//         .allowlist_function("pjsip_evsub_get_state")
//         .allowlist_function("pjsip_evsub_get_state_name")
//         .allowlist_function("pjsip_evsub_get_termination_reason")
//         .allowlist_function("pjsip_evsub_initiate")
//         .allowlist_function("pjsip_evsub_init_module")
//         .allowlist_function("pjsip_evsub_init_parser")
//         .allowlist_function("pjsip_evsub_instance")
//         .allowlist_function("pjsip_evsub_notify")
//         .allowlist_function("pjsip_evsub_register_pkg")
//         .allowlist_function("pjsip_evsub_send_request")
//         .allowlist_function("pjsip_evsub_set_mod_data")
//         .allowlist_function("pjsip_evsub_terminate")
//         .allowlist_function("pjsip_evsub_uas_set_timeout")
//         .allowlist_type("PJSIP_EVSUB.*")
//         .allowlist_type("pjsip_evsub.*")
//         .allowlist_var("PJSIP_EVSUB.*")
//         .allowlist_var("pjsip_evsub.*")

//         .allowlist_function("pjsip_get_notify_method")
//         .allowlist_function("pjsip_get_subscribe_method")
//         .allowlist_function("pjsip_iscomposing_create_body")
//         .allowlist_function("pjsip_iscomposing_create_xml")
//         .allowlist_function("pjsip_iscomposing_parse")
//         .allowlist_function("pjsip_mwi_accept")
//         .allowlist_function("pjsip_mwi_create_uac")
//         .allowlist_function("pjsip_mwi_create_uas")
//         .allowlist_function("pjsip_mwi_current_notify")
//         .allowlist_function("pjsip_mwi_initiate")
//         .allowlist_function("pjsip_mwi_init_module")
//         .allowlist_function("pjsip_mwi_instance")
//         .allowlist_function("pjsip_mwi_notify")
//         .allowlist_function("pjsip_mwi_send_request")
//         .allowlist_function("pjsip_mwi_terminate")
//         .allowlist_function("pjsip_pres_accept")
//         .allowlist_function("pjsip_pres_add_header")
//         .allowlist_function("pjsip_pres_create_pidf")
//         .allowlist_function("pjsip_pres_create_uac")
//         .allowlist_function("pjsip_pres_create_uas")
//         .allowlist_function("pjsip_pres_create_xpidf")
//         .allowlist_function("pjsip_pres_current_notify")
//         .allowlist_function("pjsip_pres_get_status")
//         .allowlist_function("pjsip_pres_initiate")
//         .allowlist_function("pjsip_pres_init_module")
//         .allowlist_function("pjsip_pres_instance")
//         .allowlist_function("pjsip_pres_notify")
//         .allowlist_function("pjsip_pres_parse_pidf")
//         .allowlist_function("pjsip_pres_parse_pidf2")
//         .allowlist_function("pjsip_pres_parse_xpidf")
//         .allowlist_function("pjsip_pres_parse_xpidf2")
//         .allowlist_function("pjsip_pres_send_request")
//         .allowlist_function("pjsip_pres_set_status")
//         .allowlist_function("pjsip_pres_terminate")
//         .allowlist_type("PJSIP_PRES_.*")
//         .allowlist_type("pjsip_pres_.*")
//         .allowlist_var("PJSIP_PRES_.*")
//         .allowlist_var("pjsip_pres_.*")
//         .allowlist_type("pjsip_notify_method")
//         .allowlist_var("pjsip_notify_method")

//         .allowlist_function("pjsip_publishc_create")
//         .allowlist_function("pjsip_publishc_destroy")
//         .allowlist_function("pjsip_publishc_get_pool")
//         .allowlist_function("pjsip_publishc_init")
//         .allowlist_function("pjsip_publishc_init_module")
//         .allowlist_function("pjsip_publishc_opt_default")
//         .allowlist_function("pjsip_publishc_publish")
//         .allowlist_function("pjsip_publishc_send")
//         .allowlist_function("pjsip_publishc_set_credentials")
//         .allowlist_function("pjsip_publishc_set_headers")
//         .allowlist_function("pjsip_publishc_set_route_set")
//         .allowlist_function("pjsip_publishc_set_via_sent_by")
//         .allowlist_function("pjsip_publishc_unpublish")
//         .allowlist_function("pjsip_publishc_update_expires")
//         .allowlist_type("PJSIP_PUBLISHC.*")
//         .allowlist_type("pjsip_publishc.*")
//         .allowlist_var("PJSIP_PUBLISHC.*")
//         .allowlist_var("pjsip_publishc.*")

//         .allowlist_function("pjsipsimple_strerror")
//         .allowlist_function("pjsip_sub_state_hdr_create")
//         .allowlist_function("pjsip_tsx_get_evsub")

//         .allowlist_function("pjxpidf_create")
//         .allowlist_function("pjxpidf_get_status")
//         .allowlist_function("pjxpidf_get_uri")
//         .allowlist_function("pjxpidf_parse")
//         .allowlist_function("pjxpidf_print")
//         .allowlist_function("pjxpidf_set_status")
//         .allowlist_function("pjxpidf_set_uri")
//         .allowlist_type("PJXPIDF_.*")
//         .allowlist_type("pjxpidf_.*")
//         .allowlist_var("PJXPIDF_.*")
//         .allowlist_var("pjxpidf_.*")

//         .allowlist_type("pjsip_event_hdr")
//         .allowlist_type("pjsip_sub_state_hdr")

//         .allowlist_recursively(false)
//         .translate_enum_integer_types(true)
//         .layout_tests(false)
//         .prepend_enum_name(false)
//         .generate()
//         // Unwrap the Result and panic on failure.
//         .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     let out_path = env::current_dir().unwrap();
//     pjsip_simple.write_to_file(out_path.join("src/lib.rs")).expect("Error write src/lib.rs");

// }
