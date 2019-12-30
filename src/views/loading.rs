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
        spinner.set_property_expand(true);
        container.add(&spinner);
        Loading { container, spinner }
    }

    pub fn start(&self) {
        self.spinner.start();
    }

    pub fn stop(&self) {
        self.spinner.stop();
    }

}
