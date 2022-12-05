#![feature(step_trait)]
#![feature(iter_array_chunks)]

use item::Priority;
use parser::parse_rucksacks;

use crate::parser::parse_groups;

mod group;
mod item;
mod parser;
mod rucksack;

fn main() {
    let input = include_str!("input.txt");
    let rucksacks = parse_rucksacks(input);
    let priorities_sum: i32 = rucksacks
        .iter()
        .map(|rucksack| rucksack.common_item().priority())
        .sum();
    println!("Priorities sum is {priorities_sum}");

    let groups = parse_groups(input);
    let group_priorities_sum = groups.iter().map(|g| g.badge_priority()).sum::<i32>();
    println!("Group priorities sum is {group_priorities_sum}");
}
