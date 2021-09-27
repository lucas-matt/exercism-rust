use std::collections::HashSet;

pub fn anagrams_for<'a>(orig: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = chars(orig);
    possible_anagrams
        .iter()
        .filter(|&&s| word == chars(s) && orig.to_lowercase() != s.to_lowercase())
        .map(|&s| s)
        .collect::<HashSet<&str>>()
}

fn chars(word:&str) -> Vec<char> {
    let mut chars = word.chars()
        .flat_map(|s| s.to_lowercase())
        .collect::<Vec<char>>();
    chars.sort();
    chars
}
