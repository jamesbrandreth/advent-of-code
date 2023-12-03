use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Game {
    id: u16,
    reveals: Vec<(u16, u16, u16)>,
}

impl Game {
    fn from_string(s: &str) -> Game {
        let (game_name, reveals_string) = s.split_once(':').unwrap();
        let (_, id_string) = game_name.split_once(' ').unwrap();
        Game {
            id: id_string.parse().unwrap(),
            reveals: reveals_string
                .split(';')
                .map(|reveal| {
                    let mut vals: (u16, u16, u16) = (0, 0, 0);
                    for colour_string in reveal.split(',') {
                        let (number, colour) = colour_string.trim().split_once(' ').unwrap();
                        let num: u16 = number.parse().unwrap();
                        match colour {
                            "red" => vals.0 = num,
                            "green" => vals.1 = num,
                            "blue" => vals.2 = num,
                            _ => panic!(),
                        };
                    }
                    vals
                })
                .collect(),
        }
    }

    fn possible_with(&self, bag: (u16, u16, u16)) -> bool {
        let mut possible: bool = true;
        for reveal in &self.reveals {
            possible = reveal.0 <= bag.0 && reveal.1 <= bag.1 && reveal.2 <= bag.2 && possible;
        }
        possible
    }

    fn minimum_bag(&self) -> (u16, u16, u16) {
        let mut bag: (u16, u16, u16) = (0, 0, 0);
        for reveal in &self.reveals {
            bag.0 = max(bag.0, reveal.0);
            bag.1 = max(bag.1, reveal.1);
            bag.2 = max(bag.2, reveal.2);
        }
        bag
    }
}

fn power(bag: (u16, u16, u16)) -> u16 {
    bag.0 * bag.1 * bag.2
}

fn part_1(filepath: &str) -> Result<u16, std::io::Error> {
    println!("{}", &filepath);
    let games: Vec<Game> = BufReader::new(File::open(filepath)?)
        .lines()
        .map(|line| Game::from_string(&line.unwrap()))
        .collect();

    let mut total: u16 = 0;
    for game in games {
        if game.possible_with((12, 13, 14)) {
            total += &game.id;
        }
    }
    Ok(total)
}

fn part_2(filepath: &str) -> Result<u16, std::io::Error> {
    println!("{}", &filepath);
    let games: Vec<Game> = BufReader::new(File::open(filepath)?)
        .lines()
        .map(|line| Game::from_string(&line.unwrap()))
        .collect();

    let mut total: u16 = 0;
    for game in games {
        total += power(game.minimum_bag());
    }
    Ok(total)
}

fn main() {
    println!("{}", part_2("./input.txt").unwrap());
}
