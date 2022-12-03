use crate::hand::{hand, outcome, Hand, Outcome};

pub type Game = Vec<(Hand, Hand)>;
pub fn score_from_hands(game: &Game) -> i32 {
    game.iter()
        .map(|(other, you)| outcome_score(&outcome(other, you)) + hand_score(you))
        .sum::<i32>()
}

pub fn score_from_outcomes(outcomes: &[(Hand, Outcome)]) -> i32 {
    outcomes
        .iter()
        .map(|(other, outcome)| outcome_score(outcome) + hand_score(&hand(other, outcome)))
        .sum::<i32>()
}

fn hand_score(hand: &Hand) -> i32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn outcome_score(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_from_hands() {
        let game: Vec<(Hand, Hand)> = vec![
            (Hand::Rock, Hand::Paper),
            (Hand::Paper, Hand::Rock),
            (Hand::Scissors, Hand::Scissors),
        ];

        assert_eq!(score_from_hands(&game), 15);
    }

    #[test]
    fn test_score_from_outcomes() {
        let game: Vec<(Hand, Outcome)> = vec![
            (Hand::Rock, Outcome::Draw),
            (Hand::Paper, Outcome::Lose),
            (Hand::Scissors, Outcome::Win),
        ];

        assert_eq!(score_from_outcomes(&game), 12);
    }
}
