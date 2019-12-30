use crate::action::Action;
use crate::widgets::{Container, List, Row, ExternalImage};
use crate::data_structs::{Monster as MonsterDataStruct};
use gtk::{Box, Orientation, WidgetExt, ListBoxRowExt};
use std::sync::Arc;

pub struct Fusions {
    pub container: Container,
    sender: glib::Sender<Action>
}

impl Fusions {

    pub fn new(sender: &glib::Sender<Action>) -> Arc<Self> {
        let sender = sender.clone();
        let main_box = Box::new(Orientation::Vertical, 0);
        let container = Container::new();
        container.margin(12).width(600).child(&main_box);
        Arc::new(Self {sender, container})
    }

    pub fn build(&self, monsters: &Vec<MonsterDataStruct>) {
        let childs = monsters.iter().map(|monster| {
            Row::new()
            .title(&monster.name)
            .subtitle(&monster.family)
            .image(&ExternalImage::new(&monster.image).dimensions(100, 100).placeholder("data/images/monster.svg").build())
            .build()
        });
        let list = List::new().class("fusions").add_from_iterator(childs, |row| row.set_selectable(false)).build();
        list.show_all();
        list.set_margin_top(6);
        list.set_margin_bottom(12);
        self.container.child(&list);
    }


}
