//! # Advent of Code - Day 13 - Part Two

use std::collections::HashSet;

pub fn part2(points_vec: &[(i32, i32)], folds: &[(char, i32)]) -> String {
    let mut points: HashSet<(i32, i32)> = HashSet::from_iter(points_vec.iter().cloned());

    for fold_along in folds.iter() {
        compute_fold(&mut points, fold_along);
    }

    return print_paper(&points);
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

fn print_paper(points: &HashSet<(i32, i32)>) -> String {
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    for (x, y) in points.iter() {
        if *x > x_max {
            x_max = *x;
        }
        if *y > y_max {
            y_max = *y;
        }
    }

    let mut paper = vec![vec!["."; 1 + x_max as usize]; 1 + y_max as usize];

    for y in 0..=y_max {
        for x in 0..=x_max {
            if points.get(&(x, y)) != None {
                paper[y as usize][x as usize] = "#";
            }
        }
    }

    // transform the matrix to string
    paper.iter()
        .map(|line| line.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod day13 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../tests.txt");
        let (points, folds) = crate::parse(input);
        let expected: String = vec![
            "#####",
            "#...#",
            "#...#",
            "#...#",
            "#####",
        ].join("\n");

        assert_eq!(expected, part2(&points, &folds));
    }
}
