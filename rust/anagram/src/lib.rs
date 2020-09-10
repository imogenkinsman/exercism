use std::collections::{HashSet, HashMap};
#[derive(PartialEq)]
struct Anagram {
    chars: HashMap<char, u16>,
}

impl Anagram {
    pub fn new(str: &str) -> Self {
        let mut chars: HashMap<char, u16> = HashMap::new();

        for c in str.chars() {
            let char = chars.entry(c).or_insert(0);
            *char += 1;
        }

        Anagram {chars: chars}
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    let word_anagram = Anagram::new(word);

    for possible in possible_anagrams {
        if possible.len() != word.len() {
            continue;
        }

        let possible_anagram = Anagram::new(possible);
        if possible_anagram == word_anagram {
            result.insert(possible);
        }
    }

    result
}
