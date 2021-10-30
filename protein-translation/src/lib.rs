use std::{collections::HashMap, mem::take};

pub struct CodonsInfo<'a> {
    proteins: HashMap<&'a str, &'a str>
}

impl <'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &'a str) -> Option<&'a str> {
        self.proteins.get(codon).map(|&s| s)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {

        let mut proteins = rna.chars()
            .collect::<Vec<char>>()
            .windows(3)
            .step_by(3)
            .map(|group| {
                let code:String = group.iter().collect();
                self.proteins.get(code.as_str()).map(|s| *s)
            })
            .collect::<Vec<Option<&'a str>>>()
            .into_iter();

        let mut complete = Vec::new();
        while let Some(Some(protein)) = proteins.next() {
            if protein == "stop codon" {
                return Some(complete);
            }
            complete.push(protein);
        }

        if complete.is_empty() || rna.len() % 3 != 0{
            return None;
        }
        
        Some(complete)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { 
        proteins: pairs.into_iter().collect()
    }
}
