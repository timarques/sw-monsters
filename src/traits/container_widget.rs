use gtk::ContainerExt;

pub trait ContainerWidget: ContainerExt {

    fn remove_childs(&self) {
        self.foreach(|child| self.remove(child));
    }

}
