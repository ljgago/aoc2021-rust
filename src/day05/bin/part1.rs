//! # Advent of Code - Day 5 - Part One

use std::collections::HashMap;

pub fn part1(vents_lines: &[(Vec<usize>, Vec<usize>)]) -> usize {
    let map_zero: HashMap<Vec<usize>, usize> = HashMap::new();

    vents_lines.iter()
        .fold(map_zero, |mut map, line| {
            build_vents_lines(&mut map, line);
            map
        })
        .iter()
        .filter(|(_key, val)| **val > 1)
        .count()
}

fn build_vents_lines(map: &mut HashMap<Vec<usize>, usize>, line: &(Vec<usize>, Vec<usize>)) {
    if line.0[0] != line.1[0] && line.0[1] != line.1[1] {
        return;
    }

    let x_line: Vec<usize> = if line.0[0] <= line.1[0] {
        (line.0[0]..=line.1[0]).collect()
    } else {
        (line.1[0]..=line.0[0]).rev().collect()
    };

    let y_line: Vec<usize> = if line.0[1] <= line.1[1] {
        (line.0[1]..=line.1[1]).collect()
    } else {
        (line.1[1]..=line.0[1]).rev().collect()
    };

    for x in x_line.iter() {
        for y in y_line.iter() {
            let point = vec![*x, *y];
            if let Some(val) = map.get_mut(&point) {
                *val += 1;
            } else {
                map.insert(point, 1);
            }
        }
    }
}

#[cfg(test)]
mod day05 {
    use super::*;

    #[test]
    fn test_part1() {
        let str_input = include_str!("../tests.txt");
        let input = crate::parse(str_input);

        assert_eq!(5, part1(&input));
    }
}
