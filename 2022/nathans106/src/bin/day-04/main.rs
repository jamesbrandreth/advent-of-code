#![feature(iter_array_chunks)]

use id_range::{fully_overlap, overlap, IdRange};
use parser::parse;

mod id_range;
mod parser;

fn main() {
    let input = include_str!("input.txt");
    let range_pairs = parse(input);
    let fully_overlap_count = num_fully_overlap(&range_pairs);
    println!("{fully_overlap_count} pairs fully overlap");

    let overlap_count = num_overlap(&range_pairs);
    println!("{overlap_count} pairs overlap");
}

fn num_overlap(ranges: &[(IdRange, IdRange)]) -> usize {
    ranges
        .iter()
        .filter(|(rng1, rng2)| overlap(rng1, rng2))
        .count()
}

fn num_fully_overlap(ranges: &[(IdRange, IdRange)]) -> usize {
    ranges
        .iter()
        .filter(|(rng1, rng2)| fully_overlap(rng1, rng2))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_overlap() {
        let data: Vec<(IdRange, IdRange)> = vec![
            (IdRange::new(2, 4), IdRange::new(6, 8)),
            (IdRange::new(2, 3), IdRange::new(4, 5)),
            (IdRange::new(5, 7), IdRange::new(7, 9)),
            (IdRange::new(2, 8), IdRange::new(3, 7)),
            (IdRange::new(6, 6), IdRange::new(4, 6)),
            (IdRange::new(2, 6), IdRange::new(4, 8)),
        ];

        assert_eq!(4, num_overlap(&data));
    }

    #[test]
    fn test_num_fully_overlap() {
        let data: Vec<(IdRange, IdRange)> = vec![
            (IdRange::new(2, 4), IdRange::new(6, 8)),
            (IdRange::new(2, 3), IdRange::new(4, 5)),
            (IdRange::new(5, 7), IdRange::new(7, 9)),
            (IdRange::new(2, 8), IdRange::new(3, 7)),
            (IdRange::new(6, 6), IdRange::new(4, 6)),
            (IdRange::new(2, 6), IdRange::new(4, 8)),
        ];

        assert_eq!(2, num_fully_overlap(&data));
    }
}
