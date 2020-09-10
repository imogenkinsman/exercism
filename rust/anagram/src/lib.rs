use std::collections::{HashSet, HashMap};
#[derive(PartialEq)]
struct AnagramMap {
    chars: HashMap<char, u16>,
}

impl AnagramMap {
    pub fn new(str: String) -> Self {
        let mut chars: HashMap<char, u16> = HashMap::new();

        for c in str.chars() {
            let entry = chars.entry(c).or_insert(0);
            *entry += 1;
        }

        AnagramMap {chars: chars}
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    let word_map = AnagramMap::new(word.to_lowercase());

    for possible in possible_anagrams {
        if possible.len() != word.len() || *possible.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let possible_map = AnagramMap::new(possible.to_lowercase());
        if possible_map == word_map {
            result.insert(possible);
        }
    }

    result
}
