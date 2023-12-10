use regex::Regex;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1(grid: Vec<String>) -> u32 {
    let re_number = Regex::new(r"\d+").unwrap();
    let re_symbol = Regex::new(r"[^.\d]").unwrap();
    let candidates: Vec<_> = grid
        .iter()
        .map(|line| re_number.find_iter(line).collect::<Vec<_>>())
        .collect();
    // Search around each candidate
    let height: usize = grid.len();
    let width: usize = grid.first().unwrap().len();
    let mut numbers: Vec<u32> = vec![];
    for i in 0..height {
        let p = max(1, i) - 1;
        let q = min(height - 2, i) + 2;
        for candidate in &candidates[i] {
            let ran = candidate.range();
            let j: usize = max(1, ran.start) - 1;
            let k: usize = min(width - 1, ran.end) + 1;
            // println!("p:{} q:{}", p, q);
            for line in p..q {
                // print!("searching in line {}: {} ", line, &grid[line][j..k]);
                if re_symbol.is_match(&grid[line][j..k]) {
                    // println!("found");
                    numbers.push(candidate.as_str().parse().unwrap());
                    break;
                } else {
                    // println!("empty");
                };
            }
        }
    }
    numbers.iter().sum()
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn adjacent_to(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

#[derive(Debug, Clone)]
struct Span {
    points: Vec<Point>,
}

impl Span {
    fn adjacent_to(&self, other: &Point) -> bool {
        for point in &self.points {
            if point.adjacent_to(other) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    location: Span,
}

fn part_2(grid: Vec<String>) -> u32 {
    let re_number = Regex::new(r"\d+").unwrap();
    let re_asterisk = Regex::new(r"\*").unwrap();

    let numbers: Vec<Number> = grid
        .iter()
        .enumerate()
        .map(|(i, line)| {
            re_number
                .find_iter(line)
                .map(|m| Number {
                    value: m.as_str().parse::<u32>().unwrap(),
                    location: Span {
                        points: m
                            .range()
                            .map(|j| Point {
                                x: j as i32,
                                y: i as i32,
                            })
                            .collect(),
                    },
                })
                .collect::<Vec<Number>>()
        })
        .collect::<Vec<Vec<Number>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<Number>>();

    let asterisks: Vec<Point> = grid
        .iter()
        .enumerate()
        .map(|(i, line)| {
            re_asterisk
                .find_iter(line)
                .map(|m| Point {
                    x: m.range().start as i32,
                    y: i as i32,
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<Point>>();

    for number in &numbers {
        println!("{:?}", &number);
    }
    for asterisk in &asterisks {
        println!("{:?}", &asterisk);
    }

    let mut gears: Vec<(u32, u32)> = vec![];

    for asterisk in asterisks.iter() {
        let mut neighbours: Vec<u32> = vec![];
        for number in numbers.iter() {
            if number.location.adjacent_to(asterisk) {
                neighbours.push(number.value);
            }
        }
        if neighbours.len() == 2 {
            gears.push((neighbours[0], neighbours[1]))
        }
    }

    for gear in &gears {
        println!("{:?}", &gear);
    }

    gears.iter().map(|(a,b)| a*b).collect::<Vec<_>>().iter().sum()
}

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("./input.txt").unwrap())
        .lines()
        .map(|res| res.unwrap())
        .collect();
    let res = part_2(lines);
    println!("{:?}", res);
}
