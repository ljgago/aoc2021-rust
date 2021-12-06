//! # Advent of Code - Day 4

mod part1;
mod part2;

fn main() {
    let input = include_str!("../input.txt");
    let (numbers, mut boards) = parse(input);

    println!("--- Part One ---");
    println!("Result: {}", part1::bingo_score(&numbers, &mut boards).unwrap());

    println!("--- Part Two ---");
    println!("Result: {}", part2::bingo_score(&numbers, &mut boards).unwrap());
}

fn parse(s: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let data: Vec<&str> = s.trim().split("\n\n").collect();

    let numbers: Vec<i32> = data[0]
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();

    for i in 1..data.len() {
        let board: Vec<Vec<i32>> = data[i]
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|value| value.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();
        boards.push(board);
    }

    (numbers, boards)
}
