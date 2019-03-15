use self::Allergen::*;

pub struct Allergies {
    allergens : Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let binary_rep = format!("{:07b}", score);
        let allergens = binary_rep.chars().rev().enumerate()
            .fold(vec![], |mut acc, (i, v)| {
                if v == '1' && i <= 7 {
                    let allergen = match i {
                        0 => Eggs,
                        1 => Peanuts,
                        2 => Shellfish,
                        3 => Strawberries,
                        4 => Tomatoes,
                        5 => Chocolate,
                        6 => Pollen,
                        7 => Cats,
                        _ => Cats,
                    };
                    acc.push(allergen);
                }
                return acc; 
            });
        return Allergies {
            allergens
        };
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
