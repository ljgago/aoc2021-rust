//! # Advent of Code - Day 2 - Part Two

use crate::Command;

pub fn part2(commands: &[Command])-> usize {
    let mut position: usize = 0;
    let mut depth: usize = 0;
    let mut iam: usize = 0;

    for command in commands.iter() {
        match command.name.as_str() {
            "forward" => {
                position += command.value;
                depth += iam * command.value;
            },
            "down" => iam += command.value,
            "up" => iam -= command.value,
            _ => (),
        }
    }

    position * depth
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../tests.txt");
        let input = crate::parse(input);

        assert_eq!(900, part2(&input));
    }
}
