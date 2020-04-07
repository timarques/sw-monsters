use crate::widgets::{List, Row};
use crate::traits::{BoxWidget, ActivableWidget, LabelWidget};
use gtk::{WidgetExt, ListBoxRowExt, ContainerExt, ComboBoxExt, SwitchExt, TreeModelExt, prelude::ComboBoxExtManual};
use std::{rc::Rc};

impl ActivableWidget for gtk::Switch {

    fn toggle(&self) {
        self.set_active(!gtk::SwitchExt::get_active(self));
    }

    fn clear(&self) {
        self.set_active(false);
    }

    fn get_value(&self) -> String {
        gtk::SwitchExt::get_active(self).to_string()
    }

    fn on_change(&self, callback: Box<dyn Fn() + 'static>) {
        self.connect_property_active_notify(move |_| callback());
    }

}

impl ActivableWidget for gtk::ComboBox {

    fn toggle(&self) {
        self.emit_popup();
    }

    fn clear(&self) {
        self.set_active(Some(0));
    }

    fn get_value(&self) -> String {
        let iter = self.get_active_iter().unwrap();
        let model = self.get_model().unwrap();
        let value = model.get_value(&iter, 0).get::<String>();
        value.unwrap().unwrap()
    }

    fn on_change(&self, callback: Box<dyn Fn() + 'static>) {
        self.connect_changed(move |_| callback());
    }

}

#[derive(Clone)]
pub struct Component {
    pub name: String,
    pub widget: Box<dyn ActivableWidget>
}

#[derive(Clone)]
pub struct PreferencesRow {
    pub components: Vec<Component>,
    pub widgets: Vec<gtk::EventBox>,
    width: Option<i32>
}

impl PreferencesRow {

    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            components: Vec::new(),
            width: None
        }
    }

    pub fn add<A>(mut self, name: &str, widget: &A) -> Self
    where A: ActivableWidget + glib::IsA<gtk::Widget> {
        widget.set_halign(gtk::Align::End);
        widget.set_valign(gtk::Align::Center);
        widget.set_hexpand(true);

        if let Some(width) = self.width {
            widget.set_property_width_request(width);
        }

        let row = Row::new()
            .child(&gtk::Label::new_subtitle(&name))
            .child(widget)
            .orientation(gtk::Orientation::Horizontal)
            .build();
        row.set_property_height_request(34);
        row.set_margin_start(15);
        row.set_margin_end(15);

        let widget_clone = widget.clone();
        let event_box = gtk::EventBox::new();
        event_box.add(&row);
        event_box.connect_button_press_event(move |_, _| {
            widget_clone.toggle();
            gtk::Inhibit(true)
        });

        self.widgets.push(event_box);
        self.components.push(Component {name: name.to_string(), widget: Box::new(widget.clone())});
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn build(&self) -> gtk::Box {
        List::new().add_rows(self.widgets.clone(), |row| row.set_selectable(false)).build()
    }

}

#[derive(Clone)]
pub struct Preferences {
    pub rows: Vec<PreferencesRow>,
    callback: Rc<dyn Fn() + 'static>
}

impl Preferences {

    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            callback: Rc::new(||{})
        }
    }

    pub fn on_change(&mut self, callback: impl Fn() + 'static) -> &mut Self {
        self.callback = Rc::new(callback);
        self.update_widgets_callbacks();
        self
    }

    pub fn add_row(&mut self, row: PreferencesRow) -> &mut Self {
        self.rows.push(row);
        self
    }

    fn update_widgets_callbacks(&self) {
        for row in &self.rows {
            for component in &row.components {
                let callback = self.callback.clone();
                component.widget.on_change(Box::new(move || {
                    callback();
                }));
            }
        }
    }

    pub fn build(&self) -> gtk::Box {
        self.update_widgets_callbacks();
        let box_widget = gtk::Box::new(gtk::Orientation::Vertical, 0);
        box_widget.pack_start_many(self.rows.iter().map(|row| row.build()), false, true, 6);
        box_widget.show_all();
        box_widget.set_border_width(12);
        box_widget
    }

}
