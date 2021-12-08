//! # Advent of Code - Day 3 - Part One

pub fn part1(input: &[String]) -> i32 {
    let num_len = input[0].len();

    let gamma_rate_str = input
        .iter()
        .map(|num_str| num_str
            .chars()
            .map(|c| if c == '1' { 1 } else { -1 })
            .collect::<Vec<i32>>()
        )
        .fold(vec![0; num_len], |acc, vec| {
            acc.iter()
                .zip(vec.iter())
                .map(|t| t.0 + t.1)
                .collect()
        })
        .iter()
        .map(|&num| if num > 0 { "1" } else { "0" })
        .collect::<String>();

    // create a mask depending the binary number length
    let mask: u32 = 2_u32.pow(num_len as u32) - 1;

    let gamma_rate = u32::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = !(gamma_rate as u32) & mask;

    (gamma_rate * epsilon_rate) as i32
}

#[cfg(test)]
mod day03 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests.txt");
        let input = crate::parse(input);

        assert_eq!(198, part1(&input));
    }
}
