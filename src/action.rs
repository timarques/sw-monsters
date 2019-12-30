use crate::data_structs::Monster;

#[derive(Clone, Debug)]
pub enum View {
    Stack(),
    Single(Monster),
    Search(String),
    Fusion(Monster)
}

#[derive(Clone, Debug)]
pub enum Action {
    CollectionReady(),
    FusionsReady(),
    GetMonster(Monster),
    Back(),
    ChangeView(View)
}
