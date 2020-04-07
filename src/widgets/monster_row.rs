use crate::data::Monster;
use crate::traits::LabelWidget;
use crate::widgets::{ExternalImage, Row, Size};
use crate::threadpool;
use gtk::{EventBox, Orientation};
use gtk::prelude::{BoxExt, WidgetExt, ContainerExt};

pub struct MonsterRow<'a> {
    monster: &'a Monster,
    family: bool,
    threadpool: Option<&'a threadpool::ThreadPool>,
    size: Size,
    callback: Option<Box<dyn Fn() + Send + 'static>>
}

impl <'a> MonsterRow <'a> {

    pub fn new(monster: &'a Monster) -> Self {
        Self {monster, family: false, threadpool: None, size: Size::Normal, callback: None}
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

    pub fn callback<A: Fn() + Send + 'static>(mut self, callback: A) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn build(mut self) -> EventBox {
        let element_and_stars = gtk::Box::new(Orientation::Horizontal, 0);
        element_and_stars.pack_start(&self.monster.element(), false, true, 0);
        element_and_stars.pack_start(&self.monster.stars(match self.size {
            Size::Small => 0,
            Size::Normal => 2
        }), false, true, 2);

        let title = match self.family {
            true => format!("{} ({})", self.monster.name, self.monster.family),
            false => self.monster.name.clone()
        };

        let image = ExternalImage::new(&self.monster.image);

        let image = match self.size {
            Size::Small => image.dimensions(25, 25),
            _ => image.dimensions(50, 50)
        }.placeholder("monster-symbolic", true);

        let image = match self.threadpool {
            Some(threadpool) => image.build_with_threadpool(threadpool),
            None => image.build()
        };

        let row = match self.size {
            Size::Small => {
                Row::new()
                    .orientation(Orientation::Horizontal)
                    .child(&element_and_stars)
                    .child(&gtk::Label::new_subtitle(&title))
                    .image(&image)
                    .build()
            },
            _ => {
                Row::new()
                    .child(&gtk::Label::new_subtitle(&title))
                    .child(&element_and_stars)
                    .image(&image)
                    .build()
            }
        };

        let event_box = EventBox::new();
        event_box.add(&row);

        if let Some(callback) = self.callback.take() {
            event_box.connect_button_press_event(move |_, _| {
                callback();
                gtk::Inhibit(false)
            });
        }

        event_box
    }

}
