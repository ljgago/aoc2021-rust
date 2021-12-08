//! # Advent of Code - Day 6

mod part1;
mod part2;

fn main() {
    let str_input = include_str!("../input.txt");
    let input = parse(str_input);

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(&input, 80));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(&input, 256));
}

fn parse(s: &str) -> Vec<usize> {
    s.split(",")
        .map(|num| num.trim().parse::<usize>().unwrap())
        .collect()
}
