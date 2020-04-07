use crate::data::{Monster, Collection as CollectionDataStruct};
use crate::widgets::{List, MonsterRow};
use crate::traits::ScrolledWindowWidget;
use crate::action::Action;
use crate::{DATA, THREAD_POOL, SENDER};
use gtk::prelude::*;

#[derive(Clone)]
pub struct Collection {
    pub container: gtk::ScrolledWindow,
    main_box: gtk::Box
}

impl Collection {

    pub fn new() -> Self {
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let container = gtk::ScrolledWindow::new_container().width(600).margin(12).build();
        container.child(&main_box);
        Self {
            container,
            main_box
        }
    }

    pub fn init(&self) {
        let threadpool = THREAD_POOL.group(2);
        for family in DATA.sort_by_families() {
            self.main_box.pack_start(&List::new()
            .title(&family.r#type)
            .add_rows(family.elements.iter().map(|monster| {
                let monster_clone = monster.clone();
                MonsterRow::new(monster)
                    .threadpool(&threadpool)
                    .callback(move || {

                    })
                    .build()
            }), |row| row.set_selectable(false))
            .build(), false, true, 6);
        }
        self.main_box.show_all();
    }

}
