/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: String = code.chars().filter(|&c| !c.is_whitespace()).collect();
    if code.len() <= 1 || code.chars().any(|c| !c.is_numeric()) {
        return false;
    }
    let sum:u32 = code.chars().map(|c| c.to_digit(10).unwrap())
        .rev()
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 0 {
                return d;
            }
            let p = d * 2;
            if p > 9 { p - 9 } else { p }
        })
        .sum();
    sum % 10 == 0
}
