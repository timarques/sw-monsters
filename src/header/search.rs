use crate::{DATA, SENDER};
use crate::data::Monster;
use crate::action::{Action, View};
use crate::header::search_menu::SearchMenu;
use gtk::prelude::*;
use libhandy::prelude::ColumnExt;

#[derive(Clone)]
pub struct Search {
    pub container: libhandy::Column,
    pub entry: gtk::SearchEntry,
    pub menu: SearchMenu
}

impl Search {

    pub fn new() -> Self {

        let menu = SearchMenu::new();

        let menu_button = gtk::MenuButton::new();
        menu_button.set_popover(Some(&menu.container));

        let entry = gtk::SearchEntry::new();
        entry.set_hexpand(true);

        let box_widget = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        box_widget.get_style_context().add_class("linked");
        box_widget.pack_start(&entry, false, true, 0);
        box_widget.pack_start(&menu_button, false, true, 0);

        let container = libhandy::Column::new();
        container.set_maximum_width(600);
        container.set_linear_growth_width(600);
        container.add(&box_widget);
        container.show_all();

        Self {
            container,
            entry,
            menu
        }
    }

    pub fn connect_events(&mut self) {
        let this = self.clone();
        self.entry.connect_activate(move |_| this.apply());
        let this = self.clone();
        self.menu.preferences.on_change(move || this.apply());
    }

    fn apply(&self) {
        let query = self.entry.get_buffer().get_text().to_lowercase();
        let preferences_rows = &self.menu.preferences.rows;

        let sort_by_value = preferences_rows[0].components[0].widget.get_value();
        let r#type = preferences_rows[1].components[0].widget.get_value();
        let element = preferences_rows[1].components[1].widget.get_value();
        let family = preferences_rows[1].components[2].widget.get_value();
        let effect = preferences_rows[1].components[3].widget.get_value();
        let fusion: bool = preferences_rows[2].components[0].widget.get_value().parse().unwrap();
        let second_awakening: bool = preferences_rows[2].components[1].widget.get_value().parse().unwrap();

        let mut monsters = DATA.get();
        match sort_by_value.as_str() {
            "Name" => monsters.sort_by(|a, b| a.name.cmp(&b.name)),
            "Family Name" => monsters.sort_by(|a, b| a.family.cmp(&b.family)),
            "Stars" => monsters.sort_by(|a, b| a.stars.cmp(&b.stars)),
            _ => {}
        }
        monsters = monsters.iter().filter(|monster| {
            (element == "All" || element.to_lowercase() == monster.element) &&
            (family == "All" || family == monster.family) &&
            (r#type == "All" || r#type.to_lowercase() == monster.r#type) &&
            (effect == "All" || Self::monster_has_skill_effect(&effect, monster)) &&
            fusion == monster.fusion.is_some() &&
            second_awakening == monster.second_awakening.is_some() &&
            monster.name.to_lowercase().contains(&query)
        }).collect();
        SENDER.send(Action::ChangeView(View::List(monsters.clone())));
    }

    pub fn init(&mut self) {
        self.menu.build();
        self.connect_events();
    }

    fn monster_has_skill_effect(effect: &str, monster: &Monster) -> bool {
        let effect = crate::utils::filters::slugify(effect);
        for skill_collection in &monster.skills {
            for skill in &skill_collection.elements {
                if skill.effects.is_some() && skill.effects.as_ref().unwrap().contains(&effect.to_string()) {
                    return true;
                }
            }
        }
        false
    }

}
