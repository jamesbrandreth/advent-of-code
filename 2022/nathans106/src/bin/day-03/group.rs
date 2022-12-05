use crate::{item::Priority, rucksack::Rucksack};

#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    elves: [Rucksack; 3],
}

impl Group {
    pub fn new(elves: [Rucksack; 3]) -> Self {
        Self { elves }
    }

    pub fn badge_priority(&self) -> i32 {
        let mut item_count = [0; 52];

        for elf in self.elves.iter() {
            for item in elf.item_types().iter() {
                item_count[(item.priority() - 1) as usize] += 1;
            }
        }

        (item_count.iter().position(|c| c == &3).unwrap() + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_priority() {
        let group = Group::new([
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect()),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect()),
            Rucksack::new("PmmdzqPrVvPwwTWBwg".chars().collect()),
        ]);

        assert_eq!(18, group.badge_priority());
    }
}
