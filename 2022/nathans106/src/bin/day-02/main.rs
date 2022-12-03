use parser::parse_moves;

use crate::{
    game::{score_from_hands, score_from_outcomes},
    parser::parse_outcomes,
};

mod game;
mod hand;
mod parser;

fn main() {
    let input = include_str!("input.txt");

    let hands_score = score_from_hands(&parse_moves(input));
    println!("If column 2 is hands the score is {hands_score}");

    let outcomes_score = score_from_outcomes(&parse_outcomes(input));
    println!("If column 2 is outcomes the score is {outcomes_score}");
}
