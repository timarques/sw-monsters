use crate::data::structs::monster::{Monster, Type as MonsterType, Element as MonsterElement};
use crate::data::structs::collection::Family;

pub enum SortType {
    Name,
    FamilyName,
    Stars
}

pub enum FilterType {
    Element(MonsterElement),
    Family(String),
    Type(MonsterType),
    Effect(SkillEffect),
    Fusion,
    SecondAwakening,
    Query(String)
}

pub trait DataOperations {

    fn group_by_families() {

    }

    fn sort(&self, sort_type: SortType) -> Self;

    fn filter(&self, filters: Vec<FilterType>) -> Self;

}

impl DataOperations for Vec<Monster> {

    fn group_by_families(&self) -> Vec<Family> {
        let mut families = Vec::new();
        let mut current_family = "".to_string();
        let mut monsters: Vec<Monster> = Vec::new();
        let data = self;
        for monster in data {
            if current_family != monster.family.name {
                families.push(Family::new(current_family, Some(monsters)));
                current_family = monster.family.name;
                monsters.clear();
            }
            family_monsters.push(monster.clone());
        }

        families.push(Family::new(data[data.len() - 1].family.name, Some(monsters)));
        families.remove(0);
        families
    }

    fn sort(&self, sort_type: SortType) -> Self {
        let data = self;
        match sort_type {
            SortType::Name => data.sort_by(|a, b| a.name.cmp(&b.name)),
            SortType::FamilyName => data.sort_by(|a, b| a.family.name.cmp(&b.family.name)),
            SortType::Stars => data.sort_by(|a, b| a.stars.cmp(&b.stars))
        };
        data
    }

    fn filter(&self, filters: Vec<FilterType>) -> Self {
        self.into_iter().filter(|monster| {
            for filter in filters {
                let filter = match filter {
                    FilterType::Element(element) => monster.element == element,
                    FilterType::Family(family_name) => monster.family.name == family_name,
                    FilterType::Type(monster_type) => monster.r#type == monster_type,
                    FilterType::Effect(effect) => {
                        for skill_collection in &monster.skills {
                            for skill in &skill_collection.elements {
                                if !skill.effects.is_empty() && skill.effects.contains(&effect) {
                                    return true;
                                }
                            }
                        }
                        false
                    },
                    FilterType::Fusion => monster.fusion.used_in.is_some() || monster.fusion.recipe.is_some(),
                    FilterType::SecondAwakening => monster.second_awakening.is_some(),
                    FilterType::Query(query) => monster.name.to_lowercase().contains(&query.to_lowercase())
                };
                if filter == true {
                    return true;
                }
            }
            false
        }).collect()
    }

}
