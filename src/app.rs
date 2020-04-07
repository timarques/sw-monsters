use crate::threadpool;
use crate::AppInfo;
use crate::data::Controller;
use crate::action::{Action, View};
use crate::views::Views;
use crate::header::Header;
use gio::{ApplicationExt, ActionMapExt};
use gtk::prelude::*;

#[derive(Clone)]
pub struct App {
    app: gtk::Application,
    window: gtk::ApplicationWindow,
    header: Header,
    views: Views,
    history: Vec<View>,
    sender: glib::Sender<Action>,
    threadpool: threadpool::ThreadPool,
    info: AppInfo,
    data: Controller
}

impl App {

    pub fn run(app: &gtk::Application, info: &AppInfo , css: &str) {
        let (sender, receiver) = glib::MainContext::channel::<Action>(glib::PRIORITY_DEFAULT);

        let threadpool = threadpool::ThreadPool::new();
        threadpool.workers(glib::get_num_processors() as i32);

        let mut path = glib::get_user_cache_dir().unwrap_or(std::path::Path::new("~/.cache").to_path_buf());
        path.push(info.id);
        path.push("data.json");
        let data = Controller::new(&path);

        let views = Views::new();
        let header = Header::new();
        header.set_title(Some(info.name), None);

        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(800, 600);
        window.set_size_request(400, 300);
        window.set_titlebar(Some(&header.container));
        window.set_position(gtk::WindowPosition::Center);
        window.add(&views.stack);

        gtk::IconTheme::get_default().unwrap().prepend_search_path("data/icons");

        let mut app = Self {
            app: app.clone(),
            window,
            header,
            views,
            history: Vec::new(),
            sender,
            threadpool,
            info: info.clone(),
            data
        };
        app.load_styles(css);
        app.window.show_all();
        app.window.present();
        app.connect_events();
        app.connect_actions();

        let data = app.data.clone();
        let sender = app.sender.clone();
        app.sender.send(Action::Load(Box::new(move || {
            data.init();
            sender.send(Action::Ready());
            Ok(View::Collection())
        })));

        receiver.attach(None, move |action| app.do_action(action));
    }

    fn load<A: FnOnce() + 'static + Send + Sync>(&self, callback: A) {
        self.header.disable_buttons();
        self.views.loading.start();
        self.views.stack.set_visible_child_name("loading");
        self.threadpool.add(callback);
    }

    fn do_action(&mut self, action: Action) -> glib::Continue {
        match action {
            Action::Ready() => {
                self.views.collection.init();
                self.header.search.init();
                self.data.sync(&self.threadpool.group(2), || self.sender.send(Action::SyncFinished()).unwrap());
            },
            Action::SyncFinished() => {
                self.header.search.menu.enable();
                self.data.save();
            },
            Action::Load(callback) => {
                let sender = self.sender;
                self.load(move || {
                    match callback() {
                        Ok(view) => sender.send(Action::ChangeView(view)),
                        Err(error) => {
                            println!("{}", error);
                        }
                    }
                })
            },
            Action::Back() => {
                self.header.disable_buttons();
                self.history.pop();
                if let Some(view) = self.history.pop() {
                    self.sender.send(Action::ChangeView(view));
                }
            },
            Action::ChangeView(view) => {
                self.views.loading.stop();
                self.header.search_button.set_sensitive(true);
                self.header.menu_button.set_sensitive(true);
                if !self.history.is_empty() {
                    self.header.back_button.set_sensitive(true);
                }
                self.history.push(view.clone());
                match view {
                    View::List(monsters) => {
                        self.views.list.build(&monsters);
                        self.views.stack.set_visible_child_name("list");
                    },
                    View::Collection() => {
                        self.views.stack.set_visible_child_name("collection");
                    },
                    View::Single(monster) => {
                        self.views.single.build(&monster);
                        self.header.set_title(Some(&monster.name), Some(&monster.family));
                        self.views.stack.set_visible_child_name("single");
                        if let Some(ref second_awakening_monster) = monster.second_awakening {
                            self.header.second_awakening_button.enable(second_awakening_monster);
                        }
                    }
                }
            }
        };
        glib::Continue(true)
    }

    fn connect_actions(&self) {
        //Quit
        let app = self.app.clone();
        let simple_action = gio::SimpleAction::new("quit", None);
        simple_action.connect_activate(move |_, _| app.quit());
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.quit", &["<primary>q"]);
        //Shortcuts
        let builder = gtk::Builder::new_from_file("data/shortcuts.ui");
        let dialog: gtk::ShortcutsWindow = builder.get_object("shortcuts").unwrap();
        self.window.set_help_overlay(Some(&dialog));
        self.app.set_accels_for_action("win.show-help-overlay", &["<primary>p"]);
        //Show Search
        let search = self.header.search_button.clone();
        let simple_action = gio::SimpleAction::new("show_search", None);
        simple_action.connect_activate(move |_, _| search.set_active(!search.get_active()));
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.show_search", &["<primary>f"]);
        //Back
        let simple_action = gio::SimpleAction::new("back", None);
        let sender = self.sender.clone();
        simple_action.connect_activate(move |_,_| sender.send(Action::Back()));
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.back", &["<primary>BackSpace"]);
        //About
        let window = self.window.clone();
        let simple_action = gio::SimpleAction::new("about", None);
        let app_info = self.info.clone();
        simple_action.connect_activate(move |_, _| {
            let dialog = gtk::AboutDialog::new();
            dialog.set_program_name(app_info.name);
            dialog.set_logo_icon_name(Some(app_info.id));
            dialog.set_authors(&app_info.authors);
            dialog.set_version(Some(app_info.version));
            dialog.set_website(Some(app_info.repository));
            dialog.set_transient_for(Some(&window));
            dialog.set_license_type(gtk::License::Gpl30);
            dialog.show();
        });
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.about", &["<primary>comma"]);
    }

    fn connect_events(&self) {
        let app = self.app.clone();
        let data = self.data.clone();
        self.window.connect_delete_event(move |_, _| {
            data.save();
            app.quit();
            gtk::Inhibit(false)
        });
    }

    fn load_styles(&self, css: &str) {
        let css_provider = gtk::CssProvider::new();
        let _ = css_provider.load_from_data(css.as_bytes());
        gtk::StyleContext::add_provider_for_screen(
            &self.window.get_screen().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER
        );
    }

}
