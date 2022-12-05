#![feature(iter_array_chunks)]

use instruction::Instruction;
use parser::parse;
use supply::Supply;

mod instruction;
mod parser;
mod supply;

fn apply_instructions(supply: &mut Supply, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        supply.apply(instruction);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (mut supply, instructions) = parse(input);
    apply_instructions(&mut supply, &instructions);
    let tops = supply.tops();
    println!("Tops are {tops}");
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn test_apply_instructions() {
        let expected = Supply::new(
            [
                VecDeque::from(['C']),
                VecDeque::from(['M']),
                VecDeque::from(['P', 'D', 'N', 'Z']),
            ]
            .to_vec(),
        );
        let mut supply = Supply::new(
            [
                VecDeque::from(['Z', 'N']),
                VecDeque::from(['M', 'C', 'D']),
                VecDeque::from(['P']),
            ]
            .to_vec(),
        );

        let instructions = vec![
            Instruction {
                number: 1,
                from: 2,
                to: 1,
            },
            Instruction {
                number: 3,
                from: 1,
                to: 3,
            },
            Instruction {
                number: 2,
                from: 2,
                to: 1,
            },
            Instruction {
                number: 1,
                from: 1,
                to: 2,
            },
        ];

        apply_instructions(&mut supply, &instructions);

        assert_eq!(expected, supply);
    }
}
