//! # Advent of Code - Day 1 - Part One

#[allow(dead_code)]
pub fn part1(measurements: &[usize]) -> usize {
    let mut count: usize = 0;

    for i in 0..(measurements.len() - 1) {
        if measurements[i+1] > measurements[i] {
            count += 1;
        }
    }
    count
}

pub fn part1_functional(measurements: &[usize]) -> usize {
    measurements
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, part1(&input));
        assert_eq!(7, part1_functional(&input));
    }
}
