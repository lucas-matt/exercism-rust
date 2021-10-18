#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if let Some(char) = string_digits.chars().find(|c| !c.is_numeric()) {
        return Err(Error::InvalidDigit(char));
    }
    if span == 0 {
        return Ok(1);
    }
    let max: u32 = string_digits.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .windows(span)
        .map(|series| series.into_iter().product())
        .max()
        .unwrap();
    Ok(max as u64)
}
