use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1(filepath: &str) -> Result<u32, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut values: Vec<u32> = vec![];
    for line in reader.lines() {
        let mut numbers: Vec<char> = vec![];
        for char in line?.chars() {
            match char.is_numeric() {
                true => numbers.push(char),
                false => {}
            };
        }
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let value: u32 = String::from_iter(vec![first, last]).parse().unwrap();
        values.push(value);
    }
    let value: u32 = values.iter().sum();
    Ok(value)
}

fn to_digit(s: &str) -> Option<u32> {
    let values: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let d = s.chars().next().unwrap();
    match d.is_numeric() {
        true => {
            return Some(d.to_digit(10).unwrap());
        }
        false => match values.get(s) {
            Some(value) => return Some(value.clone()),
            None => return None,
        },
    }
}

fn part_2(filepath: &str) -> Result<u32, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let pattern_forward = r"[0-9]|one|two|three|four|five|six|seven|eight|nine";
    let pattern_backward = r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin";

    let re_forward = Regex::new(pattern_forward).unwrap();
    let re_backward = Regex::new(pattern_backward).unwrap();

    let mut values: Vec<u32> = vec![];
    for line in reader.lines() {
        let mut l = line.unwrap();
        println!("{}", &l);
        let first = to_digit(re_forward.find(&l).unwrap().as_str()).unwrap();
        l = l.chars().rev().collect();
        println!("{}", &l);
        let rev_second: String = re_backward
            .find(&l)
            .unwrap()
            .as_str()
            .chars()
            .rev()
            .collect();
        let last = to_digit(&rev_second).unwrap();
        let val = first * 10 + last;
        println!("{} {} => {}", first, last, val);
        values.push(val);
    }
    let value: u32 = values.iter().sum();
    Ok(value)
}

fn main() {
    match part_2("./input.txt") {
        Ok(value) => {
            println!("{}", value);
        }
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
}
