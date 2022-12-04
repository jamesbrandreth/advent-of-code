use crate::rucksack::Rucksack;

pub fn parse(input_str: &str) -> Vec<Rucksack> {
    input_str
        .lines()
        .map(|line| {
            let section_size = line.len() / 2;
            let (section1_str, section2_str) = line.split_at(section_size);
            Rucksack::new(section1_str.to_string(), section2_str.to_string())
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
            Rucksack::new("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string()),
            Rucksack::new(
                "jqHRNqRjqzjGDLGL".to_string(),
                "rsFMfFZSrLrFZsSL".to_string(),
            ),
            Rucksack::new("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string()),
            Rucksack::new("wMqvLMZHhHMvwLH".to_string(), "jbvcjnnSBnvTQFn".to_string()),
            Rucksack::new("ttgJtRGJ".to_string(), "QctTZtZT".to_string()),
            Rucksack::new("CrZsJsPPZsGz".to_string(), "wwsLwLmpwMDw".to_string()),
        ];

        assert_eq!(expected, parse(input_str));
    }
}
