use regex::{Match, Regex};

use crate::id_range::IdRange;

fn parse_match_pair(start_str: &Match, end_str: &Match) -> IdRange {
    let start: i32 = start_str.as_str().parse::<i32>().unwrap();
    let end: i32 = end_str.as_str().parse::<i32>().unwrap();
    IdRange::new(start, end)
}

pub fn parse(input: &str) -> Vec<(IdRange, IdRange)> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .array_chunks::<4>()
        .map(|nums| {
            (
                parse_match_pair(&nums[0], &nums[1]),
                parse_match_pair(&nums[2], &nums[3]),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("test_input.txt");
        let expected: Vec<(IdRange, IdRange)> = vec![
            (IdRange::new(2, 4), IdRange::new(6, 8)),
            (IdRange::new(2, 3), IdRange::new(4, 5)),
            (IdRange::new(5, 7), IdRange::new(7, 9)),
            (IdRange::new(2, 8), IdRange::new(3, 7)),
            (IdRange::new(6, 6), IdRange::new(4, 6)),
            (IdRange::new(2, 6), IdRange::new(4, 8)),
        ];

        assert_eq!(expected, parse(input));
    }
}
