use core::panic;

use itertools::Itertools;

use crate::item::Item;

#[derive(PartialEq, Eq, Debug)]
pub struct Rucksack {
    pub items: Vec<Item>,
}

impl Rucksack {
    pub fn new(items: Vec<Item>) -> Self {
        Rucksack { items }
    }

    pub fn sections(&self) -> (&[Item], &[Item]) {
        let section_size = self.items.len() / 2;
        self.items.split_at(section_size)
    }

    pub fn common_item(&self) -> &Item {
        let (section1, section2) = self.sections();
        for s1_item in section1.iter() {
            for s2_item in section2.iter() {
                if s1_item == s2_item {
                    return s1_item;
                }
            }
        }

        panic!("No common item");
    }

    pub fn item_types(&self) -> Vec<Item> {
        self.items.iter().unique().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_item() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect());

        assert_eq!(&'p', rucksack.common_item());
    }
}
