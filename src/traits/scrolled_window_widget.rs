use gtk::{Viewport, ContainerExt, ScrolledWindowExt};
use libhandy::Column;
use glib::object::Cast;
use crate::widgets::Container;

impl <A: glib::IsA<gtk::ScrolledWindow> + gtk::ContainerExt> ScrolledWindowWidget for A {}

pub trait ScrolledWindowWidget: ScrolledWindowExt + ContainerExt {

    fn new_container() -> Container {
        Container::new()
    }

    fn go_top(&self) {
        self.set_vadjustment(Some(&gtk::Adjustment::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));
    }

    fn child<A: glib::IsA<gtk::Widget>>(&self, child: &A) {
        let viewport: Viewport = self.get_children()[0].clone().downcast().unwrap();
        let column: Column = viewport.get_children()[0].clone().downcast().unwrap();
        column.foreach(|child| column.remove(child));
        column.add(child);
    }

}
