use std::collections::HashMap;

const NUCLEOTIDES: &[char; 4] = &['A', 'C', 'G', 'T'];

fn validate(nucleotide: char, dna: &str) -> Result<&str, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    let invalid = dna.chars().filter(|&n| !NUCLEOTIDES.contains(&n)).next();
    match invalid {
        Some(c) => Err(c),
        _ => Ok(dna)
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let count = validate(nucleotide, dna)?.chars().filter(|&n| n == nucleotide).count();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    for &nucleotide in NUCLEOTIDES {
        map.insert(nucleotide, count(nucleotide, dna)?);
    }
    Ok(map)
}
