use crate::action::Action;
use crate::widgets::{Container, Row, ExternalImage, MonsterRow, List};
use crate::data_structs::{Monster as MonsterDataStruct};
use crate::traits::{BoxWidget, Monster as MonsterTrait, LabelWidget};
use gtk::{Box, Orientation, Revealer, Image, Label, FlowBox, FlowBoxChild, EventBox,
     ContainerExt, WidgetExt, BoxExt, ListBoxRowExt, RevealerExt, EventBoxExt, ExpanderExt,
     StyleContextExt, LabelExt
 };

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

    fn generate_mid_monster(&self, monster: &MonsterDataStruct) -> EventBox {
        let sender = self.sender.clone();
        let monster_clone = monster.clone();
        let image = ExternalImage::new(&monster.image)
            .placeholder("data/images/monster.svg")
            .dimensions(35, 35)
            .build();
        let wrapper = Box::new(Orientation::Vertical, 0);
        wrapper.pack_start(&image, false, true, 2);
        wrapper.pack_start(&{
            let label = Label::new(Some(&monster.name));
            label.get_style_context().add_class("subtitle");
            label
        }, false, true, 2);
        let event_box = EventBox::new();
        event_box.add(&wrapper);
        event_box.connect_button_press_event(move |_, _| {
            sender.send(Action::GetMonster(monster_clone.clone())).unwrap();
            gtk::Inhibit(false)
        });
        event_box
    }

    fn generate_bottom_monster(&self, monster: &MonsterDataStruct) -> EventBox {
        let sender = self.sender.clone();
        let monster_clone = monster.clone();
        let image = ExternalImage::new(&monster.image)
            .placeholder("data/images/monster.svg")
            .dimensions(20, 20)
            .build();
        let wrapper = Box::new(Orientation::Vertical, 0);
        wrapper.pack_start(&image, false, true, 2);
        wrapper.pack_start(&Label::new(Some(&monster.name)), false, true, 2);
        let event_box = EventBox::new();
        event_box.add(&wrapper);
        event_box.connect_button_press_event(move |_, _| {
            sender.send(Action::GetMonster(monster_clone.clone())).unwrap();
            gtk::Inhibit(false)
        });
        event_box
    }

    pub fn build(&self, monsters: &Vec<MonsterDataStruct>) {
        let rows = monsters.iter().map(|monster| {
            let childs = monster.fusion.as_ref().unwrap().recipe.as_ref().unwrap().iter().map(|monster| {
                let fusion = monster.fusion.as_ref().unwrap();
                let monster = self.generate_mid_monster(&monster);
                let fusion_wrapper = Box::new(Orientation::Vertical, 0);
                fusion_wrapper.pack_start(&monster, false, true, 2);
                if let Some(ref childs) = fusion.recipe {
                    let icon = Image::new_from_icon_name(Some("pan-down-symbolic"), gtk::IconSize::Button);
                    fusion_wrapper.pack_start(&icon, false, true, 2);
                    fusion_wrapper.add_childs(childs.iter().map(|monster|{
                        self.generate_bottom_monster(&monster)
                    }), false, true, 2);
                }
                fusion_wrapper
            });
            let wrapper = Box::new(Orientation::Horizontal, 0);
            wrapper.add_childs(childs, true, true, 0);
            wrapper.set_margin_top(6);
            let expander = gtk::Expander::new(None);
            expander.set_label_widget(Some(&cascade!{
                Label::new(Some("Monsters Needed"));
                ..subtitle();
            }));
            expander.add(&wrapper);
            expander.set_hexpand(true);
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
