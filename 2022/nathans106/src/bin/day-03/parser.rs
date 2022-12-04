use crate::rucksack::Rucksack;

pub fn parse(input_str: &str) -> Vec<Rucksack> {
    input_str
        .lines()
        .map(|line| {
            let section_size = line.len() / 2;
            let (section1_str, section2_str) = line.split_at(section_size);
            Rucksack::new(
                section1_str.chars().collect(),
                section2_str.chars().collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input_str = include_str!("test_input.txt");
        let expected: Vec<Rucksack> = vec![
            Rucksack::new(
                "vJrwpWtwJgWr".chars().collect(),
                "hcsFMMfFFhFp".chars().collect(),
            ),
            Rucksack::new(
                "jqHRNqRjqzjGDLGL".chars().collect(),
                "rsFMfFZSrLrFZsSL".chars().collect(),
            ),
            Rucksack::new("PmmdzqPrV".chars().collect(), "vPwwTWBwg".chars().collect()),
            Rucksack::new(
                "wMqvLMZHhHMvwLH".chars().collect(),
                "jbvcjnnSBnvTQFn".chars().collect(),
            ),
            Rucksack::new("ttgJtRGJ".chars().collect(), "QctTZtZT".chars().collect()),
            Rucksack::new(
                "CrZsJsPPZsGz".chars().collect(),
                "wwsLwLmpwMDw".chars().collect(),
            ),
        ];

        assert_eq!(expected, parse(input_str));
    }
}
