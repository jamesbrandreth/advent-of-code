use crate::hand::Hand;

pub fn parse(input_str: &str) -> Vec<(Hand, Hand)> {
    return input_str
        .lines()
        .map(|line| {
            (
                match line.chars().nth(0).unwrap() {
                    'A' => Hand::Rock,
                    'B' => Hand::Paper,
                    'C' => Hand::Scissors,
                    _ => panic!(),
                },
                match line.chars().nth(2).unwrap() {
                    'X' => Hand::Rock,
                    'Y' => Hand::Paper,
                    'Z' => Hand::Scissors,
                    _ => panic!(),
                },
            )
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input_str = include_str!("test_input.txt");
        let expected: Vec<(Hand, Hand)> = vec![
            (Hand::Rock, Hand::Paper),
            (Hand::Paper, Hand::Rock),
            (Hand::Scissors, Hand::Scissors),
        ];
        assert_eq!(expected, parse(input_str));
    }
}
