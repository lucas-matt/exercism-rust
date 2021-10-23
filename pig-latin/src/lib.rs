use std::net::ToSocketAddrs;

pub fn translate(input: &str) -> String {
    input.split_ascii_whitespace()
        .into_iter()
        .map(|word| word.to_string())
        .map(|word| {
            let (start, end) = split(word);
            format!("{}{}ay", start, end)
        })
        .collect::<Vec<String>>()
        .join(" ")  
}

fn split(word:String) -> (String, String) {
    let mut word = word.to_string();
    let start = String::new();
    let mut end = String::new();
    while !word.is_empty() {
        if begins(&["a", "e", "i", "o", "u", "xr", "yt"], &word) {
            return (word, end);
        }
        if begins(&["qu"], &word) {
            return (word[2..].to_string(), format!("{}{}", end, word[..2].to_string()));
        }
        if begins(&["y"], &word) {
            
        }
        end = format!("{}{}", end, word.remove(0));
    }
    (start, end)
}

fn begins(terms:&[&str], word:&String) -> bool {
    terms
        .iter()
        .any(|&start| word.starts_with(start))
}
