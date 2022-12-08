use std::str::Lines;

use crate::storage::{Directory, File};

fn parse_file(line: &str) -> File {
    let mut sections = line.split_whitespace();
    let size = sections.next().unwrap().parse::<i32>().unwrap();
    let name = sections.next().unwrap().to_string();
    File::new(name, size)
}

fn parse_directory(name: String, lines: &mut Lines) -> Directory {
    let mut dir = Directory::new(name);
    let mut next = lines.next();
    assert_eq!(next.unwrap(), "$ ls");

    while next.is_some() {
        let line = next.unwrap();
        if line.starts_with("$ cd ..") {
            return dir;
        } else if line.starts_with("$ cd") {
            let (_prefix, child_name) = line.split_at(5);
            let child_dir = parse_directory(child_name.to_string(), lines);
            dir.insert(Box::new(child_dir));
        } else if !line.starts_with("dir") {
            let child_file = parse_file(line);
            dir.insert(Box::new(child_file));
        }

        next = lines.next();
    }

    dir
}

pub fn parse(input: &str) -> Directory {
    let mut lines = input.lines();
    lines.next();
    parse_directory("/".to_string(), &mut lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
}
