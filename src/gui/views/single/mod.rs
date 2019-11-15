mod skill;
mod essence;

use crate::monster::{Monster, Skill, Essence};
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use libhandy::prelude::*;

pub struct Single {
    pub container: gtk::ScrolledWindow,
    main_box: gtk::Box,
    image: Option<gtk::Image>,
    title_label: Option<gtk::Label>,
    subtitle_label: Option<gtk::Label>,
    stars_container: Option<gtk::Box>,
    element_image: Option<gtk::Image>,
    skills_container: Option<gtk::ListBox>,
    essences_container: Option<gtk::ListBox>
}

impl Single {

    pub fn new() -> Self
    {
        let container = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let viewport = gtk::Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let column = libhandy::Column::new();
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        main_box.get_style_context().add_class("monster");
        column.set_maximum_width(800);
        column.add(&main_box);
        viewport.add(&column);
        container.add(&viewport);
        let mut single = Single {
            container,
            main_box,
            title_label: None,
            subtitle_label: None,
            image: None,
            stars_container: None,
            element_image: None,
            skills_container: None,
            essences_container: None
        };
        single.add_header();
        single.add_skills_container();
        single.add_essences_container();
        single
    }

    fn add_header(&mut self) {
        let header_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let title_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let image = gtk::Image::new();
        let title = gtk::Label::new(None);
        let subtitle = gtk::Label::new(None);
        let bottom_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let stars_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let element = gtk::Image::new();
        title.set_halign(gtk::Align::Start);
        title.get_style_context().add_class("title");
        subtitle.set_halign(gtk::Align::Start);
        subtitle.get_style_context().add_class("subtitle");
        bottom_box.pack_start(&element, false, true, 0);
        bottom_box.pack_start(&stars_box, false, true, 6);
        title_box.pack_start(&title, false, true, 2);
        title_box.pack_start(&subtitle, false, true, 2);
        title_box.pack_start(&bottom_box, false, true, 2);
        title_box.set_valign(gtk::Align::Center);
        stars_box.get_style_context().add_class("stars");
        header_box.get_style_context().add_class("header");
        header_box.pack_start(&image, false, true, 0);
        header_box.pack_start(&title_box, false, true, 12);
        self.main_box.pack_start(&header_box, false, true, 12);
        self.title_label = Some(title);
        self.image = Some(image);
        self.subtitle_label = Some(subtitle);
        self.stars_container = Some(stars_box);
        self.element_image = Some(element);
    }

    fn add_essences_container(&mut self) {
        let box_widget = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let label = gtk::Label::new(Some("Essences"));
        let list_box = gtk::ListBox::new();
        let frame = gtk::Frame::new(None);
        label.get_style_context().add_class("title");
        label.set_halign(gtk::Align::Start);
        list_box.get_style_context().add_class("essences");
        frame.add(&list_box);
        box_widget.pack_start(&label, false, true, 6);
        box_widget.pack_start(&frame, false, true, 6);
        self.main_box.pack_start(&box_widget, false, true, 0);
        self.essences_container = Some(list_box);
    }

    fn add_skills_container(&mut self) {
        let skills_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let skills_label = gtk::Label::new(Some("Skills"));
        let skills = gtk::ListBox::new();
        let skills_frame = gtk::Frame::new(None);
        skills.get_style_context().add_class("skills");
        skills_label.set_halign(gtk::Align::Start);
        skills_label.get_style_context().add_class("title");
        skills_frame.add(&skills);
        skills_box.pack_start(&skills_label, false, true, 6);
        skills_box.pack_start(&skills_frame, false, true, 6);
        self.main_box.pack_start(&skills_box, false, true, 0);
        self.skills_container = Some(skills);
    }

    fn set_skills(&self, skills: &Vec<Skill>) {
        let container = self.skills_container.as_ref().unwrap();
        container.foreach(|row| container.remove(row));
        for skill in skills {
            container.insert(&skill.to_widget(), -1);
        }
    }

    fn set_essences(&self, essences: &Vec<Essence>) {
        let container = self.essences_container.as_ref().unwrap();
        container.foreach(|box_widget| container.remove(box_widget));
        for essence in essences {
            container.insert(&essence.to_widget(), -1);
        }
    }

    fn set_stars(&self, quantity: &i8) {
        let container = self.stars_container.as_ref().unwrap();
        container.foreach(|star| container.remove(star));
        let pixbuf = Pixbuf::new_from_file_at_scale("data/icons/star.svg", 10, 10, true).unwrap();
        for _ in 0 .. *quantity {
            let icon = gtk::Image::new_from_pixbuf(Some(&pixbuf));
            icon.set_valign(gtk::Align::Center);
            icon.show();
            container.pack_start(&icon, true, false, 0);
        }
    }

    fn set_element_image(&self, element: &str) {
        let image = self.element_image.as_ref().unwrap();
        let icon_path = format!("data/icons/{}-large.png", element);
        let pixbuf = Pixbuf::new_from_file_at_scale(icon_path.as_str(), 20, 20, true);
        image.set_from_pixbuf(Some(pixbuf.as_ref().unwrap()));
    }

    fn set_image(&self, icon_name: &str) {
        let image = self.image.as_ref().unwrap();
        let icon_path = format!("data/icons/{}", icon_name);
        let pixbuf = Pixbuf::new_from_file_at_scale(icon_path.as_str(), 100, 100, true);
        image.set_from_pixbuf(Some(pixbuf.as_ref().unwrap()));
    }

    pub fn build(&self, monster: &Monster) {
        self.title_label.as_ref().unwrap().set_label(&monster.name);
        self.subtitle_label.as_ref().unwrap().set_label(&monster.family);
        self.set_image(&monster.icon);
        self.set_element_image(&monster.element);
        self.set_stars(&monster.stars);
        self.set_skills(&monster.skills);
        self.set_essences(&monster.awaken.essences);
    }

}
