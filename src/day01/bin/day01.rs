//! # Advent of Code - Day 1

mod part1;
mod part2;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    println!("--- Part One ---");
    println!("Result: {}", part1::part1_functional(&input));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2_functional(&input, 3));
}

fn parse(s: &str) -> Vec<usize> {
    s.lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
