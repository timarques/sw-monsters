mod search;
mod preferences;
mod second_awakening_button;
mod search_menu;

use crate::SENDER;
use crate::action::Action;
use search::Search;
use gtk::prelude::*;
use libhandy::prelude::HeaderBarExt;

#[derive(Clone)]
pub struct Header {
    pub container: libhandy::HeaderBar,
    pub back_button: gtk::Button,
    pub search_button: gtk::ToggleButton,
    pub second_awakening_button: second_awakening_button::SecondAwakeningButton,
    pub menu_button: gtk::MenuButton,
    pub search: Search
}

impl Header {
    pub fn new() -> Self {
        let back_button = gtk::Button::new_from_icon_name(Some("go-previous-symbolic"), gtk::IconSize::Button);
        let search_button = gtk::ToggleButton::new();
        search_button.add(&gtk::Image::new_from_icon_name(Some("edit-find-symbolic"), gtk::IconSize::Button));
        let second_awakening_button = second_awakening_button::SecondAwakeningButton::new();

        let menu_button = {
            let about_button = gtk::ModelButton::new();
            about_button.set_label("About");
            about_button.set_action_name(Some("app.about"));

            let shortcuts_button = gtk::ModelButton::new();
            shortcuts_button.set_label("Shortcuts");
            shortcuts_button.set_action_name(Some("win.show-help-overlay"));

            let inner_menu = gtk::Box::new(gtk::Orientation::Vertical, 0);
            inner_menu.set_border_width(10);
            inner_menu.pack_start(&shortcuts_button, false, true, 2);
            inner_menu.pack_start(&about_button, false, true, 2);
            inner_menu.show_all();

            let popover = gtk::PopoverMenu::new();
            popover.add(&inner_menu);

            let menu_button = gtk::MenuButton::new();
            menu_button.add(&gtk::Image::new_from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::Button));
            menu_button.set_popover(Some(&popover));
            menu_button
        };

        let headerbar = libhandy::HeaderBar::new();
        headerbar.set_centering_policy(libhandy::CenteringPolicy::Strict);
        headerbar.set_show_close_button(true);
        headerbar.pack_start(&back_button);
        headerbar.pack_end(&menu_button);
        headerbar.pack_end(&search_button);
        headerbar.pack_end(&second_awakening_button.container);

        let header = Header {
            container: headerbar,
            back_button,
            search_button,
            second_awakening_button,
            menu_button,
            search: Search::new()
        };
        header.connect_events();
        header
    }

    fn connect_events(&self)
    {
        self.back_button.connect_clicked(move |_|{
            SENDER.send(Action::Back());
        });
        let headerbar = self.container.clone();
        let search = self.search.clone();
        self.search_button.connect_clicked(move |this| {
            if this.get_active() {
                headerbar.set_custom_title(Some(&search.container));
                search.entry.grab_focus();
            } else {
                headerbar.set_custom_title(gtk::NONE_WIDGET);
            }
        });
    }

    pub fn set_title(&self, title: Option<&str>, subtitle: Option<&str>) {
        if title.is_none() && subtitle.is_none() {
            self.container.set_custom_title(Some(&self.search.container));
        } else {
            self.container.set_custom_title(gtk::NONE_WIDGET);
            self.container.set_title(title);
            self.container.set_subtitle(subtitle);
        }
    }

    pub fn disable_buttons(&self) {
        self.search_button.set_sensitive(false);
        self.search_button.set_active(false);
        self.back_button.set_sensitive(false);
        self.menu_button.set_sensitive(false);
        self.second_awakening_button.disable();
    }

}
