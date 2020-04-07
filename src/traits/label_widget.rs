use gtk::prelude::{StyleContextExt, LabelExt, WidgetExt};
use gtk::Label;

impl LabelWidget for gtk::Label {}

pub trait LabelWidget {

    fn new_text(string: &str) -> Label {
        let label = Label::new(Some(string));
        label.get_style_context().add_class("text");
        label.set_xalign(0.0);
        label.set_line_wrap(true);
        label
    }

    fn new_title(string: &str) -> Label {
        let label = Label::new(Some(string));
        label.get_style_context().add_class("title");
        label.set_xalign(0.0);
        label
    }

    fn new_subtitle(string: &str) -> Label {
        let label = Label::new(Some(string));
        label.get_style_context().add_class("subtitle");
        label.set_xalign(0.0);
        label.set_line_wrap(true);
        label
    }

}
