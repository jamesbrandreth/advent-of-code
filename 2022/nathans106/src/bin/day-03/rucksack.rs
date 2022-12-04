use core::panic;

use crate::item::Item;

#[derive(PartialEq, Eq, Debug)]
pub struct Rucksack {
    section1: Vec<Item>,
    section2: Vec<Item>,
}

impl Rucksack {
    pub fn new(section1: Vec<Item>, section2: Vec<Item>) -> Self {
        Rucksack { section1, section2 }
    }

    pub fn common_item(&self) -> &Item {
        for s1_item in self.section1.iter() {
            for s2_item in self.section2.iter() {
                if s1_item == s2_item {
                    return s1_item;
                }
            }
        }

        panic!("No common item");
    }
}
