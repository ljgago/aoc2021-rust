//! # Advent of Code - Day 13

mod part1;
mod part2;

fn main() {
    let input = include_str!("../input.txt");
    let (points, folds) = crate::parse(input);

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(&points, &folds));

    println!("--- Part Two ---");
    println!("Result: \n{}", part2::part2(&points, &folds));
}

fn parse(s: &str) -> (Vec<(i32, i32)>, Vec<(char, i32)>) {
    let data: Vec<&str> = s.split("\n\n").collect();

    let points = data[0]
        .trim()
        .lines()
        .map(|point| {
            let p = point
                .split(",")
                .map(|coord| coord.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (p[0], p[1])
        })
        .collect::<Vec<(i32, i32)>>();

    let folds = data[1]
        .trim()
        .lines()
        .map(|fold| {
            let mut f = fold.to_owned();
            f.drain(..11);
            let f: Vec<&str> = f.split("=").collect();
            (f[0].chars().nth(0).unwrap(), f[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<(char, i32)>>();

    (points, folds)
}
