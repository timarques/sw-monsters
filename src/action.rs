use crate::data_structs::{Collection, Monster};

#[derive(Clone, Debug)]
pub enum View {
    Stack(),
    Single(Monster),
    Search(String)
}

#[derive(Clone, Debug)]
pub enum Action {
    Ready(Vec<Collection<Monster>>),
    GetMonster(Monster),
    Back(),
    ChangeView(View)
}
