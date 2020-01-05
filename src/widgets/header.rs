use crate::action::Action;
use crate::widgets::Search;
use crate::data_structs::Monster;
use gtk::{HeaderBar, StackSwitcher, Button, IconSize, MenuButton, PopoverMenu, Image, Box, Orientation, ModelButton};
use glib::Sender;
use gtk::prelude::*;

pub struct Header{
    pub container: HeaderBar,
    pub stack_switcher: StackSwitcher,
    pub back_button: Button,
    pub search_button: Button,
    pub second_awakening_button: Button,
    pub menu_button: MenuButton,
    sender: Sender<Action>
}

impl Header {
    pub fn new(sender: &Sender<Action>, search: &Search) -> Self {
        let stack_switcher = StackSwitcher::new();
        stack_switcher.show();

        let back_button = Button::new_from_icon_name(Some("go-previous-symbolic"), IconSize::Button);
        let search_button = Button::new_from_icon_name(Some("edit-find-symbolic"), IconSize::Button);
        let second_awakening_button = Self::get_second_awakening_button();
        let menu_button = Self::get_menu();

        let headerbar = HeaderBar::new();
        headerbar.set_show_close_button(true);
        headerbar.pack_start(&back_button);
        headerbar.pack_end(&menu_button);
        headerbar.pack_end(&search_button);
        headerbar.pack_end(&second_awakening_button);

        (cascade! {
            Header {
                container: headerbar,
                stack_switcher,
                back_button,
                search_button,
                second_awakening_button,
                menu_button,
                sender: sender.clone()
            };
            ..connect_events(search);
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
        self.menu_button.set_sensitive(false);
        self.second_awakening_button.hide();
    }

    pub fn setup_second_awakening_event(&self, monster: &Monster) {
        let sender = self.sender.clone();
        let monster = monster.clone();
        self.second_awakening_button.show();
        self.second_awakening_button.connect_clicked(move |_| {
            sender.send(Action::GetMonster(monster.clone())).unwrap();
        });
    }

    fn get_second_awakening_button() -> Button {
        let button = Button::new();
        button.add(&cascade! {
            gtk::Label::new(Some("Second Awakening"));
            ..set_single_line_mode(true);
            ..set_ellipsize(pango::EllipsizeMode::End);
            ..show();
        });
        button.set_no_show_all(true);
        button
    }

    fn get_menu() -> MenuButton {

        let about_button = ModelButton::new();
        about_button.set_label("About");
        about_button.set_action_name(Some("app.about"));

        let shortcuts_button = ModelButton::new();
        shortcuts_button.set_label("Shortcuts");
        shortcuts_button.set_action_name(Some("win.show-help-overlay"));

        let inner_menu = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..set_border_width(10);
            ..pack_start(&shortcuts_button, false, true, 2);
            ..pack_start(&about_button, false, true, 2);
            ..show_all();
        };

        let popover = PopoverMenu::new();
        popover.add(&inner_menu);

        let menu_button = MenuButton::new();
        menu_button.add(&Image::new_from_icon_name(Some("open-menu-symbolic"), IconSize::Button));
        menu_button.set_popover(Some(&popover));
        menu_button
    }

}
