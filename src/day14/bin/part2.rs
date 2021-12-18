//! # Advent of Code - Day 14 - Part Two

use std::collections::HashMap;

pub fn part2(polymer: &[char], pair_insertion: &[(Vec<char>, char)], steps: i32) -> usize {
    let pair_insertion: HashMap<Vec<char>, char> = HashMap::from_iter(pair_insertion.to_vec());

    let mut elements: HashMap<char, usize> = HashMap::new();
    for el in polymer.iter() {
        if let Some(val) = elements.get_mut(el) {
            *val += 1;
        } else {
            elements.insert(*el, 1);
        }
    }

    let mut pairs: HashMap<Vec<char>, usize> = HashMap::new();
    for pair in polymer.windows(2) {
        if let Some(val) = pairs.get_mut(pair) {
            *val += 1;
        } else {
            pairs.insert(pair.to_vec(), 1);
        }
    }

    for _ in 0..steps {
        let pairs_and_elements = pairs.iter()
            .fold((pairs.clone(), elements), |(mut pairs, mut elements), (pair, count)| {
                // subtract the count in the pairs
                if let Some(val) = pairs.get_mut(pair) {
                    *val -= count
                }

                // get the element and and update your count
                let el = *pair_insertion.get(pair).unwrap();
                if let Some(val) = elements.get_mut(&el) {
                    *val += count;
                } else {
                    elements.insert(el, *count);
                }

                // generate the new pairs to left and right
                let left_pair = vec![pair[0], el];
                let right_pair = vec![el, pair[1]];

                // insert the count in the new left and right pairs
                if let Some(val) = pairs.get_mut(&left_pair) {
                    *val += count;
                } else {
                    pairs.insert(left_pair, *count);
                }

                if let Some(val) = pairs.get_mut(&right_pair) {
                    *val += count;
                } else {
                    pairs.insert(right_pair, *count);
                }

                (pairs, elements)
            });

        pairs = pairs_and_elements.0;
        elements = pairs_and_elements.1;
    }

    let max = elements.iter()
        .map(|(_, v)| v)
        .max()
        .unwrap();

    let min = elements.iter()
        .map(|(_, v)| v)
        .min()
        .unwrap();

    return max - min;
}

#[cfg(test)]
mod day14 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../tests.txt");
        let (polymer_template, pair_insertion) = crate::parse(input);

        assert_eq!(2188189693529, part2(&polymer_template, &pair_insertion, 40));
    }
}
