use crate::action::{Action, View};
use gtk::prelude::*;
use libhandy::{Column, ColumnExt, SearchBar, SearchBarExt};
use gtk::{SearchEntry, EntryExt};
use glib::Sender;

#[derive(Debug, Clone)]
pub struct Search {
    pub container: SearchBar,
    entry: SearchEntry
}

impl Search {

    pub fn new(sender: &Sender<Action>) -> Self {
        let column = Column::new();
        let bar = SearchBar::new();
        let entry = SearchEntry::new();
        entry.set_placeholder_text(Some("Monster name..."));
        column.set_maximum_width(800);
        column.add(&entry);
        bar.connect_entry(&entry);
        bar.add(&column);
        let mut search = Self {container: bar, entry};
        search.connect_events(sender);
        search
    }

    pub fn toggle_entry(&self) {
        self.container.set_search_mode(!self.container.get_search_mode());
    }

    fn connect_events(&mut self, sender: &Sender<Action>) {
        let sender = sender.clone();
        let bar = self.container.clone();
        self.entry.connect_activate(move |entry| {
            let text = entry.get_buffer().get_text();
            if text.len() > 2 {
                bar.set_search_mode(false);
                sender.send(Action::ChangeView(View::Search(text))).unwrap();
            }
        });
    }

}
