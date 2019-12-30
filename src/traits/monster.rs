use gtk::{Image, Box, Orientation, BoxExt};
use gdk_pixbuf::Pixbuf;

pub trait Monster {

    fn element(element: &str) -> Image {
        let path = format!("data/images/elements/{}.png", element);
        let pixbuf = Pixbuf::new_from_file_at_size(&path, 20, 20).unwrap();
        Image::new_from_pixbuf(Some(&pixbuf))
    }

    fn stars(stars: &i8) -> Box {
        let pixbuf = Pixbuf::new_from_file_at_size("data/images/star.svg", 10, 10).unwrap();
        let main_box = Box::new(Orientation::Horizontal, 0);
        for _ in 0 .. *stars {
            main_box.pack_start(&Image::new_from_pixbuf(Some(&pixbuf)), true, false, 0);
        }
        main_box
    }

}
