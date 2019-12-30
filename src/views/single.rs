use crate::action::{Action, View};
use crate::data_structs::Monster;
use crate::widgets::{List, ExternalImage, Container, Row, Skill as SkillWidget};
use crate::traits::{Monster as MonsterTrait, BoxWidget, ContainerWidget};
use gdk_pixbuf::Pixbuf;
use gtk::{Image, Button, Box, Orientation};
use gtk::prelude::*;
use std::cell::RefCell;

pub struct Single {
    pub container: Container,
    sender: glib::Sender<Action>,
    data: RefCell<Option<Monster>>,
    main_box: Box
}

impl MonsterTrait for Single {}

impl Single {

    pub fn new(sender: &glib::Sender<Action>) -> Self {
        let sender = sender.clone();
        let container = Container::new();
        let main_box = Box::new(Orientation::Vertical, 0);
        main_box.set_property_margin(12);
        container.margin(12).width(600).child(&main_box);
        Single {
            container,
            sender,
            main_box,
            data: RefCell::new(None)
        }
    }

    fn get_header(&self) -> Box {
        let data = self.data.borrow();
        let data = data.as_ref().unwrap();
        let image = ExternalImage::new(&data.image)
            .dimensions(100, 100)
            .placeholder("data/images/monster.svg")
            .border()
            .build();
        let row = Row::new()
            .image(&image)
            .without_margins()
            .title(&data.name)
            .subtitle(&format!(
                "{} ({})",
                &data.family,
                &data.r#type.as_ref().unwrap().to_uppercase()
            ))
            .child(&cascade! {
                Box::new(Orientation::Horizontal, 0);
                ..pack_start(&Self::element(&data.element), false, true, 0);
                ..pack_start(&Self::stars(&data.stars), false, true, 2);
            })
            .build();
        row.set_margin_bottom(6);
        row
    }

    fn get_buttons(&self) -> gtk::ButtonBox {
        let data = self.data.borrow();
        let data = data.as_ref().unwrap();
        let button_box = gtk::ButtonBox::new(Orientation::Horizontal);
        button_box.set_layout(gtk::ButtonBoxStyle::Start);
        button_box.add_if_some(data.fusion.as_ref().map(|_monsters| Button::new_with_label("Fusion")), false, true, 2);
        button_box.add_if_some(data.second_awakening.as_ref().map(|monster| {
            let sender = self.sender.clone();
            let monster = *monster.clone();
            let button = Button::new_with_label("Second Awakening");
            button.connect_clicked(move |_| {
                sender.send(Action::ChangeView(View::Single(monster.clone()))).unwrap();
            });
            button
        }), false, true, 2);
        button_box
    }

    fn get_stats(&self) -> Box {
        let data = self.data.borrow();
        let stats = data.as_ref().unwrap().stats.as_ref().unwrap();

        fn get_row(title: &str, value: &str) -> Box {
            Row::new()
                .orientation(Orientation::Horizontal)
                .subtitle(&title)
                .text(&value)
                .build()
        }

        let stats = vec![
            get_row("Speed:", &stats.speed.to_string()),
            get_row("HP:", &stats.hp.to_string()),
            get_row("Attack:", &stats.attack.to_string()),
            get_row("Defense:", &stats.defense.to_string()),
            get_row("Critical Rate:", &format!("{}%", stats.critical_rate)),
            get_row("Critical Damage:", &format!("{}%", stats.critical_damage)),
            get_row("Accuracy:", &format!("{}%", stats.accuracy)),
            get_row("Resistance:", &format!("{}%", stats.resistance))
        ];

        List::new()
        .title("Stats")
        .class("stats")
        .add_from_vec(stats, |row| {
            row.set_selectable(false);
            row.set_activatable(false);
        }).build()
    }

    fn get_skills(&self) -> Vec<Box> {
        let data = self.data.borrow();
        let skills = &data.as_ref().unwrap().skills;
        let mut lists = Vec::new();
        for collection in skills {
            let childs = collection.elements
                .iter()
                .map(|element| SkillWidget::new(element));
            let list = List::new()
                .class("skills")
                .title(&collection.r#type)
                .add_from_iterator(childs, |row| row.set_selectable(false))
                .build();
            lists.push(list);
        }
        lists
    }

    fn get_essences(&self) -> Box{
        let data = self.data.borrow();
        let essences = &data.as_ref().unwrap().essences;
        let childs = essences.iter().map(|essence|{
            let image_path = format!("data/images/essences/{}-{}.png", essence.r#type, essence.level);
            let pixbuf = Pixbuf::new_from_file_at_scale(image_path.as_str(), 25, 25, true).unwrap();
            let text = format!("{} essences {} of {} ", essence.quantity, essence.level, essence.r#type);
            let image = Image::new_from_pixbuf(Some(&pixbuf));
            Row::new().image(&image).text(&text).build()
        });

        List::new()
        .title("Essences")
        .class("essences")
        .add_from_iterator(childs, |row| {
            row.set_selectable(false);
            row.set_activatable(false);
        }).build()
    }

    pub fn build(&self, monster: &Monster) {
        *self.data.borrow_mut() = Some(monster.clone());
        self.container.go_top();
        self.main_box.remove_childs();
        self.main_box.pack_start(&self.get_header(), false, true, 0);
        self.main_box.pack_start(&self.get_stats(), false, true, 0);
        self.main_box.add_from_vec(&self.get_skills(), false, true, 0);
        self.main_box.pack_start(&self.get_essences(), false, true, 0);
        self.main_box.pack_start(&self.get_buttons(), false, true, 0);
        self.main_box.show_all();
    }

}
