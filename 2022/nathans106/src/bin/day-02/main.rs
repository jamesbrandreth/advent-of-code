use parser::parse;

use crate::game::score;

mod game;
mod hand;
mod parser;

fn main() {
    let strategy = parse(include_str!("input.txt"));
    let score = score(&strategy);
    println!("The score is {score}");
}
