pub mod list;
pub mod single;

use crate::monster::Monster;
use crate::action::Action;
use single::Single;
use list::List;
use gtk::prelude::*;

pub struct Content {
    pub list: List,
    pub single: Single,
    pub stack: gtk::Stack,
    pub views_stack: gtk::Stack
}

impl Content {
    pub fn new(sender: &glib::Sender<Action>, data: Vec<Monster>)-> Self {

        let stack = gtk::Stack::new();
        let views_stack = gtk::Stack::new();
        let single = Single::new();
        let list = List::new(&sender, data.clone());
        let content = Content {
            list,
            single,
            stack,
            views_stack
        };
        content.init();
        content
    }

    fn init(&self) {
        self.stack.set_transition_type(gtk::StackTransitionType::SlideLeft);
        self.views_stack.add_titled(&self.list.container, "list", "Monsters");
        self.stack.add_named(&self.views_stack, "views");
        self.stack.add_named(&self.single.container, "single");
    }
}
