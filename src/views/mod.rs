mod collection;
mod single;
mod loading;
mod search;
mod fusions;

pub struct Views {
    pub collection: collection::Collection,
    pub single: single::Single,
    pub loading: loading::Loading,
    pub search: search::Search,
    pub fusions: fusions::Fusions,
    pub stack: gtk::Stack,
    pub views_stack: gtk::Stack
}

impl Views {
    pub fn new(sender: &glib::Sender<crate::action::Action>)-> Self {
        let stack = gtk::Stack::new();
        let views_stack = gtk::Stack::new();

        let single = single::Single::new(&sender);
        let collection = collection::Collection::new(&sender);
        let loading = loading::Loading::new();
        let search = search::Search::new(&sender);
        let fusions = fusions::Fusions::new(&sender);

        let views = Self {
            collection,
            single,
            loading,
            search,
            fusions,
            stack,
            views_stack
        };
        views.init();
        views
    }

    fn init(&self) {
        use gtk::StackExt;
        self.stack.set_transition_type(gtk::StackTransitionType::SlideLeft);
        self.views_stack.set_transition_type(gtk::StackTransitionType::SlideLeft);
        self.stack.add_named(&self.views_stack, "views");
        self.stack.add_named(&self.single.container.get(), "single");
        self.stack.add_named(&self.loading.container, "loading");
        self.stack.add_named(&self.search.container.get(), "search");
        self.views_stack.add_titled(&self.collection.container.get(), "collection", "Collection");
        self.views_stack.add_titled(&self.fusions.container.get(), "fusions", "Fusions");
    }
}
