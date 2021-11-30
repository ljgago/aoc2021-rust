//! # Advent of Code - Day 8

mod part1;
mod part2;

fn main() {
    let _input = include_bytes!("../input.txt");

    println!("--- Part One ---");
    println!("Result: {}", part1::result().unwrap());

    println!("--- Part Two ---");
    println!("Result: {}", part2::result().unwrap());
}
