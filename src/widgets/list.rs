#![allow(dead_code)]
use crate::traits::LabelWidget;
use gtk::prelude::*;
use glib::object::IsA;
use gtk::{
    Label,
    ListBox,
    ListBoxRow,
    Frame,
    Align,
    Orientation,
    Separator,
    Widget
};


pub struct List {
    label: Label,
    list_box: ListBox,
    border: bool
}

impl List {

    pub fn new() -> Self {
        let label = Label::new(None);
        let list_box = ListBox::new();
        label.title();
        label.set_halign(Align::Start);
        Self { label, list_box, border: true}
    }

    pub fn without_border(mut self) -> Self {
        self.border = false;
        self
    }

    pub fn class(self, class: &str) -> Self {
        self.list_box.get_style_context().add_class(class);
        self
    }

    pub fn title(self, title: &str) -> Self {
        self.label.set_label(title);
        self
    }

    pub fn add_row<A, B>(self, widget: &A, callback: B) -> Self
    where
        A: IsA<Widget>,
        B: Fn(&ListBoxRow)
    {
        let row = ListBoxRow::new();
        row.add(widget);
        self.list_box.add(&row);
        callback(&row);
        self
    }

    pub fn add_rows<A, B, C>(self, childs: A, callback: C) -> Self
    where
        A: IntoIterator<Item = B>,
        B:glib::IsA<gtk::Widget>,
        C: Fn(&ListBoxRow) + 'static + Send {
        for child in childs {
            let row = ListBoxRow::new();
            row.add(&child);
            callback(&row);
            self.list_box.add(&row);
        }
        self
    }

    fn set_header_separator(current: &gtk::ListBoxRow, _before: Option<&gtk::ListBoxRow>) {
        current.set_header(Some(&Separator::new(Orientation::Vertical)));
    }

    pub fn build(&self) -> gtk::Box {
        let box_widget = gtk::Box::new(Orientation::Vertical, 0);

        // get_label always returns Some when it should be None
        if self.label.get_label().unwrap() != "" {
            box_widget.pack_start(&self.label, false, true, 6);
        }
        let container = if self.border {
            self.list_box.set_header_func(Some(Box::new(Self::set_header_separator)));
            let frame = Frame::new(None);
            frame.add(&self.list_box);
            frame.upcast::<Widget>()
        } else {
            self.list_box.clone().upcast::<Widget>()
        };
        box_widget.pack_start(&container, false, true, 6);
        box_widget
    }

}
