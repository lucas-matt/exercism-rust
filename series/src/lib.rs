use std::str::from_utf8;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![String::new(); digits.len()+1]
    }
    digits.as_bytes()
        .windows(len)
        .map(|series| from_utf8(series).unwrap().to_string())
        .collect()
}
