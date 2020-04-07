mod app;
mod views;
mod widgets;
mod traits;
mod threadpool;
mod header;
mod data;
mod action;
mod scraper;
mod utils;
mod error;

#[macro_use]
extern crate lazy_static;

use gio::prelude::{ApplicationExt, ApplicationExtManual};

static mut UNSAFE_SENDER: Option<glib::Sender<action::Action>> = None;

#[derive(Clone, Debug)]
struct AppInfo {
    pub name: &'static str,
    pub id: &'static str,
    pub version: &'static str,
    pub authors: Vec<&'static str>,
    pub repository: &'static str
}

struct CustomSender<'a> {
    sender: &'a Option<glib::Sender<action::Action>>
}

impl <'a> CustomSender <'a> {

    pub unsafe fn new() -> Self {
        Self { sender: &UNSAFE_SENDER }
    }

    pub fn send(&self, action: action::Action) {
        self.sender.as_ref().unwrap().send(action).unwrap();
    }

}

lazy_static!{
    static ref SENDER: CustomSender<'static> = unsafe {CustomSender::new()};
    static ref APP_INFO: AppInfo = AppInfo {
        name: "SW Monsters",
        id: "com.github.timarques.sw-monsters",
        authors: env!("CARGO_PKG_AUTHORS").split(", ").collect(),
        version: env!("CARGO_PKG_VERSION"),
        repository: "https://github.com/timarques/sw-monsters"
    };
    static ref DATA: data::Data = data::Data::new(&{
        let mut path = glib::get_user_cache_dir().unwrap_or(std::path::Path::new("~/.cache").to_path_buf());
        path.push(APP_INFO.id);
        path.push("data.json");
        path
    });
    static ref THREAD_POOL: threadpool::ThreadPool = {
        let threadpool = threadpool::ThreadPool::new();
        threadpool.workers(glib::get_num_processors() as i32 + 1);
        threadpool
    };
}

fn main() {

    glib::set_application_name(APP_INFO.name);
    glib::set_program_name(Some(APP_INFO.name));
    let application = gtk::Application::new(Some(APP_INFO.id), gio::ApplicationFlags::FLAGS_NONE).expect("GTK initialization failed");
    application.connect_activate(move |gtk_app| app::App::run(gtk_app, include_str!("../data/ui.css")));
    application.run(&[]);
}
