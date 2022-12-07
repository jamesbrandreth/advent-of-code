use itertools::Itertools;

fn marker_position(data: &str, marker_size: usize) -> usize {
    let chars = data.chars().collect_vec();
    let window_idx = chars
        .windows(marker_size)
        .enumerate()
        .find(|(_idx, window)| window.iter().all_unique())
        .unwrap()
        .0;
    window_idx + marker_size
}

fn main() {
    let input = include_str!("input.txt");
    let four_marker = marker_position(input, 4);
    println!("Start-of-packet 4 marker is at {four_marker}");
    let fourteen_marker = marker_position(input, 14);
    println!("Start-of-packet 14 marker is at {fourteen_marker}");
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
        let result: Vec<usize> = inputs.iter().map(|s| marker_position(s, 4)).collect();
        assert_eq!(expected, result);
    }
}
