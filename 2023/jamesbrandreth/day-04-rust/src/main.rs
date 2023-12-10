use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Card {
    numbers: Vec<u32>,
    winners: Vec<u32>,
}

impl Card {
    fn matches(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|&n| self.winners.contains(n))
            .collect::<Vec<_>>()
            .len() as u32
    }

    fn value(&self) -> u32 {
        let x = self.matches();
        match x {
            0 => 0,
            _ => 2_u32.pow(x - 1),
        }
    }
}

fn parse_number_list(s: &str) -> Vec<u32> {
    let r = &s.split(' ').collect::<Vec<&str>>();
    r.iter().filter_map(|n| n.parse::<u32>().ok()).collect()
}

fn part_1(lines: Vec<String>) -> u32 {
    let re = Regex::new(r"Card\s+\d+\: ([\d ]*) \| ([\d ]*)").unwrap();
    let cards: Vec<Card> = lines
        .iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Card {
                numbers: parse_number_list(&caps[1]),
                winners: parse_number_list(&caps[2]),
            }
        })
        .collect::<Vec<_>>();
    cards.iter().map(|card| card.value()).sum()
}

fn part_2(lines: Vec<String>) -> u32 {
    let re = Regex::new(r"Card\s+\d+\: ([\d ]*) \| ([\d ]*)").unwrap();
    let mut cards: Vec<(u32, Card)> = lines
        .iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                1,
                Card {
                    numbers: parse_number_list(&caps[1]),
                    winners: parse_number_list(&caps[2]),
                },
            )
        })
        .collect::<Vec<_>>();
    for i in 0..cards.len() {
        let d = cards[i].1.matches() as usize;
        for j in (i + 1)..(i + d + 1) {
            cards[j].0 += cards[i].0;
        }
    }

    cards.iter().map(|(n, card)| n).sum()
}

fn main() {
    let lines: Vec<String> = BufReader::new(File::open("./input.txt").unwrap())
        .lines()
        .map(|res| res.unwrap())
        .collect();
    let res = part_2(lines);
    println!("{:?}", res);
}
