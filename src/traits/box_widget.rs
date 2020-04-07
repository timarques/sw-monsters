use glib::IsA;
use gtk::{Widget, prelude::BoxExt};

impl <A: glib::IsA<gtk::Box>> BoxWidget for A {}

pub trait BoxWidget: BoxExt {

    fn pack_start_if_some<A>(
        &self,
        element: Option<A>,
        expand: bool,
        fill: bool,
        padding: u32
    ) where A: IsA<Widget> {
        if let Some(element) = element {
            self.pack_start(&element, expand, fill, padding);
        }
    }

    fn pack_start_many<A: IntoIterator<Item = B>, B: glib::IsA<gtk::Widget>>(
        &self,
        childs: A,
        expand: bool,
        fill: bool,
        padding: u32
    )  {
        for child in childs {
            self.pack_start(&child, expand, fill, padding);
        }
    }

}
