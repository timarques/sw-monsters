use crate::monster::Skill;
use gtk::prelude::*;

impl Skill {

    pub fn to_widget(&self) -> gtk::ListBoxRow {
        let container = gtk::ListBoxRow::new();
        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let event_box = gtk::EventBox::new();
        let body = {
            let icon_path = format!("data/icons/{}", &self.icon);
            let box_widget = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            let pixbuf = gdk_pixbuf::Pixbuf::new_from_file_at_scale(icon_path.as_str(), 25, 25, true);
            let image = gtk::Image::new_from_pixbuf(Some(&pixbuf.unwrap()));
            let name = gtk::Label::new(Some(&self.name));
            name.set_halign(gtk::Align::Start);
            name.get_style_context().add_class("subtitle");
            box_widget.pack_start(&image, false, true, 6);
            box_widget.pack_start(&name, false, true, 6);
            box_widget
        };
        let footer = {
            let revealer = gtk::Revealer::new();
            let details = gtk::Box::new(gtk::Orientation::Vertical, 0);
            let description = gtk::Label::new(Some(&self.description));
            if self.multiplier.is_some() {
                let multiplier = gtk::Label::new(Some(&self.multiplier.as_ref().unwrap()));
                multiplier.set_halign(gtk::Align::Start);
                multiplier.get_style_context().add_class("subtitle");
                details.pack_start(&multiplier, false, true, 6);
            }
            description.set_line_wrap(true);
            description.set_halign(gtk::Align::Start);
            description.set_xalign(0.0);
            description.get_style_context().add_class("text");
            details.pack_start(&description, false, true, 6);
            details.set_margin_start(6);
            details.set_margin_end(6);
            revealer.add(&details);
            revealer.set_reveal_child(false);
            revealer
        };
        {
            let revealer = footer.clone();
            event_box.connect_button_press_event(move |_, _|{
                if revealer.get_reveal_child() {
                    revealer.set_reveal_child(false);
                } else {
                    revealer.set_reveal_child(true);
                }
                gtk::Inhibit(false)
            });
        }
        main_box.pack_start(&body, false, true, 6);
        main_box.pack_start(&footer, false, true, 0);
        event_box.add(&main_box);
        container.get_style_context().add_class("row");
        container.add(&event_box);
        container.set_selectable(false);
        container.show_all();
        container
    }

}
