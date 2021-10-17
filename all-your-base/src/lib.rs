#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    check_base(from_base, Error::InvalidInputBase)?;
    check_base(to_base, Error::InvalidOutputBase)?;
    check_digits(from_base, number)?;

    let mut base_10:u32 = number.into_iter()
        .rev()
        .zip(0_u32..number.len() as u32)
        .map(|(x, pow)| x * from_base.pow(pow))
        .sum();

    let mut result:Vec<u32> = Vec::new();
    while base_10 > 0 {
        result.insert(0, base_10 % to_base);
        base_10 = base_10 / to_base;
    }

    if result.is_empty() {
        result.push(0);
    }

    Ok(result)
}

fn check_base(base:u32, err:Error) -> Result<(), Error> {
    if base < 2 {
        return Err(err);
    }
    Ok(())
}

fn check_digits(base:u32, number: &[u32]) -> Result<(), Error> {
    for digit in number {
        if digit >= &base {
            return Err(Error::InvalidDigit(*digit))
        }
    }
    Ok(())
}

