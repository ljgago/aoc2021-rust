//! # Advent of Code - Day 7 - Part One

pub fn part1(positions: &[i32]) -> i32 {
    let mut positions = positions.to_vec();
    positions.sort();

    let mid: usize = positions.len() / 2;

    let median: i32 = if positions.len() % 2 == 0 {
        (positions[mid - 1] + positions[mid]) / 2
    } else {
        positions[mid]
    };

    positions.iter()
        .map(|num| (num - median).abs())
        .sum()

}

#[cfg(test)]
mod day07 {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(37, part1(&input));
    }
}
