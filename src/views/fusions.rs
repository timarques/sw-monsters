use crate::action::Action;
use crate::widgets::{Container, Row, ExternalImage, MonsterRow, List, Size};
use crate::data_structs::Monster;
use crate::traits::{BoxWidget, LabelWidget};
use gtk::{Box, Orientation, Image, Label, EventBox, Expander};
use gtk::prelude::{ContainerExt, WidgetExt, BoxExt, ListBoxRowExt, ExpanderExt, StyleContextExt};

pub struct Fusions {
    pub container: Container,
    sender: glib::Sender<Action>
}

impl Fusions {

    pub fn new(sender: &glib::Sender<Action>) -> Self {
        let sender = sender.clone();
        let container = Container::new();
        container.margin(12);
        container.width(600);
        Self {sender, container}
    }

    fn get_monster(&self, monster: &Monster, size: Size) -> EventBox {
        let sender = self.sender.clone();
        let monster_clone = monster.clone();
        let dimensions = match size {
            Size::Small => [20, 20],
            Size::Normal => [35, 35]
        };
        let image = ExternalImage::new(&monster.image)
            .placeholder("monster-symbolic", true)
            .dimensions(dimensions[0], dimensions[1])
            .build();

        let label = Label::new(Some(&monster.name));

        if let Size::Normal = size {
            label.get_style_context().add_class("subtitle");
        }

        let box_widget = cascade! {
            Box::new(Orientation::Vertical, 0);
            ..pack_start(&image, false, true, 0);
            ..pack_start(&label, false, true, 0);
        };

        (cascade! {
            EventBox::new();
            ..add(&box_widget);
            ..connect_button_press_event(move |_, _| {
                sender.send(Action::GetMonster(monster_clone.clone())).unwrap();
                gtk::Inhibit(false)
            });
        })
    }

    pub fn build(&self, monsters: &Vec<Monster>) {
        let rows = monsters.iter().map(|monster| {
            let recipes = monster.fusion.as_ref().unwrap().recipe.as_ref().unwrap();
            let childs = recipes.iter().map(|monster| {
                let box_widget = Box::new(Orientation::Vertical, 0);
                box_widget.pack_start(&self.get_monster(&monster, Size::Normal), false, true, 0);
                if let Some(ref monsters) = monster.fusion.as_ref().unwrap().recipe {
                    let icon = Image::new_from_icon_name(Some("pan-down-symbolic"), gtk::IconSize::Button);
                    box_widget.pack_start(&icon, false, true, 0);
                    box_widget.pack_start_many(monsters.iter().map(|monster|{
                        self.get_monster(&monster, Size::Small)
                    }), false, true, 0);
                }
                box_widget
            });
            let wrapper = cascade! {
                Box::new(Orientation::Horizontal, 0);
                ..pack_start_many(childs, true, true, 0);
                ..set_margin_top(6);
            };
            let expander = cascade! {
                Expander::new(None);
                ..set_label_widget(Some(&cascade!{
                    Label::new(Some("Monsters Needed"));
                    ..subtitle();
                }));
                ..add(&wrapper);
                ..set_hexpand(true);
            };
            Row::new()
                .child(&MonsterRow::new(&monster, &self.sender)
                    .family()
                    .build()
                )
                .child(&expander)
                .build()
        });
        let list = List::new().class("fusions").add_rows(rows, |row| {
            row.set_selectable(false);
        }).build();
        list.show_all();
        list.set_margin_top(6);
        list.set_margin_bottom(12);
        self.container.child(&list);
    }


}
