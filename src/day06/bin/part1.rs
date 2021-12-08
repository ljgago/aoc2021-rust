//! # Advent of Code - Day 6 - Part One

/*
 * | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | <- index
 *   0   1   1   2   1   0   0   0   0   <- quantities (Initial shoal)
 *   1   1   2   1   0   0   0   0   0   <- After 1 day
 *   1   2   1   0   0   0   1   0   1   <- After 2 day
 *   2   1   0   0   0   1   1   1   1   <- After 3 day
 *   1   0   0   0   1   1   3   1   2   <- After 4 day
 *
 * */

use std::collections::HashMap;

pub fn part1(fishes: &[usize], days: usize) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for fish in fishes.iter() {
        if let Some(val) = map.get_mut(fish) {
            *val += 1;
        } else {
            map.insert(*fish, 1);
        }
    }

    let mut initial_shoal: Vec<usize> = vec![0; 9];

    for i in 0..9 {
        if let Some(val) = map.get(&i) {
            initial_shoal[i] = *val;
        }
    }

    (0..days)
        .fold(initial_shoal, |mut shoal, _val| {
            simulation(&mut shoal);
            shoal
        })
        .iter()
        .sum()
}

fn simulation(shoal: &mut [usize]) {
    let zero = shoal[0];
    shoal.rotate_left(1);
    shoal[6] += zero;
}

#[cfg(test)]
mod day06 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = crate::parse("3,4,3,1,2");

        assert_eq!(5934, part1(&input, 80));
    }
}
