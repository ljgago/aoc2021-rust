//! # Advent of Code - Day 14

mod part1;
mod part2;

fn main() {
    let input = include_str!("../input.txt");
    let (polymer_template, pair_insertion) = parse(input);

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(&polymer_template, &pair_insertion, 10));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(&polymer_template, &pair_insertion, 40));
}

fn parse(s: &str) -> (Vec<char>, Vec<(Vec<char>, char)>) {
    let data: Vec<&str> = s.split("\n\n").collect();

    let polymer_template = data[0].chars().collect();

    let pair_insertion = data[1]
        .lines()
        .map(|line| {
            let pair: Vec<&str> = line.split(" -> ").collect();
            (pair[0].chars().collect(), pair[1].chars().nth(0).unwrap())
        })
        .collect();

    (polymer_template, pair_insertion)
}
