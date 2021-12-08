//! # Advent of Code - Day 5

mod part1;
mod part2;

fn main() {
    let str_input = include_str!("../input.txt");
    let input = parse(str_input);

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(&input));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(&input));
}

fn parse(s: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    s.lines().map(|line| {
        let (start, end) = line.split_once(" -> ").unwrap();
        (
            start.split(",").map(|s| s.parse::<usize>().unwrap()).collect(),
              end.split(",").map(|e| e.parse::<usize>().unwrap()).collect(),
        )
    })
    .collect()
}
