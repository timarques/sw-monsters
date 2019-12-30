#![allow(dead_code)]
use gtk::{Label, Widget, Align};
use gtk::prelude::{BoxExt, WidgetExt, StyleContextExt, LabelExt, BuilderExtManual};
use glib::object::Cast;
use crate::traits::{LabelWidget, BoxWidget};
use crate::widgets::{Orientation, Size};

pub struct Row {
    image: Option<Widget>,
    childs: Vec<Widget>,
    margin: bool,
    orientation: Orientation,
    size: Size
}

impl Row {

    pub fn new() -> Self {
        Self {
            image: None,
            childs: Vec::new(),
            margin: true,
            orientation: Orientation::Vertical,
            size: Size::Medium
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
        let builder = gtk::Builder::new_from_string("
            <?xml version=\"1.0\" encoding=\"UTF-8\"?>
            <interface>
                <object class=\"GtkLabel\" id=\"label\">
                    <property name=\"visible\">True</property>
                    <property name=\"can_focus\">False</property>
                    <property name=\"wrap\">True</property>
                    <property name=\"xalign\">0.0</property>
                    <attributes>
                    </attributes>
                    <style>
                        <class name=\"dim-label\" />
                    </style>
                </object>
            </interface>
        ");
        let label: Label = builder.get_object("label").unwrap();
        label.set_label(string);
        self.childs.push(label.upcast::<Widget>());
        self
    }

    pub fn child<B: glib::IsA<gtk::Widget>>(mut self, child: &B) -> Self {
        self.childs.push(child.clone().upcast::<Widget>());
        self
    }

    pub fn optional_child<B: glib::IsA<gtk::Widget>>(mut self, child: Option<B>) -> Self {
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

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn build(&self) -> gtk::Box {
        let wrapper = gtk::Box::new(self.orientation, 0);
        wrapper.set_valign(Align::Center);
        wrapper.add_from_vec(&self.childs, false, true, match self.size {
            Size::Small => 2,
            Size::Medium => 2,
            Size::Large => 4
        });

        let widget = match &self.image {
            Some(image) => {
                let box_widget = gtk::Box::new(Orientation::Horizontal, 0);
                box_widget.pack_start(image, false, true, 0);
                box_widget.pack_start(&wrapper, false, true, match self.size {
                    Size::Small => 4,
                    Size::Medium => 8,
                    Size::Large => 14
                });
                box_widget
            },
            None => wrapper
        };

        let style_context = widget.get_style_context();
        style_context.add_class("row");
        style_context.add_class(match self.size {
            Size::Small => "small",
            Size::Medium => "medium",
            Size::Large => "large"
        });

        if self.margin {
            widget.set_property_margin(match self.size {
                Size::Small => 2,
                Size::Medium => 6,
                Size::Large => 12
            });
        }
        widget
    }

}
