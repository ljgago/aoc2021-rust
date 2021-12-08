//! # Advent of Code - Day 7 - Part Two

// this part can be solved with the Divide and Conquer algorithm

pub fn part2(positions: &[i32]) -> i32 {
    let mean_val: i32 = positions.iter().sum::<i32>() / positions.len() as i32;

    // I have inconsistences with decimal precision
    let mean_vec = vec![mean_val - 1, mean_val, mean_val + 1];

    // summation function
    let summation = |x: i32| (x + 1) * x / 2;

    mean_vec.iter()
        .map(|mean| {
            positions.iter()
                .map(|num| {
                    summation((num - mean).abs())
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod day07 {
    use super::*;

    #[test]
    fn test_part2() {
        let input: Vec<i32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(168, part2(&input));
    }
}
