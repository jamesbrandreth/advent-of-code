use std::{collections::VecDeque, fmt};

use crate::instruction::Instruction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Supply {
    stacks: Vec<VecDeque<char>>,
}

impl Supply {
    pub fn new(stacks: Vec<VecDeque<char>>) -> Self {
        Supply { stacks }
    }

    pub fn apply_lifo(&mut self, instruction: &Instruction) {
        for _i in 0..instruction.number {
            let crate_ = self.stacks[instruction.from - 1].pop_back().unwrap();
            self.stacks[instruction.to - 1].push_back(crate_);
        }
    }

    pub fn apply_batch(&mut self, instruction: &Instruction) {
        let split_idx = self.stacks[instruction.from - 1].len() - instruction.number;
        let mut in_transit = self.stacks[instruction.from - 1].split_off(split_idx);
        self.stacks[instruction.to - 1].append(&mut in_transit);
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
    fn test_applyLifo() {
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
        supply.apply_lifo(&instruction);

        assert_eq!(expected, supply);
    }

    #[test]
    fn test_applyBatch() {
        let expected = Supply::new(
            [
                VecDeque::from(['Z', 'N', 'C', 'D']),
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
        supply.apply_batch(&instruction);

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
