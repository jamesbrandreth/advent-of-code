#![feature(iter_array_chunks)]

use parser::parse;

mod id_range;
mod parser;

fn main() {
    let input = include_str!("input.txt");
    parse(input);
}
