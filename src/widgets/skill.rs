use crate::data_structs::Skill as SkillDataStruct;
use crate::widgets::{ExternalImage, Row};
use crate::traits::{LabelWidget, BoxWidget};
use gdk_pixbuf::Pixbuf;
use gtk::{Box, Label, Align, Image, EventBox, Revealer, Orientation};
use gtk::prelude::{BoxExt, RevealerExt, ContainerExt, WidgetExt, LabelExt};
use glib::object::Cast;

pub struct Skill<'a> {
    data: &'a SkillDataStruct
}

impl <'a> Skill <'a> {

    pub fn new(data: &'a SkillDataStruct) -> EventBox {
        let skill = Self{data};
        let details = skill.details();
        let container = Row::new()
            .child(&skill.content())
            .child(&details)
            .build();
        container.set_child_packing(&details, false, true, 0, gtk::PackType::Start);
        let event_box = EventBox::new();
        event_box.add(&container);
        event_box.connect_button_press_event(move |_, _| {
            details.set_reveal_child(!details.get_reveal_child());
            gtk::Inhibit(false)
        });
        event_box
    }

    fn image(&self) -> Image {
        ExternalImage::new(&self.data.image).dimensions(25, 25).build().downcast::<Image>().unwrap()
    }

    fn content(&self) -> Box {
        Row::new()
        .image(&self.image())
        .title(&self.data.name)
        .without_margins()
        .orientation(Orientation::Horizontal)
        .optional_child(self.effects())
        .build()
    }

    fn details(&self) -> Revealer {

        let event_box = Row::new()
        .optional_child({
            self.data.multiplier.as_ref().map(|multiplier| cascade! {
                Label::new(Some(multiplier));
                ..subtitle();
            })
        })
        .text(&self.data.description)
        .optional_child(self.skillups())
        .build();

        let revealer = Revealer::new();
        revealer.add(&event_box);
        revealer
    }

    fn effects(&self) -> Option<Box> {
        self.data.effects.as_ref().map(|effects| {
            let mut images = Vec::new();
            for effect in effects {
                let image_path = format!("data/images/effects/{}.png", effect);
                if let Ok(pixbuf) = Pixbuf::new_from_file_at_size(&image_path, 20, 20) {
                    images.push(Image::new_from_pixbuf(Some(&pixbuf)));
                }
            }
            (cascade! {
                Box::new(Orientation::Horizontal, 0);
                ..set_hexpand(true);
                ..set_halign(Align::End);
                ..add_from_vec(&images, false, true, 2);
            })
        })
    }

    fn skillups(&self) -> Option<Box> {
        self.data.skillups.as_ref().map(|skillups| {
            let skillups = skillups.iter().map(|skillup| cascade! {
                Label::new(Some(&skillup));
                ..subtitle();
            });
            (cascade! {
                Box::new(Orientation::Vertical, 0);
                ..add_from_iterator(skillups, false, true, 0);
            })
        })
    }

}
