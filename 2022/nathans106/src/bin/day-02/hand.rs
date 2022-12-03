#[derive(Debug, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

pub fn outcome(other: &Hand, you: &Hand) -> Outcome {
    match other {
        Hand::Rock => match you {
            Hand::Rock => Outcome::Draw,
            Hand::Paper => Outcome::Win,
            Hand::Scissors => Outcome::Lose,
        },
        Hand::Paper => match you {
            Hand::Rock => Outcome::Lose,
            Hand::Paper => Outcome::Draw,
            Hand::Scissors => Outcome::Win,
        },
        Hand::Scissors => match you {
            Hand::Rock => Outcome::Win,
            Hand::Paper => Outcome::Lose,
            Hand::Scissors => Outcome::Draw,
        },
    }
}
