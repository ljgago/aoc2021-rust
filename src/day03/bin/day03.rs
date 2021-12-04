//! # Advent of Code - Day 3

mod part1;
mod part2;

fn main() {
    let input: Vec<String> = include_str!("../input.txt")
        .lines()
        .map(|x| x.to_owned())
        .collect();

    println!("--- Part One ---");
    println!("Result: {}", part1::compute_power_consumption(&input).unwrap());

    println!("--- Part Two ---");
    println!("Result: {}", part2::compute_life_support(&input).unwrap());
}
