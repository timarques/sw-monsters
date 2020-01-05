use gtk::{Spinner, Box, Orientation};
use gtk::prelude::{WidgetExt, SpinnerExt, ContainerExt};

pub struct Loading {
    pub spinner: gtk::Spinner,
    pub container: gtk::Box
}

impl Loading {

    pub fn new() -> Self {
        let spinner = Spinner::new();
        let container = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..add(&spinner);
            ..set_property_expand(true);
            ..set_valign(gtk::Align::Center);
        };
        spinner.set_size_request(35, 35);
        Loading { container, spinner }
    }

    pub fn start(&self) {
        self.spinner.start();
    }

    pub fn stop(&self) {
        self.spinner.stop();
    }

}
