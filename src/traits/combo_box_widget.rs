use gtk::{ComboBox, ListStore, CellRendererText};
use gtk::prelude::{ComboBoxExt, GtkListStoreExtManual, CellLayoutExt, TreeModelExt, ObjectExt, ComboBoxExtManual};
use glib::types::StaticType;

impl <A: glib::IsA<gtk::ComboBox>> ComboBoxWidget for A {}

pub trait ComboBoxWidget: ComboBoxExt {

    fn new_with_data<'a, A>(elements: A) -> ComboBox
    where
        A: IntoIterator<Item = &'a str>,
    {
        let combo_box = ComboBox::new();
        let list_store = ListStore::new(&[String::static_type()]);
        for element in elements {
            list_store.insert_with_values(None, &[0], &[&element.to_string()]);
        }
        let renderer = CellRendererText::new();
        combo_box.set_active(Some(0));
        combo_box.set_model(Some(&list_store));
        combo_box.pack_start(&renderer, false);
        combo_box.add_attribute(&renderer, "text", 0);
        combo_box.set_row_separator_func(|treemodel, treeiter| {
            if let Some(value) = treemodel.get_value(treeiter, 0).get::<String>().unwrap() {
                return value == "_";
            }
            false
        });
        combo_box
    }

    //Text and ImagePath
    fn new_with_images<'a, A>(elements: A) -> ComboBox
    where
        A: IntoIterator<Item = (&'a str, Option<&'a str>)>
    {

        fn set_image(_layout: &gtk::CellLayout, renderer: &gtk::CellRenderer, model: &gtk::TreeModel, iter: &gtk::TreeIter) {
            let value: String = model.get_value(iter, 1).get().unwrap().unwrap();
            if value.as_str() == "" {
                return renderer.set_property("pixbuf", &gdk_pixbuf::NONE_PIXBUF_LOADER).unwrap();
            }
            renderer.set_property("pixbuf", &gdk_pixbuf::Pixbuf::new_from_file_at_size(&value, 15, 15).unwrap()).unwrap();
        }

        let list_store = ListStore::new(&[String::static_type(), String::static_type()]);
        for element in elements {
            list_store.insert_with_values(None, &[0, 1], &[&element.0.to_string(), &match element.1 {
                Some(path) => path,
                None => ""
            }]);
        }
        let pixbuf_renderer = gtk::CellRendererPixbuf::new();
        let text_renderer = CellRendererText::new();
        let combo_box = ComboBox::new();
        combo_box.set_model(Some(&list_store));
        combo_box.set_active(Some(0));
        combo_box.pack_start(&text_renderer, false);
        combo_box.pack_start(&pixbuf_renderer, false);
        combo_box.add_attribute(&text_renderer, "text", 0);
        combo_box.set_cell_data_func(&pixbuf_renderer, Some(Box::new(set_image)));
        combo_box
    }

}
