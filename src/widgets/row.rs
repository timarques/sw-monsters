#![allow(dead_code)]
use gtk::{Label, Widget, Align, Orientation};
use gtk::prelude::{BoxExt, WidgetExt, StyleContextExt};
use glib::object::Cast;
use crate::traits::{LabelWidget, BoxWidget};

pub struct Row {
    image: Option<Widget>,
    childs: Vec<Widget>,
    margin: bool,
    orientation: Orientation
}

impl Row {

    pub fn new() -> Self {
        Self {
            image: None,
            childs: Vec::new(),
            margin: true,
            orientation: Orientation::Vertical
        }
    }

    pub fn image<A: glib::IsA<Widget>>(mut self, image: &A) -> Self {
        self.image = Some(image.clone().upcast::<Widget>());
        self
    }

    pub fn title(mut self, string: &str) -> Self {
        let label = Label::new(Some(string));
        label.title();
        self.childs.push(label.upcast::<Widget>());
        self
    }

    pub fn subtitle(mut self, string: &str) -> Self {
        let label = Label::new(Some(string));
        label.subtitle();
        self.childs.push(label.upcast::<Widget>());
        self
    }

    pub fn text(mut self, string: &str) -> Self {
        let label = Label::new(Some(string));
        label.text();
        self.childs.push(label.upcast::<Widget>());
        self
    }

    pub fn child<A: glib::IsA<gtk::Widget>>(mut self, child: &A) -> Self {
        self.childs.push(child.clone().upcast::<Widget>());
        self
    }

    pub fn childs<A: IntoIterator<Item = B>, B: glib::IsA<gtk::Widget>>(mut self, childs: A) -> Self {
        for child in childs {
            self.childs.push(child.clone().upcast::<Widget>());
        }
        self
    }

    pub fn optional_child<A: glib::IsA<gtk::Widget>>(mut self, child: Option<A>) -> Self {
        if let Some(child) = child {
            self.childs.push(child.clone().upcast::<Widget>());
        }
        self
    }

    pub fn without_margins(mut self) -> Self {
        self.margin = false;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn build(&self) -> gtk::Box {
        let wrapper = gtk::Box::new(self.orientation, 0);
        wrapper.set_valign(Align::Center);
        wrapper.pack_start_many(self.childs.clone(), false, true, 2);

        let widget = match &self.image {
            Some(image) => {
                let box_widget = gtk::Box::new(Orientation::Horizontal, 0);
                box_widget.pack_start(image, false, true, 0);
                box_widget.pack_start(&wrapper, false, true, 8);
                box_widget
            },
            None => wrapper
        };

        let style_context = widget.get_style_context();
        style_context.add_class("row");

        if self.margin {
            widget.set_property_margin(6);
        }
        widget
    }

}
