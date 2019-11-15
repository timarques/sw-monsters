use crate::monster::Essence;
use gtk::prelude::*;

impl Essence {

    pub fn to_widget(&self) -> gtk::ListBoxRow {
        let row = gtk::ListBoxRow::new();
        let box_widget = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let image_path = format!("data/icons/essence-of-{}-{}.png", self.r#type, self.level);
        let pixbuf = gdk_pixbuf::Pixbuf::new_from_file_at_scale(image_path.as_str(), 25, 25, true).unwrap();
        let image = gtk::Image::new_from_pixbuf(Some(&pixbuf));
        let text = format!("{} essences {} of {} ", self.quantity, self.level, self.r#type);
        let label = gtk::Label::new(Some(text.as_str()));
        box_widget.pack_start(&image, false, true, 0);
        box_widget.pack_start(&label, false, true, 6);
        box_widget.set_property_margin(6);
        row.add(&box_widget);
        row.set_selectable(false);
        row.set_activatable(false);
        row.show_all();
        row
    }

}
