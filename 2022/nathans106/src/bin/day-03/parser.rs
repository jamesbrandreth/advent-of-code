use crate::{group::Group, rucksack::Rucksack};

pub fn parse_rucksacks(input_str: &str) -> Vec<Rucksack> {
    input_str
        .lines()
        .map(|line| Rucksack::new(line.chars().collect()))
        .collect()
}

pub fn parse_groups(input_str: &str) -> Vec<Group> {
    input_str
        .lines()
        .map(|line| Rucksack::new(line.chars().collect()))
        .array_chunks::<3>()
        .map(Group::new)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rucksacks() {
        let input_str = include_str!("test_input.txt");
        let expected: Vec<Rucksack> = vec![
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect()),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect()),
            Rucksack::new("PmmdzqPrVvPwwTWBwg".chars().collect()),
            Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".chars().collect()),
            Rucksack::new("ttgJtRGJQctTZtZT".chars().collect()),
            Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect()),
        ];

        assert_eq!(expected, parse_rucksacks(input_str));
    }

    #[test]
    fn test_parse_groups() {
        let input_str = include_str!("test_input.txt");
        let expected: Vec<Group> = vec![
            Group::new([
                Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect()),
                Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect()),
                Rucksack::new("PmmdzqPrVvPwwTWBwg".chars().collect()),
            ]),
            Group::new([
                Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".chars().collect()),
                Rucksack::new("ttgJtRGJQctTZtZT".chars().collect()),
                Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect()),
            ]),
        ];

        assert_eq!(expected, parse_groups(input_str));
    }
}
