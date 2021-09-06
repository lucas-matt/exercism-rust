/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // much simpler if we could use the regex crate
    let isbn:String = isbn.chars()
        .filter(|c| c.is_digit(10) || *c == 'X')
        .collect();
    if isbn.len() != 10 || *&isbn[0..9].chars().any(|c| !c.is_digit(10)) {
        return false;
    }
    let sum:u32 = isbn.chars()
        .map(|c| match c {
            'X' => 10,
            _ => c.to_digit(10).unwrap()
        })
        .zip((1..=10).rev())
        .map(|(x, y)| x * y)
        .sum();
    sum % 11 == 0
}
