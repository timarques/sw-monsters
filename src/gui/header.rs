use crate::action::Action;
use gtk::prelude::*;

pub struct Header{
    pub container: gtk::HeaderBar,
    pub stack_switcher: gtk::StackSwitcher,
    pub back_button: gtk::Button
}

impl Header {
    pub fn new(sender: &glib::Sender<Action>) -> Self {
        let headerbar = gtk::HeaderBar::new();
        let stack_switcher = gtk::StackSwitcher::new();
        let back_button = gtk::Button::new_from_icon_name(Some("go-previous-symbolic"), gtk::IconSize::Button);
        back_button.set_sensitive(false);
        headerbar.set_show_close_button(true);
        headerbar.set_custom_title(Some(&stack_switcher));
        headerbar.add(&back_button);
        let header = Header {
            container: headerbar,
            stack_switcher,
            back_button
        };
        header.connect_events(&sender);
        header
    }

    fn connect_events(&self, sender: &glib::Sender<Action>)
    {
        let sender = sender.clone();
        self.back_button.connect_clicked(move |_|{
            sender.send(Action::Back()).unwrap();
        });
    }
}
