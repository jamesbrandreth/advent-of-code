pub type Id = i32;

#[derive(Debug, PartialEq, Eq)]
pub struct IdRange {
    pub start: Id,
    pub end: Id,
}

impl IdRange {
    pub fn new(start: Id, end: Id) -> Self {
        Self { start, end }
    }
}
