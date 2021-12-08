//! # Advent of Code - Day 6 - Part Two

pub fn part2(fishes: &[usize], days: usize) -> usize {
    crate::part1::part1(&fishes, days)
}


#[cfg(test)]
mod day06 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = crate::parse("3,4,3,1,2");

        assert_eq!(26984457539, part2(&input, 256));
    }
}
