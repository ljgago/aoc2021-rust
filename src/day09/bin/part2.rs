//! # Advent of Code - Day 9 - Part Two

use std::collections::HashSet;

pub fn part2(heightmap: &[Vec<usize>]) -> usize {
    let x_size = heightmap.len();
    let y_size = heightmap[0].len();

    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut sizes: Vec<usize> = Vec::new();

    for i in 0..x_size {
        for j in 0..y_size {
            let result = compute_basin(i, j, heightmap, &mut points);
            sizes.push(result);
        }
    }

    sizes.sort_by(|a, b| b.cmp(a));

    return sizes[0] * sizes[1] * sizes[2];
}

fn compute_basin(i: usize, j: usize, h: &[Vec<usize>], points: &mut HashSet<(usize, usize)>) -> usize {
    let x_size = h.len();
    let y_size = h[0].len();
    let mut result = 0;

    // check if the point has already used
    if points.get(&(i, j)) == None {
        points.insert((i, j));
    } else {
        return 0;
    }

    // check if the value of the point is 9
    if h[i][j] == 9 {
        return 0;
    }

    // v 0 0
    // 0 0 0
    // 0 0 0
    if i == 0 && j == 0 {
        result += 1;
        result += compute_basin(i + 1, j, h, points);
        result += compute_basin(i, j + 1, h, points);
    }

    // 0 v 0
    // 0 0 0
    // 0 0 0
    if i > 0 && (i + 1) < x_size && j == 0 {
        result += 1;
        result += compute_basin(i - 1, j, h, points);
        result += compute_basin(i + 1, j, h, points);
        result += compute_basin(i, j + 1, h, points);
    }

    // 0 0 v
    // 0 0 0
    // 0 0 0
    if (i + 1) == x_size && j == 0 {
        result += 1;
        result += compute_basin(i - 1, j, h, points);
        result += compute_basin(i, j + 1, h, points);
    }

    // 0 0 0
    // v 0 0
    // 0 0 0
    if i == 0 && j > 0 && (j + 1) < y_size {
        result += 1;
        result += compute_basin(i + 1, j, h, points);
        result += compute_basin(i, j - 1, h, points);
        result += compute_basin(i, j + 1, h, points);
    }

    // 0 0 0
    // 0 v 0
    // 0 0 0
    if i > 0 && (i + 1) < x_size && j > 0 && (j + 1) < y_size {
        result += 1;
        result += compute_basin(i - 1, j, h, points);
        result += compute_basin(i + 1, j, h, points);
        result += compute_basin(i, j - 1, h, points);
        result += compute_basin(i, j + 1, h, points);
    }

    // 0 0 0
    // 0 0 v
    // 0 0 0
    if (i + 1) == x_size && j > 0 && (j + 1) < y_size {
        result += 1;
        result += compute_basin(i - 1, j, h, points);
        result += compute_basin(i, j - 1, h, points);
        result += compute_basin(i, j + 1, h, points);
    }

    // 0 0 0
    // 0 0 0
    // v 0 0
    if i == 0 && (j + 1) == y_size {
        result += 1;
        result += compute_basin(i + 1, j, h, points);
        result += compute_basin(i, j - 1, h, points);
    }

    // 0 0 0
    // 0 0 0
    // 0 v 0
    if i > 0 && (i + 1) < x_size && (j + 1) == y_size {
        result += 1;
        result += compute_basin(i - 1, j, h, points);
        result += compute_basin(i + 1, j, h, points);
        result += compute_basin(i, j - 1, h, points);
    }

    // 0 0 0
    // 0 0 0
    // 0 0 v
    if (i + 1) == x_size && (j + 1) == y_size {
        result += 1;
        result += compute_basin(i - 1, j, h, points);
        result += compute_basin(i, j - 1, h, points);
    }

    return result;

}

#[cfg(test)]
mod day09 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../tests.txt");
        let input = crate::parse(input);

        assert_eq!(1134, part2(&input));
    }
}
