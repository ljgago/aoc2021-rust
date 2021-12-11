//! # Advent of Code - Day 9

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

fn parse(s: &str) -> Vec<Vec<usize>> {
    s.lines()
        .map(|x| {
            x.trim().chars()
                .map(|y| y.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
