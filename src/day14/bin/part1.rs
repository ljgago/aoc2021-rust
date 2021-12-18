//! # Advent of Code - Day 14 - Part One

use std::collections::HashMap;

pub fn part1(polymer: &[char], pair_insertion: &[(Vec<char>, char)], steps: i32) -> usize {
    let pair: HashMap<Vec<char>, char> = HashMap::from_iter(pair_insertion.to_vec());
    let mut elements: HashMap<char, usize> = HashMap::new();

    for el in polymer.iter() {
        if let Some(val) = elements.get_mut(el) {
            *val += 1;
        } else {
            elements.insert(*el, 1);
        }
    }

    for sub_polymer in polymer.windows(2) {
        compute_insertion(sub_polymer, &pair, &mut elements, steps)
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

fn compute_insertion(sub_polymer: &[char], pair: &HashMap<Vec<char>, char>, elements: &mut HashMap<char, usize>, steps: i32) {
    if steps < 1 {
        return;
    }

    let mut left_polymer: Vec<char> = Vec::new();
    let mut right_polymer: Vec<char> = Vec::new();

    left_polymer.insert(0, sub_polymer[0]);
    right_polymer.insert(0, sub_polymer[1]);

    if let Some(el) = pair.get(sub_polymer) {
        if let Some(val) = elements.get_mut(el) {
            *val += 1;
        } else {
            elements.insert(*el, 1);
        }

        left_polymer.insert(1, *el);
        right_polymer.insert(0, *el);
    }

    compute_insertion(&left_polymer, &pair, elements, steps - 1);
    compute_insertion(&right_polymer, &pair, elements, steps - 1);
}

#[cfg(test)]
mod day14 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests.txt");
        let (polymer_template, pair_insertion) = crate::parse(input);

        assert_eq!(1588, part1(&polymer_template, &pair_insertion, 10));
    }
}
