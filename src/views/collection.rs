use crate::action::Action;
use crate::data_structs::{Monster, Collection as CollectionDataStruct};
use crate::widgets::{List, MonsterRow, Container};
use gtk::{
    Box,
    Orientation,
    prelude::{
        WidgetExt,
        BoxExt,
        ListBoxRowExt
    }
};

pub struct Collection {
    pub container: Container,
    main_box: Box,
    sender: glib::Sender<Action>
}

impl Collection {

    pub fn new(sender: &glib::Sender<Action>) -> Self {
        let main_box = Box::new(Orientation::Vertical, 0);
        let container = Container::new();
        container.margin(12);
        container.width(600);
        container.child(&main_box);
        Self {
            container,
            main_box,
            sender: sender.clone()
        }
    }

    pub fn build(&self, families: &Vec<CollectionDataStruct<Monster>>) {
        let threadpool = threadpool::ThreadPool::new(glib::get_num_processors() as usize);
        for family in families {
            self.main_box.pack_start(&List::new()
            .title(&family.r#type)
            .class("family")
            .add_rows(family.elements.iter().map(|monster| {
                MonsterRow::new(&monster, &self.sender).threadpool(&threadpool).build()
            }), |row| row.set_selectable(false))
            .build(), false, true, 6);
        }
        self.main_box.show_all();
    }

}
