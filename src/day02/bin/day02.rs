//! # Advent of Code - Day 2

mod part1;
mod part2;

use std::str::FromStr;
use std::convert::Infallible;

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub name: String,
    pub value: i32,
}

// parse str line to Command struct
impl FromStr for Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str_command = s.split_whitespace().collect::<Vec<&str>>();

        let name: String = str_command[0].to_owned();
        let value: i32 = str_command[1].parse().unwrap();

        Ok(Command{
            name,
            value,
        })
    }
}

fn main() {
    let input: Vec<Command> = include_str!("../input.txt")
        .lines()
        .flat_map(|x| x.trim().parse())
        .collect();

    println!("--- Part One ---");
    println!("Result: {}", part1::calc_position_depth(&input).unwrap());

    println!("--- Part Two ---");
    println!("Result: {}", part2::calc_position_depth_with_iam(&input).unwrap());
}
