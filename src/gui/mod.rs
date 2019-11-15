mod header;
mod views;

use crate::monster::Monster;
use crate::action::Action;
use header::Header;
use views::Content;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Gui {
    pub window: gtk::ApplicationWindow,
    header: Header,
    content: Content,
    receiver: RefCell<Option<glib::Receiver<Action>>>,
}

impl Gui{
    pub fn new(app: &gtk::Application, data: Vec<Monster>) -> Self {
        let (sender, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let header = Header::new(&sender);
        let window = gtk::ApplicationWindow::new(app);
        let content = Content::new(&sender, data.clone());
        let receiver = RefCell::new(Some(r));
        Gui {
            window,
            header,
            content,
            receiver
        }
    }

    pub fn init(&self)
    {
        self.header.stack_switcher.set_stack(Some(&self.content.views_stack));
        self.window.set_default_size(800, 600);
        self.window.set_titlebar(Some(&self.header.container));
        self.window.set_position(gtk::WindowPosition::Center);
        self.window.add(&self.content.stack);
        self.window.show_all();
        self.window.present();
    }

    pub fn load_styles(&self, css: &str) {
        let css_provider = gtk::CssProvider::new();
        let _ = css_provider.load_from_data(css.as_bytes());
        gtk::StyleContext::add_provider_for_screen(
            &self.window.get_screen().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER
        );
    }

    pub fn do_action(&self, action: Action) -> glib::Continue {
        match action {
            Action::Show(monster) => {
                let subtitle = format!("{} {}", &monster.family, &monster.element);
                self.content.single.build(&monster);
                self.header.container.set_title(Some(&monster.name));
                self.header.container.set_subtitle(Some(subtitle.as_str()));
                self.header.container.set_custom_title(gtk::NONE_WIDGET);
                self.content.stack.set_visible_child_name("single");
                self.header.back_button.set_sensitive(true);
            },
            Action::Back() => {
                self.content.stack.set_visible_child_name("views");
                self.header.back_button.set_sensitive(false);
                self.header.container.set_custom_title(Some(&self.header.stack_switcher));
            }
        };
        glib::Continue(true)
    }

    pub fn connect_events(&self, gui: Rc<Self>) {
        let receiver = self.receiver.borrow_mut().take().unwrap();
        receiver.attach(None, move |action| gui.do_action(action));
        self.window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            gio::signal::Inhibit(false)
        });
    }

}
