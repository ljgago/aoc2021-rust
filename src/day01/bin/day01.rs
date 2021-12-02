//! # Advent of Code - Day 1

mod part1;
mod part2;

fn main() {
    let input: Vec<i32> = include_str!("../input.txt")
        .lines()
        .flat_map(|x| x.trim().parse::<i32>())
        .collect();

    println!("--- Part One ---");
    println!("Result: {}", part1::count_higher_measure_functional(&input).unwrap());

    println!("--- Part Two ---");
    println!("Result: {}", part2::count_higher_measure_functional(&input, 3).unwrap());
}
