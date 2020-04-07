pub mod filters {

    pub fn only_numbers(string: &str) -> i32 {
        let regex = regex::Regex::new(r"\d+").unwrap();
        match regex.find(string) {
            Some(result) => result.as_str().parse().unwrap(),
            None => 0
        }
    }

    pub fn only_letters(string: &str) -> &str {
        let regex = regex::Regex::new("[a-z]+").unwrap();
        match regex.find(string) {
            Some(result) => result.as_str(),
            None => ""
        }
    }

    pub fn capitalize(string: &str) -> String {
        let mut chars = string.chars();
        chars.next().unwrap().to_uppercase().chain(chars).collect()
    }

    pub fn remove_slug(string: &str) -> String {
        let words = string.split("_");
        words.map(|word| {
            let mut chars = word.chars();
            chars.next().unwrap().to_uppercase().chain(chars).collect()
        }).collect::<Vec<String>>().join(" ")
    }

    pub fn slugify(string: &str) -> String {
        let regex = regex::Regex::new(r"[^0-9a-zA-Z:-]+").unwrap();
        regex.replace_all(&string.to_lowercase(), "-").to_string()
    }

}
