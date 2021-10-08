use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let words:String = words.chars().map(|c| match c {
        c if c.is_ascii_alphanumeric() => c.to_ascii_lowercase(),
        '\'' => c,
        _ => ' '
    }).collect();
    words.split_ascii_whitespace()
        .fold(HashMap::new(), |mut acc, w| {
            let w = w.trim_start_matches("'").trim_end_matches("'");
            *acc.entry(w.to_string()).or_insert(0) += 1;
            acc
        })
}
