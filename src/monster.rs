use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Essence {
    pub r#type: String,
    pub level: String,
    pub quantity: i8
}

#[derive(Deserialize, Clone, Debug)]
pub struct Awaken {
    pub bonus: String,
    pub essences: Vec<Essence>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Skill {
    pub name: String,
    pub r#type: String,
    pub description: String,
    pub multiplier: Option<String>,
    pub icon: String,
    pub skillups: Vec<String>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Monster {
    pub name: String,
    pub family: String,
    pub icon: String,
    pub stars: i8,
    pub element: String,
    pub awaken: Awaken,
    pub skills: Vec<Skill>
}
