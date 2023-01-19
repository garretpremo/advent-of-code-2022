use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Rucksack {
    compartment_one: HashSet<char>,
    compartment_two: HashSet<char>
}

impl Rucksack {
    pub fn from(compartment_one: &str, compartment_two: &str) -> Rucksack {
        Rucksack {
            compartment_one: compartment_one.chars().collect(),
            compartment_two: compartment_two.chars().collect()
        }
    }

    pub fn get_common_item(&self) -> char {
        let common_items: Vec<&char> = self.compartment_one.intersection(&self.compartment_two).collect();
        assert_eq!(common_items.len(), 1);
        *common_items[0]
    }

    pub fn get_all_items(&self) -> HashSet<char> {
        self.compartment_one.union(&self.compartment_two).copied().collect()
    }
}
