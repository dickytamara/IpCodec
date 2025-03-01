
use gtk::prelude::*;
use gtk::{Switch, Label, Builder};
use std::cell::RefCell;
use std::path::PathBuf;


use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct SettingsUaWidgetStorage {
    lbl_autoanswer: Label,
    lbl_no_forcelr: Label,
    swt_autoanswer: Switch,
    swt_no_forcelr: Switch
}

impl SettingsUaWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsUaWidgetStorage {
            lbl_autoanswer: gtk_builder.get_object("lbl_ua_autoanswer").unwrap(),
            lbl_no_forcelr: gtk_builder.get_object("lbl_ua_no_forcelr").unwrap(),
            swt_autoanswer: gtk_builder.get_object("swt_ua_autoanswer").unwrap(),
            swt_no_forcelr: gtk_builder.get_object("swt_ua_no_forcelr").unwrap()
        }
    }
}

#[derive(Clone)]
pub struct SettingsUaWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsUaWidgetStorage>
}

impl SettingsUaWidget {

    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsUaWidget{
            ctx: RefCell::new(SettingsUaWidgetStorage::new(gtk_builder))
        }
    }

    /// sub procedure reset state gui
    pub fn reset(&self) {
        self.ctx.borrow().swt_autoanswer.set_state(false);
        self.ctx.borrow().swt_no_forcelr.set_state(false);
    }

    // set autoanswer state
    pub fn set_autoanswer(&self, value: bool) {
        self.ctx.borrow().swt_autoanswer.set_state(value);
    }

    // get autoanswer state
    pub fn get_autoanswer(&self) -> bool {
        self.ctx.borrow().swt_autoanswer.get_state()
    }

    pub fn set_no_forcelr(&self, value: bool) {
        self.ctx.borrow_mut().swt_no_forcelr.set_state(value);
    }

    pub fn get_no_forcelr(&self) -> bool {
        self.ctx.borrow().swt_no_forcelr.get_state()
    }
}


impl HelperFileSettings for SettingsUaWidget {

    // load from file
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_autoanswer(config.get("ua", "autoanswer").unwrap_or(String::from("false")).parse().unwrap());
        self.set_no_forcelr(config.get("ua", "no_force_lr").unwrap_or(String::from("false")).parse().unwrap());
    }

    // save to file
    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("ua", "autoanswer", Some(self.get_autoanswer().to_string()));
        config.set("ua", "no_forcelr", Some(self.get_no_forcelr().to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}




