use gtk::{ScrolledWindow, Viewport, prelude::{WidgetExt, ContainerExt, ScrolledWindowExt}};
use libhandy::{Column, ColumnExt};
use crate::traits::ContainerWidget;

pub struct Container {
    pub scrolled_window: ScrolledWindow,
    pub viewport: Viewport,
    pub column: Column
}

impl Container {
    pub fn new() -> Self {
        let scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let viewport = Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let column = Column::new();
        viewport.add(&column);
        scrolled_window.add(&viewport);
        Self {scrolled_window, viewport, column}
    }

    pub fn go_top(&self) -> &Self {
        self.scrolled_window.set_vadjustment(Some(&gtk::Adjustment::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));
        self
    }

    pub fn width(&self, value: i32) -> &Self {
        self.column.set_maximum_width(value);
        self.column.set_linear_growth_width(value);
        self
    }

    pub fn margin(&self, value: i32) -> &Self {
        self.column.set_margin_start(value);
        self.column.set_margin_end(value);
        self
    }

    pub fn child<A>(&self, child: &A) -> &Self
    where A: glib::IsA<gtk::Widget> {
        self.column.remove_childs();
        self.column.add(child);
        self
    }

    pub fn get(&self) -> ScrolledWindow {
        self.scrolled_window.clone()
    }
}