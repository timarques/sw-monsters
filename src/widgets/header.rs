use crate::action::Action;
use crate::widgets::Search;
use gtk::{HeaderBar, StackSwitcher, Button, IconSize, MenuButton, PopoverMenu, Image, Box, Orientation, ModelButton};
use glib::Sender;
use gtk::prelude::*;

pub struct Header{
    pub container: HeaderBar,
    pub stack_switcher: StackSwitcher,
    pub back_button: Button,
    pub search_button: Button,
    sender: Sender<Action>
}

impl Header {
    pub fn new(sender: &Sender<Action>, search: &Search) -> Self {
        let stack_switcher = StackSwitcher::new();
        let back_button = Button::new_from_icon_name(Some("go-previous-symbolic"), IconSize::Button);
        let search_button = Button::new_from_icon_name(Some("edit-find-symbolic"), IconSize::Button);

        let headerbar = HeaderBar::new();
        headerbar.set_show_close_button(true);
        headerbar.pack_start(&back_button);
        headerbar.pack_end(&search_button);

        (cascade! {
            Header {
                container: headerbar,
                stack_switcher,
                back_button,
                search_button,
                sender: sender.clone()
            };
            ..set_title(None, None);
            ..connect_events(search);
            //..set_menu();
        })
    }

    pub fn set_title(&self, title: Option<&str>, subtitle: Option<&str>) {
        if title.is_none() && subtitle.is_none() {
            self.container.set_custom_title(Some(&self.stack_switcher));
        } else {
            self.container.set_custom_title(gtk::NONE_WIDGET);
            self.container.set_title(title);
            self.container.set_subtitle(subtitle);
        }
    }

    fn connect_events(&self, search: &Search)
    {
        let sender = self.sender.clone();
        self.back_button.connect_clicked(move |_|{
            sender.send(Action::Back()).unwrap();
        });
        let search = search.clone();
        self.search_button.connect_clicked(move |_| search.toggle_entry());
    }

    pub fn diable_buttons(&self) {
        self.search_button.set_sensitive(false);
        self.back_button.set_sensitive(false);
    }

    /*fn set_menu(&self) {

        let about_button = ModelButton::new();
        about_button.set_label("About");

        let shortcuts_button = ModelButton::new();
        shortcuts_button.set_label("Shortcuts");

        let inner_menu = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..set_border_width(10);
            ..pack_start(&about_button, false, true, 2);
            ..pack_start(&shortcuts_button, false, true, 2);
            ..show_all();
        };

        let popover = PopoverMenu::new();
        popover.add(&inner_menu);

        let menu_button = MenuButton::new();
        menu_button.add(&Image::new_from_icon_name(Some("open-menu-symbolic"), IconSize::Button));
        menu_button.set_popover(Some(&popover));
        self.container.pack_end(&menu_button);
    }*/

}
