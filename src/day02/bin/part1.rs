//! # Advent of Code - Day 2 - Part One

use crate::Command;

pub fn calc_position_depth(commands: &[Command])-> Result<i32, &'static str> {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;

    for command in commands.iter() {
        match command.name.as_str() {
            "forward" => position += command.value,
            "down" => depth += command.value,
            "up" => depth -= command.value,
            _ => return Err("Command not found"),
        }
    }

    Ok(position * depth)
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_calc_position_depth() {
        let input: Vec<Command> = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ]
        .iter()
        .flat_map(|&x| x.parse())
        .collect();

        assert_eq!(Ok(150), calc_position_depth(&input));
    }
}
