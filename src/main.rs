mod app;
mod data_structs;
mod scraper;
mod action;
mod utils;
mod views;
mod widgets;
mod traits;

#[macro_use]
extern crate cascade;

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use std::sync::Arc;

impl traits::LabelWidget for gtk::Label {}
impl <A: glib::IsA<gtk::Box>> traits::BoxWidget for A {}
impl <A: glib::IsA<gtk::Container>> traits::ContainerWidget for A {}

const APP_ID: &str = "com.github.timarques.sw-monsters";
const APP_NAME: &str = "SW Monsters";
const CSS: &str = include_str!("../data/ui.css");

fn main() {
    glib::set_application_name(APP_NAME);
    glib::set_program_name(Some(APP_NAME));
    let application = gtk::Application::new(Some(APP_ID), gio::ApplicationFlags::FLAGS_NONE)
        .expect("GTK initialization failed");
    application.connect_activate(move |gtk_app| {
         let app = Arc::new(app::App::new(gtk_app.clone()));
         app.load_styles(CSS);
         app.run(app.clone());
     });
    application.run(&[]);
}
