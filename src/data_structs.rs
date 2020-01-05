#[derive(Clone, Debug)]
pub struct AppInfo {
    pub name: &'static str,
    pub id: &'static str,
    pub version: &'static str,
    pub authors: Vec<&'static str>,
    pub repository: &'static str
}

#[derive(Clone, Debug)]
pub struct Fusion {
    pub used_in: Option<Monster>,
    pub recipe: Option<Vec<Monster>>
}

#[derive(Clone, Debug)]
pub struct Collection<T> {
    pub r#type: String,
    pub elements: Vec<T>
}

#[derive(Clone, Debug)]
pub struct Essence {
    pub r#type: String,
    pub level: String,
    pub quantity: i8
}

#[derive(Clone, Debug)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub multiplier: Option<String>,
    pub image: String,
    pub skillups: Option<Vec<String>>,
    pub effects: Option<Vec<String>>
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
    pub second_awakening: Option<Box<Monster>>,
    pub family_elements: Option<Vec<Monster>>
}
