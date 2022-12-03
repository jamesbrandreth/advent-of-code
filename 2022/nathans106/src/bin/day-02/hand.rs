#[derive(Debug, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
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

pub fn hand(other: &Hand, outcome: &Outcome) -> Hand {
    match other {
        Hand::Rock => match outcome {
            Outcome::Win => Hand::Paper,
            Outcome::Lose => Hand::Scissors,
            Outcome::Draw => Hand::Rock,
        },
        Hand::Paper => match outcome {
            Outcome::Win => Hand::Scissors,
            Outcome::Lose => Hand::Rock,
            Outcome::Draw => Hand::Paper,
        },
        Hand::Scissors => match outcome {
            Outcome::Win => Hand::Rock,
            Outcome::Lose => Hand::Paper,
            Outcome::Draw => Hand::Scissors,
        },
    }
}
