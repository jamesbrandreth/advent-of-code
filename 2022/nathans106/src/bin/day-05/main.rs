#![feature(iter_array_chunks)]

use instruction::Instruction;
use parser::parse;
use supply::Supply;

mod instruction;
mod parser;
mod supply;

fn apply_lifo(supply: &mut Supply, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        supply.apply_lifo(instruction);
    }
}

fn apply_batch(supply: &mut Supply, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        supply.apply_batch(instruction);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (mut supply1, instructions) = parse(input);
    let mut supply2 = supply1.clone();
    apply_lifo(&mut supply1, &instructions);
    let lifo_tops = supply1.tops();
    println!("Lifo tops are {lifo_tops}");

    apply_batch(&mut supply2, &instructions);
    let batch_tops = supply2.tops();
    println!("Lifo tops are {batch_tops}");
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

        apply_lifo(&mut supply, &instructions);

        assert_eq!(expected, supply);
    }
}
