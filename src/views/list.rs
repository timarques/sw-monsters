use crate::data::Monster;
use crate::traits::ScrolledWindowWidget;
use crate::widgets::{List as ListWidget, MonsterRow};
use crate::{THREAD_POOL};
use gtk::{WidgetExt, ListBoxRowExt};

#[derive(Clone)]
pub struct List {
    pub container: gtk::ScrolledWindow
}

impl List {

    pub fn new() -> Self {
        Self {
            container: gtk::ScrolledWindow::new_container().width(600).margin(12).build()
        }
    }

    pub fn build(&self, data: &Vec<Monster>) {
        let threadpool = THREAD_POOL.group(1);
        let childs = data.iter().map(|monster| {
            MonsterRow::new(&monster)
                .threadpool(&threadpool)
                .family()
                .build()
        });
        let list = ListWidget::new().add_rows(childs, |row| row.set_selectable(false)).build();
        list.show_all();
        list.set_margin_top(6);
        list.set_margin_bottom(12);
        self.container.child(&list);
        self.container.go_top();
    }

}
