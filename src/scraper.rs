use crate::data::structs::{Monster, Stats, Skill, Essence};
use crate::data::structs::collection::Skills;
use crate::utils::filters;
use crate::error::Error;
use crate::traits::IntoResult;
use select::{node::Node, document::Document};
use select::predicate::{Name, Class, Attr, Predicate, And, Not};

pub fn single(original_monster: &Monster) -> Result<Monster, Error> {

    fn parse_stats<'a, A: Iterator<Item = Node<'a>>>(mut tables: A) -> Result<Stats, Error> {
        let table_data = tables.next().ok()?.find(Name("td"));
        let second_table_data = tables
            .next()
            .ok()?
            .find(Name("tbody")
            .descendant(Name("tr")))
            .last()
            .ok()?
            .find(Name("td"));

        let mut data = table_data
            .chain(second_table_data)
            .map(|data| filters::only_numbers(
                match data.find(Name("span")).last() {
                    Some(span) => span.text(),
                    None => data.text()
                }.as_str()
            ));
        Ok(
            Stats::builder()
                .speed(data.next().ok()?)
                .critical_rate(data.next().ok()?)
                .critical_damage(data.next().ok()?)
                .resistance(data.next().ok()?)
                .accuracy(data.next().ok()?)
                .health(data.nth(2).ok()?)
                .attack(data.next().ok()?)
                .defense(data.next().ok()?)
                .build()
        )
    }



    fn parse_skills<'a>(skills_division: &Node<'a>) -> Result<Vec<Skill>, Error> {

        skills_division.find(Class("skill")).map(|node| {
            let image = node
                .find(Name("img"))
                .next()
                .ok()?
                .attr("src")
                .unwrap()
                .to_string();
            let name = node
                .find(Class("skill-title"))
                .next()
                .ok()?
                .text();
            let description = node
                .find(Class("description"))
                .next()
                .ok()?
                .text();
            let multiplier = node
                .find(Class("multiplier"))
                .next()
                .map(|element| element.text().replace("Multiplier: ", ""));
            let effects: Option<Vec<String>> = node
                .find(Class("buff-debuffs"))
                .next()
                .and_then(|node| {
                    node.find(Name("span")).map(|node| {
                        node
                        .attr("style")
                        .and_then(|string| string.split("/").last())
                        .and_then(|string| string.split(".").next())
                        .map(|string| {
                            let effect = string
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
                        })
                    }).collect()
                });
            let skillups = node
                .find(Class("level-data").descendant(Name("p")))
                .next()
                .map(|element| element.text().split("\n").map(|s|s.to_string()).collect());

            Ok(Skill::new(
                name,
                description,
                multiplier,
                image,
                skillups,
                effects
            ))
        }).collect()
    }

    let response = reqwest::get(&original_monster.source);
    let document = Document::from_read(response?)?;
    let article = document.find(Class("monster-page")).next().ok()?;
    let content = article.find(Class("content")).next().ok()?;

    let mut stats_divisions = content.find(And(And(Class("wrapper"), Class("text-center")), Not(Class("portrait-container"))));
    let mut second_awakening_skills: Option<Vec<Skill>> = None;

    let mut monster = original_monster.clone();

    let monster_type = article
        .find(Class("stars").descendant(Name("span")))
        .next()
        .map(|element| filters::capitalize(&element.text().to_lowercase()))
        .ok()?;

    let stats = stats_divisions
        .next()
        .and_then(|element| {
            let tables = element.find(Class("stats-right").descendant(Name("table")));
            parse_stats(tables).ok()
        })
        .ok()?;

    let essences = content
        .find(Class("essences").descendant(Name("span")))
        .map(|node| {
            let text = node.text();
            let mut split = text.split(" ");
            let raw_quantity = split.next().ok()?.clone();
            let essence_type = split.nth(2).ok()?.to_lowercase();
            let raw_level = split.last().ok()?.to_lowercase();
            let essence_level = filters::only_letters(&raw_level).to_string();
            let essence_quantity = filters::only_numbers(&raw_quantity) as i8;
            Ok(Essence::new(essence_type, essence_level, essence_quantity))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let sections_name = content
        .find(Name("h2"))
        .filter_map(|node| {
            let text = node.text();
            match text.to_lowercase().contains("skills") {
                true => Some(text.replace(&format!("{} ", original_monster.name), "")),
                false => None
            }
        });

    let skills = content
        .find(And(Class("wrapper"), Class("skills")))
        .filter_map(|node| {
            let title = sections_name.next().ok();
            if title.is_err() { return title.err().map(|error| Err(error)); }
            let title = title.unwrap();

            let skills = parse_skills(&node);
            if skills.is_err() { return skills.err().map(|error| Err(error)); }
            let skills = skills.unwrap();

            if title == "SECOND AWAKENING SKILLS" {
                second_awakening_skills = Some(skills);
                return None;
            }

            Some(Ok(Skills::new(title, Some(skills))))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    if let Some(skills) = second_awakening_skills {
        let image = document
            .find(Class("monster-images").descendant(Name("img")))
            .nth(2)
            .map(|element| element.attr("src").unwrap().to_string())
            .ok()?;

        let stats = stats_divisions
            .next()
            .and_then(|element| parse_stats(element.find(Name("table"))).ok())
            .ok()?;

        monster.set_second_awakening(Monster::second_awakening(&monster, image, stats, skills));
    }

    monster.set_skills(skills);
    monster.set_essences(essences);
    monster.set_stats(stats);
    monster.set_type(monster_type);

    Ok(monster)
}



pub fn all() -> Result<Vec<Monster>, Error> {
    let response = reqwest::get("https://summonerswarskyarena.info/monster-list/");
    let document = Document::from_read(response?)?;
    let rows = document.find(Attr("id", "monster-list").descendant(Class("searchable")));

    rows.map(|row| {
        let mut td = row.find(Name("td"));
        let source = row.attr("data-link").ok()?.to_string();
        let element = row.attr("data-element").ok()?.to_string();
        let stars_text = td.next().unwrap().find(Name("span")).next().ok()?.text();
        let stars = stars_text.trim().parse::<i8>()?;
        td.next();
        let family = td
            .next()
            .ok()?
            .find(Name("h3"))
            .next()
            .ok()?
            .text();
        let image = td
            .next()
            .ok()?
            .find(Name("img"))
            .next()
            .ok()?
            .attr("data-src")
            .ok()?
            .to_string();
        let name = td.next().unwrap().text();
        Ok(Monster::new(name, image, stars, element, family, source))
    }).collect()
}
