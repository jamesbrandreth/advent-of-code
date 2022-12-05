use parser::parse;

mod parser;

fn main() {
    let input = include_str!("input.txt");
    parse(input);
}
