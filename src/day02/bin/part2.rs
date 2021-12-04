//! # Advent of Code - Day 2 - Part Two

use crate::Command;

pub fn calc_position_depth_with_iam(commands: &[Command])-> Result<i32, &'static str> {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    let mut iam: i32 = 0;

    for command in commands.iter() {
        match command.name.as_str() {
            "forward" => {
                position += command.value;
                depth += iam * command.value;
            },
            "down" => iam += command.value,
            "up" => iam -= command.value,
            _ => return Err("Command not found"),
        }
    }

    Ok(position * depth)
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_calc_position_depth_with_iam() {
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

        assert_eq!(Ok(900), calc_position_depth_with_iam(&input));
    }
}
