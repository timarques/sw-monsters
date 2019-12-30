#![allow(dead_code)]
use crate::data_structs::Monster;
use crate::action::Action;
use crate::traits::{Monster as MonsterTrait};
use crate::widgets::{ExternalImage, Row};
use gtk::{
    Box,
    EventBox,
    Orientation,
    prelude::{
        BoxExt,
        WidgetExt,
        ContainerExt
    }
};

pub struct MonsterRow<'a> {
    data: &'a Monster,
    family: bool,
    threadpool: Option<&'a threadpool::ThreadPool>
}

impl MonsterTrait for MonsterRow<'_> {}

impl <'a> MonsterRow <'a> {

    pub fn new(data: &'a Monster) -> Self {
        Self {data, family: false, threadpool: None}
    }

    pub fn family(mut self) -> Self {
        self.family = true;
        self
    }

    pub fn threadpool(mut self, threadpool: &'a threadpool::ThreadPool) -> Self {
        self.threadpool = Some(threadpool);
        self
    }

    pub fn build(&self, sender: &glib::Sender<Action>) -> EventBox {
        let bottom_box = cascade! {
            Box::new(Orientation::Horizontal, 0);
            ..pack_start(&Self::element(&self.data.element), false, true, 0);
            ..pack_start(&Self::stars(&self.data.stars), false, true, 2);
        };

        let title = match self.family {
            true => format!("{} ({})", self.data.name, self.data.family),
            false => self.data.name.clone()
        };

        let image = ExternalImage::new(&self.data.image).dimensions(50, 50).placeholder("data/images/monster.svg");

        let image = match self.threadpool {
            Some(threadpool) => image.build_with_threadpool(threadpool),
            None => image.build()
        };

        let row = Row::new().subtitle(&title).child(&bottom_box).image(&image).build();

        let sender = sender.clone();
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
