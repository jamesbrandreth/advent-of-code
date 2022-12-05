use std::{collections::VecDeque, fmt};

use crate::instruction::Instruction;

#[derive(Debug, PartialEq, Eq)]
pub struct Supply {
    stacks: Vec<VecDeque<char>>,
}

impl Supply {
    pub fn new(stacks: Vec<VecDeque<char>>) -> Self {
        Supply { stacks }
    }

    pub fn apply(&mut self, instruction: &Instruction) {
        println!("Applying {instruction}");

        for _i in 0..instruction.number {
            let crate_ = self.stacks[instruction.from - 1].pop_back().unwrap();
            self.stacks[instruction.to - 1].push_back(crate_);
        }
    }

    pub fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.back().unwrap())
            .collect()
    }
}

impl fmt::Display for Supply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = "".to_string();
        for stack in self.stacks.iter() {
            for crate_ in stack.iter() {
                string += &crate_.to_string();
            }

            string += "||";
        }

        write!(f, "{string}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let expected = Supply::new(
            [
                VecDeque::from(['Z', 'N', 'D', 'C']),
                VecDeque::from(['M']),
                VecDeque::from(['P']),
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

        let instruction = Instruction {
            number: 2,
            from: 2,
            to: 1,
        };
        supply.apply(&instruction);

        assert_eq!(expected, supply);
    }

    #[test]
    fn test_tops() {
        let expected = "CMZ";

        let supply = Supply::new(
            [
                VecDeque::from(['C']),
                VecDeque::from(['M']),
                VecDeque::from(['P', 'D', 'N', 'Z']),
            ]
            .to_vec(),
        );

        assert_eq!(expected, supply.tops());
    }
}
