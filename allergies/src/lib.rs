use crate::Allergen::{Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats};

pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {

    pub fn new(score: u32) -> Self {
        let mut allergens:Vec<Allergen> = Vec::new();
        for allergen in &[Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats] {
            if score | *allergen as u32 == score {
                allergens.push(allergen.clone())
            }
        }
        Allergies(allergens)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.clone()
    }

}
