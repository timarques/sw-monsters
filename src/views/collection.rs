use crate::action::Action;
use crate::data_structs::{Monster, Collection as CollectionDataStruct};
use crate::widgets::{List, MonsterRow, Container};
use std::sync::Arc;
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

    pub fn new(sender: &glib::Sender<Action>) -> Arc<Self> {
        let main_box = Box::new(Orientation::Vertical, 0);
        let container = Container::new();
        container.margin(12).width(600).child(&main_box);
        Arc::new(Self {
            container,
            main_box,
            sender: sender.clone()
        })
    }

    fn get_families(&self, monsters: &Vec<Monster>) -> Vec<CollectionDataStruct<Monster>> {
        let mut families = Vec::new();
        let mut current_family = String::from("");
        let mut family_monsters: Vec<Monster> = Vec::new();
        for monster in monsters {
            if current_family != monster.family {
                families.push(CollectionDataStruct{
                    r#type: current_family,
                    elements: family_monsters.clone()
                });
                current_family = monster.family.clone();
                family_monsters.clear();
            }
            family_monsters.push(monster.clone());
        }
        let last_monster = monsters.get(monsters.len() - 1).unwrap();
        families.push(CollectionDataStruct {
            r#type: last_monster.family.clone(),
            elements: family_monsters
        });
        families.remove(0);
        families
    }

    pub fn build(&self, monsters: &Vec<Monster>) {
        let threadpool = threadpool::ThreadPool::new(glib::get_num_processors() as usize);
        let families = self.get_families(monsters);
        for family in families {
            let childs = family.elements.iter().map(|monster| MonsterRow::new(&monster).threadpool(&threadpool).build(&self.sender));
            self.main_box.pack_start(&List::new()
            .title(&family.r#type)
            .class("family")
            .add_from_iterator(childs, |row| row.set_selectable(false))
            .build(), false, true, 6);
        }
        self.main_box.show_all();
    }

}
