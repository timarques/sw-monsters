use gtk::WidgetExt;

impl SpinnerWidget for gtk::Spinner {}

pub trait SpinnerWidget {

    fn new_loading() -> gtk::Spinner {
        let spinner = gtk::Spinner::new();
        spinner.set_size_request(32, 32);
        spinner.set_property_expand(true);
        spinner.set_valign(gtk::Align::Center);
        spinner
    }

}
