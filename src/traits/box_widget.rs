use glib::IsA;
use gtk::{Widget, prelude::BoxExt};

pub trait BoxWidget: BoxExt {

    fn add_if_some<A>(
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

    fn add_from_vec<A>(
        &self,
        elements:
        &Vec<A>,
        expand: bool,
        fill: bool,
        padding: u32
    ) where A: IsA<Widget> {
        for element in elements {
            self.pack_start(element, expand, fill, padding);
        }
    }

    fn add_from_iterator<'a, A, B>(
        &self,
        elements: A,
        expand: bool,
        fill: bool,
        padding: u32
    ) where
    A: Iterator<Item = B>,
    B: IsA<Widget> {
        for element in elements {
            self.pack_start(&element, expand, fill, padding);
        }
    }

}
