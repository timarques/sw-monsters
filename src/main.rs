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

fn main() {
    let app_info = data_structs::AppInfo {
        name: "SW Monsters",
        id: "com.github.timarques.sw-monsters",
        authors: env!("CARGO_PKG_AUTHORS").split(", ").collect(),
        version: env!("CARGO_PKG_VERSION"),
        repository: "com.github.timarques.sw-monsters"
    };
    glib::set_application_name(app_info.name);
    glib::set_program_name(Some(app_info.name));
    let application = gtk::Application::new(Some(app_info.id), gio::ApplicationFlags::FLAGS_NONE)
        .expect("GTK initialization failed");
    application.connect_activate(move |gtk_app| {
         let app = Arc::new(app::App::new(gtk_app.clone(), &app_info));
         app.load_styles(include_str!("../data/ui.css"));
         app.run(app.clone());
     });
    application.run(&[]);
}
