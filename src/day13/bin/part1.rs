//! # Advent of Code - Day 13 - Part One

use std::collections::HashSet;

pub fn part1(points_vec: &[(i32, i32)], folds: &[(char, i32)]) -> usize {
    let mut points: HashSet<(i32, i32)> = HashSet::from_iter(points_vec.iter().cloned());

    compute_fold(&mut points, &folds[0]);

    return points.len();
}

fn compute_fold(points: &mut HashSet<(i32, i32)>, fold_along: &(char, i32)) {
    let fold_coord = fold_along.0;
    let fold_value = fold_along.1;

    for (x, y) in points.clone().into_iter() {
        if 'x' == fold_coord {
            if x > fold_value {
                points.insert(((x - fold_value * 2).abs(), y));
                points.remove(&(x, y));
            }
        } else {
            if y > fold_value {
                points.insert((x, (y - fold_value * 2).abs()));
                points.remove(&(x, y));
            }
        }
    }
}

#[cfg(test)]
mod day13 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests.txt");
        let (points, folds) = crate::parse(input);

        assert_eq!(17, part1(&points, &folds));
    }
}
