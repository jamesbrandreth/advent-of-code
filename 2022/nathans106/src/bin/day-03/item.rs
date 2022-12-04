use std::iter::Step;

pub type Item = char;

pub trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for Item {
    fn priority(&self) -> i32 {
        if self.is_lowercase() {
            return (char::steps_between(&'a', self).unwrap() + 1) as i32;
        }

        (char::steps_between(&'A', self).unwrap() + 27) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_priority() {
        assert_eq!(1, 'a'.priority());
        assert_eq!(26, 'z'.priority());
    }

    #[test]
    fn test_uppercase_priority() {
        assert_eq!(27, 'A'.priority());
        assert_eq!(52, 'Z'.priority());
    }
}
