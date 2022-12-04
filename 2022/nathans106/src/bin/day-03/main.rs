#![feature(step_trait)]

use item::Priority;
use parser::parse;

mod item;
mod parser;
mod rucksack;

fn main() {
    let input = include_str!("input.txt");
    let rucksacks = parse(input);
    let priorities_sum: i32 = rucksacks
        .iter()
        .map(|rucksack| rucksack.common_item().priority())
        .sum();
    println!("Priorities sum is {priorities_sum}");
}
