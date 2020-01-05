#![allow(dead_code)]
use gtk::{Image, ImageExt, Widget, ContainerExt, Frame};
use glib::{MainContext, Bytes};
use gio::{MemoryInputStream, Cancellable};
use gdk_pixbuf::Pixbuf;
use std::io::Read;
use glib::object::Cast;

pub struct ExternalImage<'a> {
    source: &'a str,
    dimensions: Option<[i32; 2]>,
    image: Image,
    border: bool,
}

impl <'a> ExternalImage <'a> {

    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            image: Image::new(),
            dimensions: None,
            border: false
        }
    }

    pub fn new_from_image(image: &Image, source: &'a str) -> Self {
        Self {
            source,
            image: image.clone(),
            dimensions: None,
            border: false
        }
    }

    pub fn border(mut self) -> Self {
        self.border = true;
        self
    }

    pub fn placeholder(self, path: &str, icon: bool) -> Self {
        match icon {
            true => {
                self.image.set_from_icon_name(Some(path), gtk::IconSize::Button);
                if let Some([width, _]) = self.dimensions {
                    self.image.set_pixel_size(width);
                }
            },
            false => {
                self.image.set_from_pixbuf(Some(&match self.dimensions {
                    Some([width, height]) => Pixbuf::new_from_file_at_scale(path, width, height, false),
                    None => Pixbuf::new_from_file(path)
                }.unwrap()));
            }
        }
        self
    }

    pub fn dimensions(mut self, width: i32, height: i32) -> Self {
        self.dimensions = Some([width, height]);
        self
    }

    fn get_bytes(source: String) -> Result<Vec<u8>, reqwest::Error> {
        let mut data = Vec::new();
        reqwest::get(&source)
        .map(|mut response|response.read_to_end(&mut data))
        .map(|_|data)
    }

    fn connect_receiver(&self, receiver: glib::Receiver<Result<Vec<u8>, reqwest::Error>>) {
        let dimensions = self.dimensions.clone();
        let image = self.image.clone();
        receiver.attach(None, move |bytes|{
            if let Err(error) = bytes {
                log::warn!("{}", error);
                return glib::Continue(false);
            }
            let stream = MemoryInputStream::new_from_bytes(&Bytes::from_owned(bytes.unwrap()));
            let pixbuf = match dimensions {
                Some([width, height]) => Pixbuf::new_from_stream_at_scale(
                    &stream,
                    width,
                    height,
                    false,
                    None::<&Cancellable>
                ),
                None => Pixbuf::new_from_stream(&stream, None::<&Cancellable>)
            }.unwrap();
            image.set_from_pixbuf(Some(&pixbuf));
            glib::Continue(false)
        });
    }

    fn get_widget(&self) -> Widget {
        if self.border {
            let frame = Frame::new(None);
            frame.add(&self.image);
            frame.upcast::<Widget>()
        } else {
            self.image.clone().upcast::<Widget>()
        }
    }

    pub fn load(&self) {
        use std::thread::spawn;
        let source = self.source.to_string().clone();
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
        self.connect_receiver(receiver);
        spawn(move || sender.send(Self::get_bytes(source)).unwrap());
    }

    pub fn build(&self) -> Widget {
        self.load();
        self.get_widget()
    }

    pub fn build_with_threadpool(&self, threadpool: &threadpool::ThreadPool) -> Widget {
        let source = self.source.to_string().clone();
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
        self.connect_receiver(receiver);
        threadpool.execute(move || sender.send(Self::get_bytes(source)).unwrap());
        self.get_widget()
    }

}
