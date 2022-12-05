use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub number: usize,
    pub from: usize,
    pub to: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.number, self.from, self.to)
    }
}
