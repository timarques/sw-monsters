#[derive(Clone, Debug, PartialEq)]
pub struct Fusion {
    pub parent: Option<Monster>,
    pub childs: Option<Vec<Monster>>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Collection<T> {
    pub r#type: String,
    pub elements: Vec<T>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Essence {
    pub r#type: String,
    pub level: String,
    pub quantity: i8
}

#[derive(Clone, Debug, PartialEq)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub multiplier: Option<String>,
    pub image: String,
    pub skillups: Option<Vec<String>>,
    pub effects: Option<Vec<String>>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    pub speed: i32,
    pub critical_rate: i32,
    pub critical_damage: i32,
    pub resistance: i32,
    pub accuracy: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32
}

#[derive(Clone, Debug, PartialEq)]
pub struct Monster {
    pub name: String,
    pub image: String,
    pub stars: i8,
    pub element: String,
    pub family: String,
    pub essences: Vec<Essence>,
    pub skills: Vec<Collection<Skill>>,
    pub r#type: Option<String>,
    pub stats: Option<Stats>,
    pub source: String,
    pub fusion: Option<Box<Fusion>>,
    pub second_awakening: Option<Box<Monster>>
}
