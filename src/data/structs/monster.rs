use crate::data::structs::collection::{Skills, Family};
use crate::data::structs::stats::Stats;
use crate::data::structs::essence::Essence;
use crate::data::structs::skill::Skill;
use crate::scraper;
use crate::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Element {
    Water,
    Wind,
    Fire,
    Light,
    Dark
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Type {
    Attack,
    Defense,
    Health,
    Support
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Monster {
    pub name: String,
    pub image: String,
    pub stars: i8,
    pub element: Element,
    pub essences: Vec<Essence>,
    pub skills: Vec<Skills>,
    pub r#type: Type,
    pub stats: Stats,
    pub source: String,
    pub fusion: Option<Vec<String>>,
    pub second_awakening: Option<Box<Monster>>,
    pub family: Family
}

impl Monster {

    pub fn new(
        name: String,
        image: String,
        stars: i8,
        element: String,
        family: String,
        source: String
    ) -> Self {
        Self {
            name,
            stars,
            source,
            image,
            element: Element::from(element),
            family: Family::new(family, None),
            essences: Vec::new(),
            skills: Vec::new(),
            r#type: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            stats: Stats::empty(),
            fusion: None,
            second_awakening: None
        }
    }

    pub fn second_awakening(monster: &Monster, image: String, stats: Stats, skills: Vec<Skill>) -> Self {
        let mut monster = monster.clone();
        monster.family = Family::new(format!("{} Second Awakening", monster.family.name), None);
        monster.image = image;
        monster.stats = stats;
        monster.skills = vec![Skills::new("Skills".to_string(), Some(skills))];
        monster
    }

    pub fn set_second_awakening(&mut self, monster: Monster) {
        self.second_awakening = Some(Box::new(monster));
    }

    pub fn set_type<A: Into<Type>>(&mut self, monster_type: A) {
        self.r#type = monster_type.into();
    }

    pub fn set_stats(&mut self, stats: Stats) {
        self.stats = stats;
    }

    pub fn set_essences(&mut self, essences: Vec<Essence>) {
        self.essences = essences;
    }

    pub fn set_skills(&mut self, skills: Vec<Skills>) {
        self.skills = skills;
    }

    pub fn set_fusion(&mut self, fusion: Vec<String>) {
        self.fusion = Some(fusion);
    }

    pub fn load(&mut self) -> Result<(), Error> {
        if self.skills.is_empty() {
            return scraper::single(&self).map(|data| {
                self.essences = data.essences;
                self.skills = data.skills;
                self.r#type = data.r#type;
                self.stats = data.stats;
                self.second_awakening = data.second_awakening;
            })
        }
        Ok(())
    }

}

impl From<Monster> for String {

    fn from(monster: Monster) -> Self {
        monster.name
    }

}

impl From<String> for Element {

    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "water" => Element::Water,
            "wind" => Element::Wind,
            "fire" => Element::Fire,
            "light" => Element::Light,
            "dark" => Element::Dark,
            _ => panic!("{} don't have a match", string)
        }
    }

}

impl From<Element> for String {

    fn from(element: Element) -> Self {
        match element {
            Element::Water => "water",
            Element::Wind => "wind",
            Element::Fire => "fire",
            Element::Light => "light",
            Element::Dark => "dark",
        }.to_string()
    }

}

impl From<String> for Type {

    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "atk" => Type::Attack,
            "attack" => Type::Attack,
            "hp" => Type::Health,
            "health" => Type::Health,
            "def" => Type::Defense,
            "defense" => Type::Defense,
            "sup" => Type::Support,
            "support" => Type::Support,
            _ => panic!("{} don't have a match", string)
        }
    }

}

impl From<Type> for String {

    fn from(monster_type: Type) -> Self {
        match monster_type {
            Type::Attack => "attack",
            Type::Defense => "defense",
            Type::Health => "health",
            Type::Support => "support"
        }.to_string()
    }

}
