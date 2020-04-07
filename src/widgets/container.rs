use gtk::{ScrolledWindow, Viewport, WidgetExt, ContainerExt};
use libhandy::{Column, ColumnExt};

pub struct Container {
    scrolled_window: ScrolledWindow,
    column: Column
}

impl Container {
    pub fn new() -> Self {
        let scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let viewport = Viewport::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let column = Column::new();
        viewport.add(&column);
        scrolled_window.add(&viewport);
        Self {scrolled_window, column}
    }

    pub fn width(self, value: i32) -> Self {
        self.column.set_maximum_width(value);
        self.column.set_linear_growth_width(value);
        self
    }

    pub fn margin(self, value: i32) -> Self {
        self.column.set_margin_start(value);
        self.column.set_margin_end(value);
        self
    }

    pub fn build(self) -> ScrolledWindow {
        self.scrolled_window
    }

}
