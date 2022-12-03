use crate::hand::{outcome, Hand, Outcome};

pub type Game = Vec<(Hand, Hand)>;

pub fn score(game: &Game) -> i32 {
    game.iter().map(|(other, you)|
        match outcome(other, you) {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        } + match you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    ).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let game: Vec<(Hand, Hand)> = vec![
            (Hand::Rock, Hand::Paper),
            (Hand::Paper, Hand::Rock),
            (Hand::Scissors, Hand::Scissors),
        ];

        assert_eq!(score(&game), 15);
    }
}
