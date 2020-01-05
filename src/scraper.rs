use crate::data_structs::{Monster, Skill, Essence, Collection, Stats, Fusion};
use crate::utils::filters;
use select::{node::{Find, Node}, document::Document};
use select::predicate::{Name, Class, Attr, Predicate, And, Not};

pub fn get_fusions_monsters(monsters: &Vec<Monster>) -> Result<Vec<Monster>, &'static str> {
    fn find_monster(monsters: &Vec<Monster>, name: String) -> Monster {
        monsters.iter().find(|monster| monster.name == name).unwrap().clone()
    }
    let response = reqwest::get("https://summonerswarskyarena.info/fusion-hexagram-chart/");
    if response.is_err() { return Err("connection error"); }
    let document = Document::from_read(response.unwrap()).unwrap();
    let charts = document.find(Class("chart-section"));
    let mut fusions_monsters = Vec::new();
    for chart in charts {
        let name = chart.find(Class("name")).next().unwrap().text();
        let mut nat5_monster = find_monster(monsters, name);
        let recipe: Vec<Monster> = chart.find(Class("sub")).map(|sub| {
            let mut names = sub.find(Class("name"));
            let mut nat4_monster = find_monster(monsters, names.next().unwrap().text());
            let recipe: Vec<Monster> = names.map(|name| {
                let mut monster = find_monster(monsters, name.text());
                monster.fusion = Some(Box::new(Fusion{ used_in: Some(nat4_monster.clone()), recipe: None }));
                fusions_monsters.push(monster.clone());
                monster
            }).collect();
            nat4_monster.fusion = Some(Box::new(Fusion {
                used_in: Some(nat5_monster.clone()),
                recipe: match recipe.is_empty() {
                    true => None,
                    false => Some(recipe)
                }
            }));
            fusions_monsters.push(nat4_monster.clone());
            nat4_monster
        }).collect();
        nat5_monster.fusion = Some(Box::new(Fusion{
            used_in: None,
            recipe: Some(recipe)
        }));
        fusions_monsters.push(nat5_monster);
    }
    Ok(fusions_monsters)
}

pub fn get_monster(monster: &Monster) -> Result<Monster, &'static str> {

    fn parse_stats<A: Predicate>(mut tables: Find<A>) -> Stats {
        let table_data = tables.next().unwrap().find(Name("td"));
        let second_table_data = tables
            .next()
            .unwrap()
            .find(Name("tbody").descendant(Name("tr")))
            .last()
            .unwrap()
            .find(Name("td"));
        let mut data = table_data.chain(second_table_data).map(|data| {
            filters::only_numbers(
                match data.find(Name("span")).last() {
                    Some(span) => span.text(),
                    None => data.text()
                }.as_str()
            )
        });
        Stats {
            speed: data.next().unwrap(),
            critical_rate: data.next().unwrap(),
            critical_damage: data.next().unwrap(),
            resistance: data.next().unwrap(),
            accuracy: data.next().unwrap(),
            hp: data.nth(2).unwrap(),
            attack: data.next().unwrap(),
            defense: data.next().unwrap()
        }
    }

    fn parse_skills(skills_division: &Node) -> Vec<Skill> {

        fn filter_effect(effect: &str) -> String {
            let effect = effect
            .to_lowercase()
            .replace("atk", "attack")
            .replace("def", "defense")
            .replace("reduce", "decrease")
            .replace("status_", "")
            .replace("cri", "critical");
            match effect.as_str() {
                "reduce-acc" => "glancing",
                "attack-bar-up" => "increase-attack-bar",
                "attack-bar-down" => "decrease-attack-bar",
                "icon_duration_vampire-50x50" => "vampire",
                "oblivious" => "oblivion",
                "blessed" => "recovery",
                "counterattack" => "counter",
                _ => effect.as_str()
            }.to_string()
        }

        skills_division.find(Class("skill")).map(|skill_node| {
            let image = skill_node.find(Name("img")).next().unwrap().attr("src").unwrap().to_string();
            let name = skill_node.find(Class("skill-title")).next().unwrap().text();
            let description = skill_node.find(Class("description")).next().unwrap().text();
            let multiplier_element = skill_node.find(Class("multiplier")).next();
            let multiplier = multiplier_element.map(|element| element.text().replace("Multiplier: ", ""));
            let effects = skill_node
                .find(Class("buff-debuffs"))
                .next()
                .map(|element| element.find(Name("span")).map(|effect| {
                    filter_effect(
                        effect
                       .attr("style")
                       .unwrap()
                       .split("/")
                       .last()
                       .unwrap()
                       .split(".")
                       .next()
                       .unwrap()
                   )
               }).collect());
            let skillups = skill_node
                .find(Class("level-data").descendant(Name("p")))
                .next()
                .map(|element| element.text().split("\n").map(|s|s.to_string()).collect());
            Skill {
                name,
                image,
                description,
                multiplier,
                skillups,
                effects
            }
        }).collect()
    }

    let response = reqwest::get(&monster.source);
    if response.is_err() { return Err("connection error"); }
    let document = Document::from_read(response.unwrap()).unwrap();
    let article = document.find(Class("monster-page")).next().unwrap();
    let content = article.find(Class("content")).next().unwrap();
    let mut monster = monster.clone();
    monster.r#type = Some(filters::capitalize(
            &article
            .find(Class("stars")
            .descendant(Name("span")))
            .next()
            .unwrap()
            .text()
            .to_lowercase()
        )
    );
    monster.essences = content.find(Class("essences").descendant(Name("span"))).map(|essence| {
        let text = essence.text();
        let mut split = text.split(" ");
        let quantity = filters::only_numbers(split.next().unwrap()) as i8;
        let r#type = split.nth(2).unwrap().to_lowercase();
        let level = split.last().unwrap().to_lowercase();
        let level = filters::only_letters(&level).to_string();
        Essence {
            r#type,
            level,
            quantity
        }
    }).collect();
    let mut second_awaken_monster = monster.clone();
    let mut stats_divisions = content.find(And(And(Class("wrapper"), Class("text-center")), Not(Class("portrait-container"))));
    let skills_divisions = content.find(And(Class("wrapper"), Class("skills")));

    monster.stats = Some(parse_stats(
        stats_divisions
        .next()
        .unwrap()
        .find(Class("stats-right").descendant(Name("table")))
    ));

    let skills_divisions_vec: Vec<_> = skills_divisions.collect();
    let (last_skill_division, skills_divisions) = skills_divisions_vec.split_last().unwrap();
    let second_awakening_skills: Vec<Skill> = parse_skills(last_skill_division);

    if !second_awakening_skills.is_empty() {
        second_awaken_monster.skills = vec![
            Collection {
                r#type: String::from("Skills"),
                elements: second_awakening_skills
            }
        ];
        second_awaken_monster.stats = Some(parse_stats(stats_divisions.next().unwrap().find(Name("table"))));
        second_awaken_monster.family = format!("{} Second Awakening", second_awaken_monster.family);
        second_awaken_monster.image = document
            .find(Class("monster-images").descendant(Name("img")))
            .nth(2)
            .unwrap()
            .attr("src")
            .unwrap()
            .to_string();
        monster.second_awakening = Some(Box::new(second_awaken_monster));
    }

    for division in skills_divisions {
        let skills: Vec<Skill> = parse_skills(division);
        if skills.is_empty() { continue; }
        monster.skills.push(Collection {
            elements: skills,
            r#type: match division.prev().unwrap().text().contains("Transformed") {
                true => "Transformed Skills",
                false => "Skills"
            }.to_string()
        });
    };
    Ok(monster)
}



pub fn get_monsters() -> Result<Vec<Monster>, &'static str> {
    let response = reqwest::get("https://summonerswarskyarena.info/monster-list/");
    if response.is_err() { return Err("connection error"); }
    let document = Document::from_read(response.unwrap()).unwrap();
    let rows = document.find(Attr("id", "monster-list").descendant(Class("searchable")));
    Ok(rows.map(|row| {
        let mut td = row.find(Name("td"));
        let source = row.attr("data-link").unwrap().to_string();
        let element = row.attr("data-element").unwrap().to_string();
        let stars_text = td.next().unwrap().find(Name("span")).next().unwrap().text();
        let stars = stars_text.trim().parse::<i8>().unwrap();
        td.next();
        let family = td.next().unwrap().find(Name("h3")).next().unwrap().text();
        let image = td.next().unwrap().find(Name("img")).next().unwrap().attr("data-src").unwrap().to_string();
        let name = td.next().unwrap().text();
        Monster {
            name,
            element,
            stars,
            image,
            source,
            family,
            r#type: None,
            essences: Vec::new(),
            skills: Vec::new(),
            fusion: None,
            second_awakening: None,
            stats: None,
            family_elements: None
        }
    }).collect())
}
