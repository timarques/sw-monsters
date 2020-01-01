use gtk::{Spinner, Box, Orientation};
use gtk::prelude::{WidgetExt, SpinnerExt, ContainerExt};

pub struct Loading {
    pub spinner: gtk::Spinner,
    pub container: gtk::Box
}

impl Loading {

    pub fn new() -> Self {
        let spinner = Spinner::new();
        let container = Box::new(Orientation::Vertical, 0);
        spinner.set_size_request(50, 50);
        container.add(&spinner);
        container.set_property_expand(true);
        container.set_valign(gtk::Align::Center);
        Loading { container, spinner }
    }

    pub fn start(&self) {
        self.spinner.start();
    }

    pub fn stop(&self) {
        self.spinner.stop();
    }

}
