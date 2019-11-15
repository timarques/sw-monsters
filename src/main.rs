mod gui;
mod monster;
mod action;

use glib;
use gui::Gui;
use monster::Monster;
use serde_json;
use std::rc::Rc;
use gio::prelude::*;

const APP_ID: &str = "com.github.timarques.sw-monsters";
const APP_NAME: &str = "SW Monsters";
const CSS: &str = include_str!("../data/ui.css");
const MONSTERS: &str = include_str!("../data/monsters.json");

fn main() {
    glib::set_program_name(APP_NAME.into());
    glib::set_application_name(APP_NAME);
    let data: Vec<Monster> = serde_json::from_str(&MONSTERS).unwrap();
    let application = gtk::Application::new(Some(APP_ID), gio::ApplicationFlags::FLAGS_NONE)
        .expect("GTK initialization failed");
    application.connect_activate(move |app| {
        let gui = Rc::new(Gui::new(app, data.clone()));
        gui.load_styles(&CSS);
        gui.connect_events(gui.clone());
        gui.init();
    });
    application.run(&[]);
}
