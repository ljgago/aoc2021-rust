//! # Advent of Code - Day 11 - Part One

use std::collections::HashSet;

pub fn part1(energy: &[Vec<usize>]) -> usize {
    let mut energy: Vec<Vec<usize>> = energy.to_vec();
    let mut flashes: usize = 0;

    for _ in 0..100 {
        flashes += compute_flashes(&mut energy);
    }

    return flashes;
}

fn compute_flashes(energy: &mut [Vec<usize>]) -> usize {
    let x_size = energy[0].len();
    let y_size = energy.len();

    let mut set_flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut set: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..x_size {
        for j in 0..y_size {
            update_energy((i, j), energy, &mut set_flashed, &mut set);
        }
    }

    compute_flashes_neighbors(energy, &mut set_flashed, &mut set);

    return set_flashed.len();
}

fn compute_flashes_neighbors(energy: &mut [Vec<usize>], set_flashed: &mut HashSet<(usize, usize)>, set: &mut HashSet<(usize, usize)>) {
    let x_size = energy[0].len();
    let y_size = energy.len();

    if set.len() == 0 {
        return;
    }

    let set_new = set.clone();
    set.clear();

    for (i, j) in set_new {
        // v x 0
        // x x 0
        // 0 0 0
        if i == 0 && j == 0 {
            for point in vec![(i + 1, j), (i + 1, j + 1), (i, j + 1), ] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }

        // x v x
        // x x x
        // 0 0 0
        if i > 0 && (i + 1) < x_size && j == 0 {
            for point in vec![(i + 1, j), (i + 1, j + 1), (i, j + 1), (i - 1, j + 1), (i - 1, j)] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }

        // 0 x v
        // 0 x x
        // 0 0 0
        if (i + 1) == x_size && j == 0 {
            for point in vec![(i, j + 1), (i - 1, j + 1), (i - 1, j)] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }

        // x x 0
        // v x 0
        // x x 0
        if i == 0 && j > 0 && (j + 1) < y_size {
            for point in vec![(i, j - 1), (i + 1 , j - 1), (i + 1, j), (i + 1, j + 1), (i, j + 1)] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }

        // x x x
        // x v x
        // x x x
        if i > 0 && (i + 1) < x_size && j > 0 && (j + 1) < y_size {
            for point in vec![(i, j - 1), (i + 1 , j - 1), (i + 1, j), (i + 1, j + 1), (i, j + 1), (i - 1, j + 1), (i - 1, j), (i - 1, j - 1)] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }

        // 0 x x
        // 0 x v
        // 0 x x
        if (i + 1) == x_size && j > 0 && (j + 1) < y_size {
            for point in vec![(i, j + 1), (i , j - 1), (i - 1, j + 1), (i - 1, j), (i - 1, j - 1)] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }

        // 0 0 0
        // x x 0
        // v x 0
        if i == 0 && (j + 1) == y_size {
            for point in vec![(i, j - 1), (i + 1, j - 1), (i + 1, j)] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }

        // 0 0 0
        // x x x
        // x v x
        if i > 0 && (i + 1) < x_size && (j + 1) == y_size {
            for point in vec![(i, j - 1), (i + 1, j - 1), (i + 1, j), (i - 1, j), (i - 1, j - 1)] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }

        // 0 0 0
        // 0 x x
        // 0 x v
        if (i + 1) == x_size && (j + 1) == y_size {
            for point in vec![(i, j - 1), (i - 1, j), (i - 1, j - 1)] {
                if None == set_flashed.get(&point) {
                    update_energy(point, energy, set_flashed, set);
                }
            }
        }
    }

    return compute_flashes_neighbors(energy, set_flashed, set);
}

fn update_energy(point: (usize, usize), energy: &mut [Vec<usize>], set_flashed: &mut HashSet<(usize, usize)>, set: &mut HashSet<(usize, usize)>) {
    let (i, j) = point;

    if energy[i][j] != 9 {
        energy[i][j] += 1;
    } else {
        energy[i][j] = 0;
        set_flashed.insert((i, j));
        set.insert((i, j));
    }
}

#[cfg(test)]
mod day11 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests.txt");
        let input = crate::parse(input);

        assert_eq!(1656, part1(&input));
    }
}
