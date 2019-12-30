use crate::action::Action;
use crate::data_structs::Monster;
use crate::widgets::{List, MonsterRow, Container};
use gtk::prelude::{WidgetExt, ListBoxRowExt};
use glib::Sender;

pub struct Search {
    pub container: Container,
    sender: Sender<Action>
}

impl Search {

    pub fn new(sender: &Sender<Action>) -> Self {
        let container = Container::new();
        container.margin(12).width(600);
        Self {container, sender: sender.clone()}
    }

    pub fn build(&self, monsters: &Vec<Monster>) {
        let threadpool = threadpool::ThreadPool::new(glib::get_num_processors() as usize);
        let childs = monsters.iter().map(|monster| MonsterRow::new(&monster).threadpool(&threadpool).family().build(&self.sender));
        let list = List::new().class("search").add_from_iterator(childs, |row| row.set_selectable(false)).build();
        list.show_all();
        list.set_margin_top(6);
        list.set_margin_bottom(12);
        self.container.child(&list).go_top();
    }

}
