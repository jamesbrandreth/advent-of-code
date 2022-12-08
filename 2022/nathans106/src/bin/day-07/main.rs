#![feature(iter_advance_by)]

use parser::parse;

mod parser;
mod storage;

fn main() {
    let input = include_str!("input.txt");
    let _root = parse(input);
}

#[cfg(test)]
mod tests {
    use super::*;
}
