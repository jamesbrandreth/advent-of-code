use crate::elves::Elves;

pub fn parse(text: &str) -> Elves {
    let mut data: Elves = vec![];
    let mut cur_elf: Vec<i32> = vec![];
    for line in text.lines() {
        if line.is_empty() {
            data.push(cur_elf.clone());
            cur_elf.clear();
        } else {
            cur_elf.push(line.parse::<i32>().unwrap());
        }
    }

    if !cur_elf.is_empty() {
        data.push(cur_elf);
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input_str = include_str!("test_input.txt");
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let result = parse(input_str);

        assert_eq!(expected, result);
    }
}
