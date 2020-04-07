mod list;
mod single;
mod collection;

use crate::traits::SpinnerWidget;
use gtk::StackExt;

#[derive(Clone)]
pub struct Views {
    pub list: list::List,
    pub single: single::Single,
    pub loading: gtk::Spinner,
    pub collection: collection::Collection,
    pub stack: gtk::Stack
}

impl Views {
    pub fn new()-> Self {
        let stack = gtk::Stack::new();

        let single = single::Single::new();
        let list = list::List::new();
        let collection = collection::Collection::new();
        let loading = gtk::Spinner::new_loading();

        stack.set_transition_type(gtk::StackTransitionType::SlideLeft);
        stack.add_named(&loading, "loading");
        stack.add_named(&collection.container, "collection");
        stack.add_named(&list.container, "list");
        stack.add_named(&single.container, "single");

        Self {
            list,
            single,
            loading,
            collection,
            stack
        }
    }
}
