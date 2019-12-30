use gtk::prelude::{StyleContextExt, LabelExt, WidgetExt};

pub trait LabelWidget: LabelExt + WidgetExt {

    fn text(&self) -> &Self {
        self.get_style_context().add_class("text");
        self.set_xalign(0.0);
        self.set_line_wrap(true);
        self
    }

    fn title(&self) -> &Self {
        self.get_style_context().add_class("title");
        self.set_xalign(0.0);
        self
    }

    fn subtitle(&self) -> &Self {
        self.get_style_context().add_class("subtitle");
        self.set_xalign(0.0);
        self
    }

}
