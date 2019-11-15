use crate::monster::Monster;
use crate::action::Action;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;

impl Monster {

    pub fn to_column(&self, sender: glib::Sender<Action>) -> gtk::FlowBoxChild {
        let column = gtk::FlowBoxChild::new();
        let event_box = gtk::EventBox::new();
        let box_widget = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let title = gtk::Label::new(Some(&self.name));
        let image = {
            let path = format!("data/icons/{}", self.icon);
            let pixbuf = Pixbuf::new_from_file_at_scale(path.as_str(), 80, 80, true).unwrap();
            gtk::Image::new_from_pixbuf(Some(&pixbuf))
        };
        let stars = {
            let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            let pixbuf = Pixbuf::new_from_file_at_scale("data/icons/star.svg", 10, 10, true).unwrap();
            for _ in 0 .. self.stars {
                let icon = gtk::Image::new_from_pixbuf(Some(&pixbuf));
                icon.set_valign(gtk::Align::Center);
                icon.show();
                container.pack_start(&icon, true, false, 0);
            }
            container.get_style_context().add_class("stars");
            container
        };
        box_widget.pack_start(&image, false, true, 0);
        box_widget.pack_start(&title, false, true, 0);
        box_widget.pack_start(&stars, false, true, 0);
        event_box.add(&box_widget);
        let this = (*self).clone();
        event_box.connect_button_press_event(move |_, _| {
            let monster = this.clone();
            sender.send(Action::Show(monster)).unwrap();
            gtk::Inhibit(false)
        });
        column.add(&event_box);
        column.set_margin_bottom(12);
        column

    }

}

pub struct List {
    pub container: gtk::ScrolledWindow,
    viewport: gtk::Viewport,
    pub flow_box: gtk::FlowBox
}

impl List {

    pub fn new(sender: &glib::Sender<Action>, monsters: Vec<Monster>) -> Self {
        let container = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let viewport = gtk::Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let flow_box = gtk::FlowBox::new();
        for monster in monsters {
            flow_box.add(&monster.to_column(sender.clone()));
        };
        let list = List {container, viewport, flow_box};
        list.init();
        list
    }

    fn init(&self)
    {
        self.flow_box.set_homogeneous(true);
        //self.flow_box.set_max_children_per_line(4);
        self.flow_box.set_column_spacing(24);
        self.flow_box.set_halign(gtk::Align::Center);
        self.flow_box.set_valign(gtk::Align::Start);
        self.flow_box.set_property_margin(12);
        self.flow_box.set_selection_mode(gtk::SelectionMode::None);
        self.viewport.add(&self.flow_box);
        self.container.add(&self.viewport);
    }

}
