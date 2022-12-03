mod elves;
mod highest_calories;
mod parser;

use parser::parse;

use crate::highest_calories::{highest_calories, highest_three_calories};

fn main() {
    let input_str = include_str!("input.txt");
    let elves = parse(input_str);

    let highest = highest_calories(&elves);
    println!("The highest calories are {highest}");

    let highest_three = highest_three_calories(&elves);
    println!("The highest three calories are {highest_three}");
}
