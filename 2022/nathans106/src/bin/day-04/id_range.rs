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

    pub fn len(&self) -> i32 {
        self.end - self.start
    }

    pub fn contains(&self, id: &Id) -> bool {
        id >= &self.start && id <= &self.end
    }
}

pub fn overlap(rng1: &IdRange, rng2: &IdRange) -> bool {
    rng1.contains(&rng2.start) || rng1.contains(&rng2.end)
}

pub fn fully_overlap(rng1: &IdRange, rng2: &IdRange) -> bool {
    let (larger, smaller) = {
        if rng1.len() > rng2.len() {
            (rng1, rng2)
        } else {
            (rng2, rng1)
        }
    };

    larger.start <= smaller.start && larger.end >= smaller.end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_true() {
        assert!(overlap(&IdRange::new(5, 7), &IdRange::new(7, 9)));
    }

    #[test]
    fn test_overlap_false() {
        assert!(!overlap(&IdRange::new(2, 3), &IdRange::new(4, 5)));
    }

    #[test]
    fn test_fully_overlap_true() {
        assert!(fully_overlap(&IdRange::new(2, 8), &IdRange::new(3, 7)));
    }

    #[test]
    fn test_fully_overlap_false() {
        assert!(!fully_overlap(&IdRange::new(2, 4), &IdRange::new(6, 8)));
    }
}
