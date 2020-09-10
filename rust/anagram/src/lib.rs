use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    for w in possible_anagrams {
        if w.len() == word.len() {
            result.insert(w);
        }
    }

    result
}
