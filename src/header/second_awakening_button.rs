use crate::SENDER;
use crate::action::{Action, View};
use crate::data::Monster;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;

#[derive(Clone)]
pub struct SecondAwakeningButton {
    monster: Rc<RefCell<Option<Monster>>>,
    pub container: gtk::Button
}

impl SecondAwakeningButton {

    pub fn new() -> Self {
        let label = gtk::Label::new(Some("Second Awakening"));
        label.set_single_line_mode(true);
        label.set_ellipsize(pango::EllipsizeMode::End);
        label.show();
        let button = gtk::Button::new();
        button.add(&label);
        button.set_no_show_all(true);
        let this = Self {
            monster: Rc::new(RefCell::new(None)),
            container: button,
        };
        this.connect_click_event();
        this
    }

    fn connect_click_event(&self) {
        let monster = self.monster.clone();
        self.container.connect_clicked(move |this| {
            this.hide();
            SENDER.send(Action::ChangeView(View::Single(monster.borrow_mut().take().unwrap())));
        });
    }

    pub fn disable(&self) {
        self.container.hide();
    }

    pub fn enable(&self, monster: &Monster) {
        self.container.show();
        *self.monster.borrow_mut() = Some(monster.clone());
    }

}
