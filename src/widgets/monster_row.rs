#![allow(dead_code)]
use crate::data_structs::Monster;
use crate::action::Action;
use crate::traits::{Monster as MonsterTrait};
use crate::widgets::{ExternalImage, Row, Size};
use gtk::{EventBox, Orientation};
use gtk::prelude::{BoxExt, WidgetExt, ContainerExt };
use glib::Sender;

pub struct MonsterRow<'a> {
    data: &'a Monster,
    family: bool,
    threadpool: Option<&'a threadpool::ThreadPool>,
    sender: &'a Sender<Action>,
    size: Size
}

impl MonsterTrait for MonsterRow<'_> {}

impl <'a> MonsterRow <'a> {

    pub fn new(data: &'a Monster, sender: &'a Sender<Action>) -> Self {
        Self {data, family: false, threadpool: None, sender, size: Size::Normal}
    }

    pub fn family(mut self) -> Self {
        self.family = true;
        self
    }

    pub fn threadpool(mut self, threadpool: &'a threadpool::ThreadPool) -> Self {
        self.threadpool = Some(threadpool);
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn build(&self) -> EventBox {
        let element_and_stars = cascade! {
            gtk::Box::new(Orientation::Horizontal, 0);
            ..pack_start(&Self::element(&self.data.element), false, true, 0);
            ..pack_start(&Self::stars(&self.data.stars), false, true, 2);
        };

        let title = match self.family {
            true => format!("{} ({})", self.data.name, self.data.family),
            false => self.data.name.clone()
        };

        let image = ExternalImage::new(&self.data.image);

        let image = match self.size {
            Size::Small => image.dimensions(25, 25),
            _ => image.dimensions(50, 50)
        }.placeholder("data/images/monster.svg");

        let image = match self.threadpool {
            Some(threadpool) => image.build_with_threadpool(threadpool),
            None => image.build()
        };

        let row = match self.size {
            Size::Small => {
                Row::new()
                    .orientation(Orientation::Horizontal)
                    .child(&element_and_stars)
                    .subtitle(&title)
                    .image(&image)
                    .build()
            },
            _ => Row::new().subtitle(&title).child(&element_and_stars).image(&image).build()
        };

        let sender = self.sender.clone();
        let data = self.data.clone();
        (cascade! {
            EventBox::new();
            ..add(&row);
            ..connect_button_press_event(move |_, _| {
                sender.send(Action::GetMonster(data.clone())).unwrap();
                gtk::Inhibit(false)
            });
        })
    }

}
