use super::data::Monster;
use super::error::Error;

#[derive(Clone)]
pub enum View {
    List(Vec<Monster>),
    Collection(),
    Single(Monster)
}

pub enum Action {
    Ready(),
    SyncFinished(),
    Load(Box<dyn Send + Sync + 'static + FnOnce() -> Result<View, Error>>),
    Back(),
    ChangeView(View)
}
