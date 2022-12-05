use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

use crate::{instruction::Instruction, supply::Supply};

fn parse_supply(input: &str) -> Supply {
    let num_stacks = (input.lines().next().unwrap().len() + 1) / 4;
    let stacks_re = Regex::new(r"(   |\[\w\]) ?").unwrap();
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); num_stacks];
    for crate_matches in stacks_re.find_iter(input).chunks(num_stacks).into_iter() {
        for (stack_idx, crate_match) in crate_matches.into_iter().enumerate() {
            let crate_str = crate_match.as_str();
            if crate_str.starts_with('[') {
                let crate_ = crate_str.chars().nth(1).unwrap();
                stacks[stack_idx].push_front(crate_);
            }
        }
    }

    Supply::new(stacks)
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let stacks_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    stacks_re
        .captures_iter(input)
        .map(|m| {
            let number = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = m.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to = m.get(3).unwrap().as_str().parse::<usize>().unwrap();
            Instruction { number, from, to }
        })
        .collect()
}

pub fn parse(input: &str) -> (Supply, Vec<Instruction>) {
    (parse_supply(input), parse_instructions(input))
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[ignore]
    #[test]
    fn test_parse_supply() {
        let expected = Supply::new(
            [
                VecDeque::from(['Z', 'N']),
                VecDeque::from(['M', 'C', 'D']),
                VecDeque::from(['P']),
            ]
            .to_vec(),
        );

        assert_eq!(expected, parse_supply(TEST_INPUT));
    }

    #[test]
    fn test_parse_instructions() {
        let expected = vec![
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

        assert_eq!(expected, parse_instructions(TEST_INPUT));
    }
}
