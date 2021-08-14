use phf::phf_map;

#[derive(Debug, PartialEq)]
pub struct Dna {
    body: String
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    body: String
}

static DNA_TO_RNA:phf::Map<char, char> = phf_map! {
    'G' => 'C',
    'C' => 'G',
    'T' => 'A',
    'A' => 'U'
};

fn validate(allowed:Vec<char>, chain: &str) -> Option<usize> {
    chain.chars()
        .enumerate()
        .filter(|(_i, nt)| !allowed.contains(nt))
        .next()
        .map(|(i, _)| i)
}

impl Dna {

    pub fn new(dna: &str) -> Result<Dna, usize> {
        match validate(DNA_TO_RNA.keys().cloned().collect(), dna) {
            None => Ok(Dna {
                body: dna.to_string()
            }),
            Some(i) => Err(i)
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            body: self.body.chars()
                .map(|c| DNA_TO_RNA.get(&c).unwrap())
                .collect()
        }
    }

}

impl Rna {

    pub fn new(rna: &str) -> Result<Rna, usize> {
        match validate(DNA_TO_RNA.values().cloned().collect(), rna) {
            None => Ok(Rna {
                body: rna.to_string()
            }),
            Some(i) => Err(i)
        }
    }

}
