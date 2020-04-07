use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Type {
    Magic,
    Fire,
    Water,
    Wind,
    Light,
    Dark
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Level {
    Low,
    Mid,
    High
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Essence {
    pub r#type: Type,
    pub level: Level,
    pub quantity: i8
}

impl Essence {

    pub fn new(essence_type: String, level: String, quantity: i8) -> Self {
        Self {
            r#type: Type::from(essence_type),
            level: Level::from(level),
            quantity
        }
    }

}

impl From<String> for Level {

    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "low" => Level::Low,
            "mid" => Level::Mid,
            "high" => Level::High,
            _ => panic!("{} don't have a match", string)
        }
    }

}

impl From<Level> for String {

    fn from(level: Level) -> Self {
        match level {
            Level::Low => "low",
            Level::Mid => "mid",
            Level::High => "high"
        }.to_string()
    }

}

impl From<String> for Type {

    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "magic" => Type::Magic,
            "fire" => Type::Fire,
            "water" => Type::Water,
            "wind" => Type::Wind,
            "light" => Type::Light,
            "dark" => Type::Dark,
            _ => panic!("{} don't have a match", string)
        }
    }

}

impl From<Type> for String {

    fn from(essence_type: Type) -> Self {
        match essence_type {
            Type::Magic => "magic",
            Type::Fire => "fire",
            Type::Water => "water",
            Type::Wind => "wind",
            Type::Light => "light",
            Type::Dark => "dark"
        }.to_string()
    }

}
