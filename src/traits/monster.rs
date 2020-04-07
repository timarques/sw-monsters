use gtk::{Image, Box, Orientation, BoxExt, ImageExt};
use gdk_pixbuf::Pixbuf;

pub trait Monster {

    fn element(element: &str) -> Image {
        let path = format!("data/icons/elements/{}.png", element);
        let pixbuf = Pixbuf::new_from_file_at_size(&path, 20, 20).unwrap();
        Image::new_from_pixbuf(Some(&pixbuf))
    }

    fn stars(stars: i8, spacing: i8) -> Box {
        use std::convert::TryInto;
        let main_box = Box::new(Orientation::Horizontal, 0);
        for _ in 0 .. stars {
            let image = Image::new_from_icon_name(Some("starred-symbolic"), gtk::IconSize::unscaled());
            image.set_pixel_size(10);
            main_box.pack_start(&image, true, false, spacing.try_into().unwrap());
        }
        main_box
    }

}
