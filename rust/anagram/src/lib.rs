use std::collections::{HashSet, HashMap};
#[derive(PartialEq)]
struct Anagram {
    chars: HashMap<String, u16>,
}

impl Anagram {
    pub fn new(str: &str) -> Self {
        let mut chars: HashMap<String, u16> = HashMap::new();

        for c in str.chars() {
            let lowercase : String = c.to_lowercase().collect();

            let entry = chars.entry(lowercase).or_insert(0);
            *entry += 1;
        }

        Anagram {chars: chars}
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    let word_anagram = Anagram::new(word);

    for possible in possible_anagrams {
        if possible.len() != word.len() || *possible.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let possible_anagram = Anagram::new(possible);
        if possible_anagram == word_anagram {
            result.insert(possible);
        }
    }

    result
}
