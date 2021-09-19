pub fn check(candidate: &str) -> bool {
    let mut chars: Vec<char> = candidate.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let size = chars.len();
    chars.sort();
    chars.dedup();
    size == chars.len()
}
