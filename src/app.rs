use crate::scraper;
use crate::data_structs::{Monster, Collection};
use crate::action::{Action, View};
use crate::views::Views;
use crate::widgets::{Header, Search};
use std::{cell::RefCell, sync::{Arc, Mutex}, thread};
use glib::{Sender, Receiver, MainContext};
use gio::{SimpleAction, ApplicationExt, ActionMapExt};
use libhandy::SearchBarExt;
use gtk::{Application, ApplicationWindow, WindowPosition, StyleContext, CssProvider, Box, Orientation};
use gtk::prelude::{WidgetExt, StackExt, GtkWindowExtManual, StackSwitcherExt, BoxExt, GtkWindowExt, ContainerExt, GtkApplicationExt, CssProviderExt};

pub struct App {
    app: Application,
    window: ApplicationWindow,
    header: Header,
    search: Search,
    views: Views,
    receiver: RefCell<Option<Receiver<Action>>>,
    sender: Sender<Action>,
    history: RefCell<Vec<Action>>,
    data: Arc<Mutex<Vec<Monster>>>
}

impl App{
    pub fn new(app: Application) -> Self {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
        let views = Views::new(&sender);
        let search = Search::new(&sender);
        let header = Header::new(&sender, &search);
        let main_box = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..pack_start(&search.container, false, true, 0);
            ..pack_start(&views.stack, false, true, 0);
        };

        let window = cascade! {
            ApplicationWindow::new(&app);
            ..set_default_size(800, 600);
            ..set_titlebar(Some(&header.container));
            ..set_position(WindowPosition::Center);
            ..add(&main_box);
            ..show_all();
            ..present();
        };
        let receiver = RefCell::new(Some(receiver));
        header.stack_switcher.set_stack(Some(&views.views_stack));
        Self {
            app,
            window,
            header,
            search,
            views,
            receiver,
            sender,
            history: RefCell::new(Vec::new()),
            data: Arc::new(Mutex::new(Vec::new()))
        }
    }

    fn load<CB>(&self, callback: CB) where CB: Fn() + 'static + Send {
        self.views.loading.start();
        self.views.stack.set_visible_child_name("loading");
        self.header.diable_buttons();
        thread::spawn(move || { callback() });
    }

    pub fn do_action(&self, action: Action) -> glib::Continue {
        let mut history = self.history.try_borrow_mut().unwrap();
        let action_clone = action.clone();
        let data = self.data.lock().unwrap();
        match action {
            Action::CollectionReady() => {
                self.views.collection.build(&data);
                self.sender.send(Action::ChangeView(View::Stack())).unwrap();
            },
            Action::FusionsReady() => {
                self.views.fusions.build(&data.iter().cloned().filter(|monster| {
                    monster.stars == 5 && monster.fusion.is_some()
                }).collect());
            },
            Action::GetMonster(monster) => {
                let sender = self.sender.clone();
                let data = self.data.clone();
                self.load(move || {
                    let mut data = data.lock().unwrap();
                    if let Some(monster) = data
                    .iter()
                    .find_map(|element| {
                        if element.name != monster.name || element.r#type.is_none() { return None; }
                        Some(element.clone())
                    }).or_else(|| {
                        match scraper::get_monster(monster.clone()) {
                            Ok(monster) => {
                                *data = data.iter().map(|element| match element.name == monster.name {
                                    true => monster.clone(),
                                    false => element.clone()
                                }).collect();
                                Some(monster)
                            },
                            Err(_) => None
                        }
                    }){ sender.send(Action::ChangeView(View::Single(monster.clone()))).unwrap(); }
                });
            },
            Action::Back() => {
                history.pop();
                if let Some(action) = history.pop() { self.sender.send(action).unwrap(); }
                if history.is_empty() { self.header.back_button.set_sensitive(false); }
            },
            Action::ChangeView(view) => {
                self.search.container.set_search_mode(false);
                self.views.loading.stop();
                self.header.search_button.set_sensitive(true);
                if !history.is_empty() { self.header.back_button.set_sensitive(true); }
                history.push(action_clone);
                match view {
                    View::Fusion(monster) => {
                    },
                    View::Stack() => {
                        self.header.set_title(None, None);
                        self.views.stack.set_visible_child_name("views");
                    },
                    View::Single(monster) => {
                        self.views.single.build(&monster);
                        self.header.set_title(Some(&monster.name), Some(&monster.family));
                        self.views.stack.set_visible_child_name("single");
                    },
                    View::Search(query) => {
                        let query_lowercase = query.to_lowercase();
                        let monsters: Vec<Monster> = data.iter().filter_map(|monster| {
                            if monster.name.to_lowercase().contains(&query_lowercase) ||
                            monster.family.to_lowercase() == query_lowercase {
                                return Some(monster.clone());
                            }
                            None
                        }).collect();
                        if monsters.len() == 1 {
                            if monsters[0].name.to_lowercase() == query.to_lowercase() {
                                history.pop();
                                self.sender.send(Action::GetMonster(monsters[0].clone())).unwrap();
                                return glib::Continue(true);
                            }
                        }
                        self.views.search.build(&monsters);
                        self.header.set_title(Some(&query), None);
                        self.views.stack.set_visible_child_name("search");
                    }
                }
            }
        };
        glib::Continue(true)
    }

    pub fn load_styles(&self, css: &str) {
        let css_provider = CssProvider::new();
        let _ = css_provider.load_from_data(css.as_bytes());
        StyleContext::add_provider_for_screen(
            &self.window.get_screen().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER
        );
    }

    fn connect_actions(&self) {
        //Quit
        let app = self.app.clone();
        let simple_action = SimpleAction::new("quit", None);
        simple_action.connect_activate(move |_, _| app.quit());
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.quit", &["<primary>q"]);
        //Show Search
        let search = self.search.clone();
        let simple_action = SimpleAction::new("show_search", None);
        simple_action.connect_activate(move |_, _| search.toggle_entry());
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.show_search", &["<primary>f"]);
        //Back
        let sender = self.sender.clone();
        let simple_action = SimpleAction::new("back", None);
        simple_action.connect_activate(move |_,_| sender.send(Action::Back()).unwrap());
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.back", &["<primary>BackSpace"]);
    }

    fn init(&self) {
        let sender = self.sender.clone();
        let data = self.data.clone();
        self.load(move || {
            let mut data = data.lock().unwrap();
            let monsters = scraper::get_monsters().expect("Something happend while loading monsters");
            *data = monsters.clone();
            sender.send(Action::CollectionReady()).unwrap();
            let fusions = scraper::get_fusions_monsters(&monsters).expect("Something happend while loading fusions");
            *data = data.iter().map(|monster| {
                fusions.iter().find(|fusion_monster| {
                    fusion_monster.name == monster.name
                }).unwrap_or(monster).clone()
            }).collect();
            sender.send(Action::FusionsReady()).unwrap();
        });
    }

    pub fn run(&self, this: Arc<Self>) {
        let receiver = self.receiver.borrow_mut().take().unwrap();
        receiver.attach(None, move |action| this.do_action(action));
        let app = self.app.clone();
        self.window.connect_delete_event(move |_, _| {
            app.quit();
            gtk::Inhibit(false)
        });
        self.connect_actions();
        self.init();
    }

}
