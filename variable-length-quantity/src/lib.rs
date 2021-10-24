#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter()
        .flat_map(|n| {
            let mut modifier = 0;
            let mut n = *n;
            let mut bytes:Vec<u8> = Vec::new();
            while n > 0x7f {
                bytes.push((n & 0x7f) as u8 + modifier);
                n = n >> 7;
                modifier = 0x80;
            }
            bytes.push(n as u8 + modifier);
            bytes.into_iter()
                .rev()
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    partition(bytes)?
        .iter()
        .map(|parts| {
            parts.into_iter()
                .fold(Ok(0_u32), |acc, n| {
                    let base = acc?;
                    if base & 0xfe_00_00_00 > 0 {
                        return Err(Error::Overflow);
                    }
                    Ok((base << 7) + (n & 0x7f) as u32)
                })
        })
        .collect::<Result<Vec<u32>, Error>>()
}

fn partition(bytes: &[u8]) -> Result<Vec<Vec<u8>>, Error> {
    let mut partitioned = Vec::new();
    let mut batch = Vec::new();
    for byte in bytes {
        batch.push(*byte);
        if 0x80 & byte == 0 {
            partitioned.push(batch);
            batch = Vec::new();
        }
    }
    if partitioned.is_empty() {
        return Err(Error::IncompleteNumber);
    }
    Ok(partitioned)
}
