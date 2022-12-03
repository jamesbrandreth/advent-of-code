use std::cmp::Reverse;

use itertools::{max, Itertools};

use crate::elves::Elves;

pub fn highest_calories(elves: &Elves) -> i32 {
    let calories = elves.iter().map(|vals| vals.iter().sum::<i32>());
    max(calories).unwrap()
}

pub fn highest_three_calories(elves: &Elves) -> i32 {
    elves
        .iter()
        .map(|snacks| snacks.iter().sum::<i32>())
        .sorted_by_key(|c| Reverse(*c))
        .take(3)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_calories() {
        let data = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_eq!(highest_calories(&data), 24000);
    }

    #[test]
    fn test_highest_three_calories() {
        let data = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_eq!(highest_three_calories(&data), 45000);
    }
}
