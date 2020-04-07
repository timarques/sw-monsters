pub trait ActivableWidget: dyn_clone::DynClone {

    fn toggle(&self);

    fn clear(&self);

    fn on_change(&self, callback: Box<dyn Fn() + 'static>);

    fn get_value(&self) -> String;

}

dyn_clone::clone_trait_object!(ActivableWidget);
