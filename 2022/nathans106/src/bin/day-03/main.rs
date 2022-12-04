use parser::parse;

mod parser;
mod rucksack;

fn main() {
    let input = include_str!("input.txt");
    let _rucksacks = parse(input);
}
