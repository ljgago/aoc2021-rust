//! # Advent of Code - Day 2 - Part One

use crate::Command;

pub fn part1(commands: &[Command])-> usize {
    let mut position: usize = 0;
    let mut depth: usize = 0;

    for command in commands.iter() {
        match command.name.as_str() {
            "forward" => position += command.value,
            "down" => depth += command.value,
            "up" => depth -= command.value,
            _ => (),
        }
    }

    position * depth
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests.txt");
        let input = crate::parse(input);

        assert_eq!(150, part1(&input));
    }
}
