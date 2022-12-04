use crate::hand::{Hand, Outcome};

pub fn parse_moves(input_str: &str) -> Vec<(Hand, Hand)> {
    return input_str
        .lines()
        .map(|line| {
            (
                match line.chars().next().unwrap() {
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

pub fn parse_outcomes(input_str: &str) -> Vec<(Hand, Outcome)> {
    return input_str
        .lines()
        .map(|line| {
            (
                match line.chars().next().unwrap() {
                    'A' => Hand::Rock,
                    'B' => Hand::Paper,
                    'C' => Hand::Scissors,
                    _ => panic!(),
                },
                match line.chars().nth(2).unwrap() {
                    'X' => Outcome::Lose,
                    'Y' => Outcome::Draw,
                    'Z' => Outcome::Win,
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
    fn test_parse_moves() {
        let input_str = include_str!("test_input.txt");
        let expected: Vec<(Hand, Hand)> = vec![
            (Hand::Rock, Hand::Paper),
            (Hand::Paper, Hand::Rock),
            (Hand::Scissors, Hand::Scissors),
        ];
        assert_eq!(expected, parse_moves(input_str));
    }

    #[test]
    fn test_parse_outcomes() {
        let input_str = include_str!("test_input.txt");
        let expected: Vec<(Hand, Outcome)> = vec![
            (Hand::Rock, Outcome::Draw),
            (Hand::Paper, Outcome::Lose),
            (Hand::Scissors, Outcome::Win),
        ];
        assert_eq!(expected, parse_outcomes(input_str));
    }
}
