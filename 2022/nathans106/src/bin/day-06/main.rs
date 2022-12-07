use itertools::Itertools;

fn marker_position(data: &str) -> usize {
    let chars = data.chars().collect_vec();
    let window_idx = chars
        .windows(4)
        .enumerate()
        .find(|(_idx, window)| window.iter().all_unique())
        .unwrap()
        .0;
    window_idx + 4
}

fn main() {
    let input = include_str!("input.txt");
    let marker = marker_position(input);
    println!("Start-of-packet marker is at {marker}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_position() {
        let inputs = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let expected: Vec<usize> = vec![7, 5, 6, 10, 11];
        let result: Vec<usize> = inputs.iter().map(|s| marker_position(s)).collect();
        assert_eq!(expected, result);
    }
}
