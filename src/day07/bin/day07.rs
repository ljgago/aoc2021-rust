//! # Advent of Code - Day 7

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

fn parse(s: &str) -> Vec<i32> {
    s.split(",")
        .map(|num| num.trim().parse().unwrap())
        .collect()
}
