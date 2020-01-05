use crate::scraper;
use crate::data_structs::{Monster, Collection, AppInfo};
use crate::action::{Action, View};
use crate::views::Views;
use crate::widgets::{Header, Search};
use std::{cell::RefCell, sync::{Arc, Mutex}, thread};
use glib::{Sender, Receiver, MainContext};
use gio::{SimpleAction, ApplicationExt, ActionMapExt};
use libhandy::SearchBarExt;
use gtk::{Application, ApplicationWindow, StyleContext, CssProvider, Box, Orientation};
use gtk::prelude::{
    WidgetExt, StackExt, GtkWindowExtManual, StackSwitcherExt, BoxExt, GtkWindowExt,
    ContainerExt, GtkApplicationExt, CssProviderExt, IconThemeExt, AboutDialogExt,
    BuilderExtManual, ApplicationWindowExt
};

pub struct App {
    app: Application,
    window: ApplicationWindow,
    header: Header,
    search: Search,
    views: Views,
    receiver: RefCell<Option<Receiver<Action>>>,
    sender: Sender<Action>,
    history: RefCell<Vec<Action>>,
    data: Arc<Mutex<Vec<Monster>>>,
    info: AppInfo
}

impl App{

    pub fn new(app: Application, info: &AppInfo) -> Self {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
        let views = Views::new(&sender);
        let search = Search::new(&sender);
        let header = Header::new(&sender, &search);
        header.set_title(Some(info.name), None);
        header.stack_switcher.set_stack(Some(&views.views_stack));
        let main_box = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..pack_start(&search.container, false, true, 0);
            ..pack_start(&views.stack, false, true, 0);
        };

        let window = cascade! {
            ApplicationWindow::new(&app);
            ..set_default_size(800, 600);
            ..set_size_request(400, 300);
            ..set_titlebar(Some(&header.container));
            ..set_position(gtk::WindowPosition::Center);
            ..add(&main_box);
            ..show_all();
            ..present();
        };
        let receiver = RefCell::new(Some(receiver));
        Self {
            app,
            window,
            header,
            search,
            views,
            receiver,
            sender,
            history: RefCell::new(Vec::new()),
            data: Arc::new(Mutex::new(Vec::new())),
            info: info.clone()
        }
    }

    fn load<A: Fn() + 'static + Send>(&self, callback: A) {
        self.views.loading.start();
        self.views.stack.set_visible_child_name("loading");
        thread::spawn(move || { callback() });
    }

    fn do_action(&self, action: Action) -> glib::Continue {
        let mut history = self.history.try_borrow_mut().unwrap();
        let action_clone = action.clone();
        self.header.diable_buttons();
        match action {
            Action::Ready(families) => {
                let data = self.data.lock().unwrap();
                self.views.collection.build(&families);
                self.sender.send(Action::ChangeView(View::Stack())).unwrap();
                self.views.fusions.build(&data.iter().cloned().filter(|monster| {
                    monster.stars == 5 && monster.fusion.is_some()
                }).collect());
            },
            Action::GetMonster(selected_monster) => {
                let sender = self.sender.clone();
                let data = self.data.clone();
                self.load(move || {
                    let mut data = data.lock().unwrap();
                    let mut monster = data.iter().cloned().find(|element|{
                        element.name == selected_monster.name
                    }).unwrap();
                    if monster.r#type.is_none() {
                        monster = scraper::get_monster(&monster).expect("Something happend while loading monster data");
                        *data = data.iter().cloned().map(|element| {
                            match element.name == monster.name {
                                true => monster.clone(),
                                false => element
                            }
                        }).collect();
                    }
                    if monster.image != selected_monster.image && monster.second_awakening.is_some() {
                        monster = *monster.second_awakening.unwrap();
                    }
                    sender.send(Action::ChangeView(View::Single(monster))).unwrap();
                });
            },
            Action::Back() => {
                history.pop();
                if let Some(action) = history.pop() {
                    self.sender.send(action).unwrap();
                }
            },
            Action::ChangeView(view) => {
                self.views.loading.stop();
                self.search.container.set_search_mode(false);
                self.header.search_button.set_sensitive(true);
                self.header.menu_button.set_sensitive(true);
                if !history.is_empty() {
                    self.header.back_button.set_sensitive(true);
                }
                history.push(action_clone);
                match view {
                    View::Stack() => {
                        self.header.set_title(None, None);
                        self.views.stack.set_visible_child_name("views");
                    },
                    View::Single(monster) => {
                        self.views.single.build(&monster);
                        self.header.set_title(Some(&monster.name), Some(&monster.family));
                        if let Some(second_awakening_monster) = &monster.second_awakening {
                            self.header.setup_second_awakening_event(&second_awakening_monster);
                        }
                        self.views.stack.set_visible_child_name("single");
                    },
                    View::Search(query) => {
                        let data = self.data.lock().unwrap();
                        let query_lowercase = query.to_lowercase();
                        let monsters: Vec<Monster> = data.iter().cloned().filter(|monster| {
                            monster.name.to_lowercase().contains(&query_lowercase) ||
                            monster.family.to_lowercase() == query_lowercase
                        }).collect();
                        if monsters.len() == 1 && monsters[0].name.to_lowercase() == query.to_lowercase() {
                            history.pop();
                            self.sender.send(Action::GetMonster(monsters[0].clone())).unwrap();
                        } else {
                            self.views.search.build(&monsters);
                            self.header.set_title(Some(&query), None);
                            self.views.stack.set_visible_child_name("search");
                        }
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

    fn add_icons(&self) {
        let theme = gtk::IconTheme::get_default().unwrap();
        theme.prepend_search_path("data/icons");
    }

    fn connect_actions(&self) {
        //Quit
        let app = self.app.clone();
        let simple_action = SimpleAction::new("quit", None);
        simple_action.connect_activate(move |_, _| app.quit());
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.quit", &["<primary>q"]);
        //Shortcuts
        let builder = gtk::Builder::new_from_file("data/shortcuts.ui");
        let dialog: gtk::ShortcutsWindow = builder.get_object("shortcuts").unwrap();
        self.window.set_help_overlay(Some(&dialog));
        self.app.set_accels_for_action("win.show-help-overlay", &["<primary>p"]);
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
        //About
        let window = self.window.clone();
        let info = self.info.clone();
        let simple_action = gio::SimpleAction::new("about", None);
        simple_action.connect_activate(move |_, _| {
            cascade! {
                gtk::AboutDialog::new();
                ..set_program_name(info.name);
                ..set_logo_icon_name(Some(info.id));
                ..set_authors(&info.authors);
                ..set_version(Some(info.version));
                ..set_website(Some(info.repository));
                ..set_transient_for(Some(&window));
                ..set_license_type(gtk::License::Gpl30);
                ..show();
            };
        });
        self.app.add_action(&simple_action);
        self.app.set_accels_for_action("app.about", &["<primary>comma"]);
    }

    fn init(&self) {
        let sender = self.sender.clone();
        let data = self.data.clone();
        self.header.diable_buttons();
        self.load(move || {
            let mut data = data.lock().unwrap();
            let initial_monsters = scraper::get_monsters().expect("Something happend while loading monsters");
            let fusions = scraper::get_fusions_monsters(&initial_monsters).expect("Something happend while loading fusions");
            let mut families = Vec::new();
            let mut current_family = String::from("");
            let mut family_monsters: Vec<Monster> = Vec::new();
            let mut monsters: Vec<Monster> = Vec::new();
            for monster in &initial_monsters {
                let monster = fusions.iter().find(|fusion_monster| {
                    fusion_monster.name == monster.name
                }).unwrap_or(monster).clone();
                if current_family != monster.family {
                    families.push(Collection{
                        r#type: current_family,
                        elements: family_monsters.clone()
                    });
                    current_family = monster.family.clone();
                    family_monsters.clear();
                }
                family_monsters.push(monster.clone());
                monsters.push(monster);
            }
            let last_monster = monsters.get(monsters.len() - 1).unwrap();
            families.push(Collection {
                r#type: last_monster.family.clone(),
                elements: family_monsters
            });
            families.remove(0);

            let families_clone = families.clone();
            *data = monsters.iter().map(move |monster| {
                let mut monster = monster.clone();
                let family_elements = families_clone.iter().find(|family| {
                    family.r#type == monster.family
                }).unwrap().elements.iter().cloned().filter(|family_element|{
                    family_element.name != monster.name
                }).collect();
                monster.family_elements = Some(family_elements);
                monster
            }).collect();
            sender.send(Action::Ready(families)).unwrap();
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
        self.add_icons();
        self.connect_actions();
        self.init();
    }

}
