//! # Advent of Code - Day 5 - Part Two

use std::collections::HashMap;

pub fn part2(vents_lines: &[(Vec<usize>, Vec<usize>)]) -> usize {
    let map_zero: HashMap<Vec<usize>, usize> = HashMap::new();

    vents_lines.iter()
        .fold(map_zero, |mut map, line| {
            build_vents_lines_horizontal_vertical(&mut map, line);
            build_vents_lines_diagonal(&mut map, line);
            map
        })
        .iter()
        .filter(|(_key, val)| **val > 1)
        .count()
}

fn build_vents_lines_horizontal_vertical(map: &mut HashMap<Vec<usize>, usize>, line: &(Vec<usize>, Vec<usize>)) {
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

fn build_vents_lines_diagonal(map: &mut HashMap<Vec<usize>, usize>, line: &(Vec<usize>, Vec<usize>)) {
    if line.0[0] == line.1[0] || line.0[1] == line.1[1] {
        return;
    }

    let x_line: Vec<usize> = if line.0[0] <= line.1[0] {
        (line.0[0]..=line.1[0]).collect()
    } else {
        (line.1[0]..=line.0[0]).rev().collect()
    };

    let mut y_line: Vec<usize> = if line.0[1] <= line.1[1] {
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
            // delete the first element in the y_line for run in diagonal
            y_line.remove(0);
            break;
        }
    }
}

#[cfg(test)]
mod day05 {
    use super::*;

    #[test]
    fn test_part2() {
        let str_input = include_str!("../tests.txt");
        let input = crate::parse(str_input);

        assert_eq!(12, part2(&input));
    }
}
