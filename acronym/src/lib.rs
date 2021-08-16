use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    let nonalpha = Regex::new("[^a-zA-Z']+").unwrap();
    let allcaps = Regex::new("^[A-Z]+$").unwrap();
    nonalpha.split(phrase)
        .map(|word| word.chars().filter(|c| c.is_alphabetic()).collect())
        .filter(|word| word != "")
        .map(|word: String| format!("{}{}", (&word[..1].to_string()).to_ascii_uppercase(), &word[1..]))
        .map(|word: String| match word {
            w if allcaps.is_match(w.as_str()) => w[..1].to_ascii_uppercase(),
            _ => word.chars().filter(|c| c.is_uppercase()).collect()
        })
        .collect()
}
