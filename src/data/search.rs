use crate::data::{Monster, MonsterType, MonsterElement, SkillEffect};

pub enum SortType {
    Default,
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

trait Trait {

    fn value(&self) -> Self;

    fn sort(&mut self, sort_type: SortType) -> Self;

    fn filter(&mut self, filters: Vec<FilterType>) -> Self;

}

impl Trait for Vec<Monster> {

    fn value(&self) -> Self {
        self
    }

    fn sort(&mut self, sort_type: SortType) -> Self {
        match sort_type {
            SortType::Default => self.sort_by(|a, b| a.id.cmp(&b.id)),
            SortType::Name => self.sort_by(|a, b| a.name.cmp(&b.name)),
            SortType::FamilyName => self.sort_by(|a, b| a.family.name.cmp(&b.family.name)),
            SortType::Stars => monsters.sort_by(|a, b| a.stars.cmp(&b.stars))
        }
    }

    fn filter(&mut self, filters: Vec<FilterType>) -> Self {
        self.iter().filter(|monster| {
            for filter in filters {
                match filter {
                    FilterType::Element(element) => monster.element == element,
                    FilterType::Family(family_name) => monster.family.name == family_name,
                    FilterType::Type(monster_type) => monster.r#type == monster_type,
                    FilterType::Effect(effect) => {
                        for skill_collection in &monster.skills {
                            for skill in &skill_collection.elements {
                                if !skill.effects.is_empty() && skill.effects.contains(effect) {
                                    return true;
                                }
                            }
                        }
                        false
                    },
                    FilterType::Fusion => monster.fusion.used_in.is_some() || monster.fusion.recipe.is_some(),
                    FilterType::SecondAwakening => monster.second_awakening.is_some(),
                    FilterType::Query(query) => monster.name.to_lowercase().contains(query.to_lowercase())
                }
            }
        })
    }

}
