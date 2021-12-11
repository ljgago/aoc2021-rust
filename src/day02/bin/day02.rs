//! # Advent of Code - Day 2

mod part1;
mod part2;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(&input));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(&input));
}

use std::str::FromStr;
use std::convert::Infallible;

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub name: String,
    pub value: usize,
}

// parse str line to Command struct
impl FromStr for Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_command = s.split_whitespace().collect::<Vec<&str>>();

        let name: String = str_command[0].to_owned();
        let value: usize = str_command[1].parse().unwrap();

        Ok(Command{
            name,
            value,
        })
    }
}

fn parse(s: &str) -> Vec<Command> {
    s.lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
