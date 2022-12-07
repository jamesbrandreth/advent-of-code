use parser::parse;

mod parser;

fn main() {
    let input = include_str!("input.txt");
    let x = parse(input);
}

#[cfg(test)]
mod test {
    use super::*;

}
