use crate::data_structs::{Monster, Skill, Essence, Collection, Stats, Fusion};
use crate::utils::{filters};
use select::{
    predicate::{
        Name,
        Class,
        Attr,
        Predicate,
        And,
        Not
    },
    node::{Find, Node},
    document::Document
};

fn parse_stats<A>(mut tables: Find<A>) -> Stats where A:Predicate {
    let table_data = tables.next().unwrap().find(Name("td"));
    let second_table_data = tables
        .next()
        .unwrap()
        .find(Name("tbody").descendant(Name("tr")))
        .last()
        .unwrap()
        .find(Name("td"));
    let data: Vec<i32> = table_data.chain(second_table_data).map(|data| {
        filters::only_numbers(
            match data.find(Name("span")).last() {
                Some(span) => span.text(),
                None => data.text()
            }.as_str()
        )
    }).collect();
    Stats {
        speed: data[0],
        critical_rate: data[1],
        critical_damage: data[2],
        resistance: data[3],
        accuracy: data[4],
        hp: data[7],
        attack: data[8],
        defense: data[9]
    }
}

fn parse_skill(element: Node) -> Skill {

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

    let image = element.find(Name("img")).next().unwrap().attr("src").unwrap().to_string();
    let name = element.find(Class("skill-title")).next().unwrap().text();
    let description = element.find(Class("description")).next().unwrap().text();
    let multiplier_element = element.find(Class("multiplier")).next();
    let multiplier = multiplier_element.map(|element| element.text().replace("Multiplier: ", ""));
    let effects = element
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
    let skillups = element
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
}

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
        let fusion_monsters: Vec<Monster> = chart.find(Class("sub")).map(|sub| {
            let mut names = sub.find(Class("name"));
            let mut nat4_monster = find_monster(monsters, names.next().unwrap().text());
            let elements: Vec<Monster> = names.map(|name| {
                let mut monster = find_monster(monsters, name.text());
                monster.fusion = Some(Box::new(Fusion{ used_in: Some(nat4_monster.clone()), recipe: None }));
                fusions_monsters.push(monster.clone());
                monster
            }).collect();
            nat4_monster.fusion = Some(Box::new(Fusion {
                used_in: Some(nat5_monster.clone()),
                recipe: match elements.is_empty() {
                    true => None,
                    false => Some(elements)
                }
            }));
            fusions_monsters.push(nat4_monster.clone());
            nat4_monster
        }).collect();
        nat5_monster.fusion = Some(Box::new(Fusion{
            used_in: None,
            recipe: Some(fusion_monsters)
        }));
        fusions_monsters.push(nat5_monster);
    }
    Ok(fusions_monsters)
}

pub fn get_monster(monster: Monster) -> Result<Monster, &'static str> {
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
    let mut stats_divisions = content.find(And(And(Class("wrapper"), Class("text-center")), Not(Class("portrait-container"))));
    monster.stats = Some(parse_stats(
        stats_divisions
        .next()
        .unwrap()
        .find(Class("stats-right").descendant(Name("table")))
    ));
    let name = monster.name.clone();
    for (index, element) in content.find(Class("wrapper")).enumerate() {
        if !element.attr("class").unwrap().contains("skills") { continue; }
        let r#type = content
            .find(Name("h2"))
            .nth(index)
            .unwrap()
            .text()
            .replace(&format!("{} ", name), "");
        let skills: Vec<Skill> = element.find(Class("skill")).map(|element| parse_skill(element)).collect();
        if skills.is_empty() { continue; }
        if r#type == "SECOND AWAKENING SKILLS" {
            let mut second_awaken_monster = monster.clone();
            second_awaken_monster.skills = vec![
                Collection {
                    r#type: String::from("Skills"),
                    elements: skills
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
            break;
        }
        monster.skills.push(Collection {
            elements: skills,
            r#type
        });
    }
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
            stats: None
        }
    }).collect())
}
