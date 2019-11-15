use crate::monster::Monster;

#[derive(Clone)]
pub enum Action {
    Show(Monster),
    Back()
}
