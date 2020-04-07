use crate::data::{Monster, Skill, Stats, Collection, Essence, Fusion};
use crate::widgets::{List, ExternalImage, Row, MonsterRow, Size};
use crate::traits::{BoxWidget, ScrolledWindowWidget, LabelWidget};
use gdk_pixbuf::Pixbuf;
use gtk::{Image, Box, Orientation, ScrolledWindow};
use gtk::prelude::*;

impl Monster {

    pub fn element(&self) -> gtk::Image {
        let path = format!("data/icons/elements/{}.png", self.element);
        let pixbuf = Pixbuf::new_from_file_at_size(&path, 20, 20).unwrap();
        Image::new_from_pixbuf(Some(&pixbuf))
    }

    pub fn stars(&self, space: i32) -> gtk::Box {
        let container = Box::new(Orientation::Horizontal, 0);
        container.pack_start_many((0..self.stars).collect::<Vec<_>>().iter().map(|_|{
            let image = Image::new_from_icon_name(Some("starred-symbolic"), gtk::IconSize::unscaled());
            image.set_pixel_size(10);
            image
        }), false, true, space as u32);
        container
    }

    pub fn single(&self) -> gtk::Box {

        fn header(this: &Monster) -> gtk::Box {
            let mut subtitle = format!("{} - ({}", this.family, this.r#type);
            if this.fusion.is_some() { subtitle = format!("{}, Fusion", subtitle); }
            if this.second_awakening.is_some() { subtitle = format!("{}, Second Awakening", subtitle); }
            subtitle = format!("{})", subtitle);

            let image = ExternalImage::new(&this.image)
                .dimensions(100, 100)
                .placeholder("monster-symbolic", true)
                .border()
                .build();

            let row = Row::new()
                .image(&image)
                .without_margins()
                .child(&gtk::Label::new_title(&this.name))
                .child(&gtk::Label::new_subtitle(&subtitle))
                .child(&{
                    let container = Box::new(Orientation::Horizontal, 0);
                    container.pack_start(&this.element(), false, true, 0);
                    container.pack_start(&this.stars(2), false, true, 2);
                    container
                }).build();
            row.set_margin_bottom(6);
            row
        }

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.set_margin_top(12);
        container.set_margin_bottom(12);
        container.pack_start(&header(self), false, true, 0);
        container.pack_start(&self.stats.widget(), false, true, 0);
        container.pack_start_many(self.skills.iter().map(|skill| skill.widget()), false, true, 0);
        container.pack_start(&{
            List::new()
            .title("Essences")
            .add_rows(self.essences.iter().map(|essence| essence.widget()), |row| {
                row.set_selectable(false);
                row.set_activatable(false);
            }).build()
        }, false, true, 0);
        container.pack_start(&{
            List::new()
            .title("Family")
            .add_rows(self.family_elements.iter().map(|monster| {
                    MonsterRow::new(&monster).family().size(Size::Small).build()
            }), |row| row.set_selectable(false))
            .build()
        }, false, true, 0);
        if let Some(ref fusion) = self.fusion {
            container.pack_start_if_some(fusion.target_monster(), false, true, 0);
            container.pack_start_if_some(fusion.recipe(), false, true, 0);
        }
        container.show_all();
        container
    }

}

impl Fusion {

    pub fn target_monster(&self) -> Option<gtk::Box> {
        /*self.target_monster.map(|monster| {
            List::new()
            .title(format!("Monster used to fuse"))// Fusão usada para
            .add_row(&MonsterRow::new(&monster, &sender).family().size(Size::Small).build(), |row| {
                row.set_selectable(false);
            }).build()
        })*/
        None
    }

    pub fn recipe(&self) -> Option<gtk::Box> {
        self.recipe.as_ref().map(|recipe| {
            List::new()
            .title("Fusion elements")
            .add_rows(recipe.iter().map(|monster| {
                MonsterRow::new(&monster)
                    .family()
                    .size(Size::Small)
                    .build()
            }), |row| row.set_selectable(false))
            .build()
        })
    }

}

impl Essence {

    pub fn widget(&self) -> gtk::Box {
        let image_path = format!("data/icons/essences/{}-{}.png", self.r#type, self.level);
        let pixbuf = Pixbuf::new_from_file_at_scale(&image_path, 25, 25, true).unwrap();
        let image = Image::new_from_pixbuf(Some(&pixbuf));
        let text = format!("{} essences {} of {} ", self.quantity, self.level, self.r#type);
        Row::new().image(&image).child(&gtk::Label::new_text(&text)).build()
    }

}

impl Stats {

    pub fn widget(&self) -> gtk::Box {
        List::new()
        .title("Stats")
        .add_rows({
            vec![
                ("Speed:", &self.speed.to_string()),
                ("HP:", &self.hp.to_string()),
                ("Attack:", &self.attack.to_string()),
                ("Defense:", &self.defense.to_string()),
                ("Critical Rate:", &format!("{}%", self.critical_rate)),
                ("Critical Damage:", &format!("{}%", self.critical_damage)),
                ("Accuracy:", &format!("{}%", self.accuracy)),
                ("Resistance:", &format!("{}%", self.resistance))
            ].iter().map(|(title, value)| {
                Row::new()
                    .orientation(gtk::Orientation::Horizontal)
                    .child(&gtk::Label::new_subtitle(&title))
                    .child(&gtk::Label::new_text(&value))
                    .build()
            })
        }, |row| {
            row.set_selectable(false);
            row.set_activatable(false);
        }).build()
    }

}

impl Collection<Skill> {

    pub fn widget(&self) -> gtk::Box {
        List::new()
        .title(&self.r#type)
        .add_rows({
            self.elements.iter().map(|element| element.widget())
        }, |row| {
            row.set_selectable(false)
        }).build()
    }

}

impl Skill {

    pub fn widget(&self) -> gtk::EventBox {
        let revealer = gtk::Revealer::new();
        revealer.add(&{
            Row::new()
            .optional_child(self.multiplier.as_ref().map(|multiplier| gtk::Label::new_subtitle(&multiplier)))
            .child(&gtk::Label::new_text(&self.description))
            .optional_child(self.skillups.as_ref().map(|skillups| {
                let labels = skillups.iter().map(|skillup| gtk::Label::new_subtitle(&skillup));
                let skillups = gtk::Box::new(gtk::Orientation::Vertical, 0);
                skillups.pack_start_many(labels, false, true, 0);
                skillups
            })).build()
        });

        let content = Row::new()
        .image(&ExternalImage::new(&self.image).dimensions(25, 25).build())
        .child(&gtk::Label::new_subtitle(&self.name))
        .without_margins()
        .orientation(Orientation::Horizontal)
        .optional_child(self.effects.as_ref().map(|effects| {
            let images = effects.iter().filter_map(|effect| {
                let image_path = format!("data/icons/effects/{}.png", effect);
                Pixbuf::new_from_file_at_size(&image_path, 20, 20).ok().map(|pixbuf| {
                    Image::new_from_pixbuf(Some(&pixbuf))
                })
            });
            let effects = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            effects.set_hexpand(true);
            effects.set_halign(gtk::Align::End);
            effects.pack_start_many(images, false, true, 2);
            effects
        })).build();

        let container = Row::new().child(&content).build();
        container.pack_start(&revealer, false, true, 0);

        let event_box = gtk::EventBox::new();
        event_box.add(&container);
        event_box.connect_button_press_event(move |_, _| {
            revealer.set_reveal_child(!revealer.get_reveal_child());
            gtk::Inhibit(false)
        });
        event_box
    }

}

#[derive(Clone)]
pub struct Single {
    pub container: ScrolledWindow
}

impl Single {

    pub fn new() -> Self {
        let container = ScrolledWindow::new_container().width(600).margin(12).build();
        Single { container }
    }

    pub fn build(&self, monster: &Monster) {
        self.container.go_top();
        self.container.child(&monster.single());
    }

}
