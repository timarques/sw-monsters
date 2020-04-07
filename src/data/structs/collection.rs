use crate::data::structs::monster::Monster;
use crate::data::structs::skill::Skill;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub type Family = Collection<Monster>;
pub type Skills = Collection<Skill>;
pub type Fusion = Collection<String>;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Collection<T> {
    pub name: String,
    pub elements: Vec<T>
}

impl <T> Collection<T> {

    pub fn new(name: String, elements: Option<Vec<T>>) -> Self {
        Self {
            name,
            elements: elements.unwrap_or(Vec::new())
        }
    }

    pub fn elements(&mut self, elements: Vec<T>) {
        self.elements = elements;
    }

}
