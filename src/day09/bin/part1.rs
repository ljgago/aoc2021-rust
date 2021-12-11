//! # Advent of Code - Day 9 - Part One

pub fn part1(heightmap: &[Vec<usize>]) -> usize {
    let x_size = heightmap.len();
    let y_size = heightmap[0].len();

    let mut height: usize = 0;

    for i in 0..x_size {
        for j in 0..y_size {
            height += check_adjacent(i, j, heightmap);
        }
    }

    return height;
}

fn check_adjacent(i: usize, j: usize, h: &[Vec<usize>]) -> usize {
    let x_size = h.len();
    let y_size = h[0].len();
    let v = h[i][j];

    // v 0 0
    // 0 0 0
    // 0 0 0
    if i == 0 && j == 0 {
        if h[i + 1][j] > v && h[i][j + 1] > v {
            return v + 1;
        }
    }

    // 0 v 0
    // 0 0 0
    // 0 0 0
    if i > 0 && (i + 1) < x_size && j == 0 {
        if h[i - 1][j] > v && h[i + 1][j] > v && h[i][j + 1] > v {
            return v + 1;
        }
    }

    // 0 0 v
    // 0 0 0
    // 0 0 0
    if (i + 1) == x_size && j == 0 {
        if h[i][j + 1] > v && h[i - 1][j] > v {
            return v + 1;
        }
    }

    // 0 0 0
    // v 0 0
    // 0 0 0
    if i == 0 && j > 0 && (j + 1) < y_size {
        if h[i][j - 1] > v && h[i][j + 1] > v && h[i + 1][j] > v {
            return v + 1;
        }
    }

    // 0 0 0
    // 0 v 0
    // 0 0 0
    if i > 0 && (i + 1) < x_size && j > 0 && (j + 1) < y_size {
        if h[i - 1][j] > v && h[i + 1][j] > v && h[i][j - 1] > v && h[i][j + 1] > v {
            return v + 1;
        }
    }

    // 0 0 0
    // 0 0 v
    // 0 0 0
    if (i + 1) == x_size && j > 0 && (j + 1) < y_size {
        if h[i][j - 1] > v && h[i][j + 1] > v && h[i - 1][j] > v {
            return v + 1;
        }
    }

    // 0 0 0
    // 0 0 0
    // v 0 0
    if i == 0 && (j + 1) == y_size {
        if h[i][j - 1] > v && h[i + 1][j] > v {
            return v + 1;
        }
    }

    // 0 0 0
    // 0 0 0
    // 0 v 0
    if i > 0 && (i + 1) < x_size && (j + 1) == y_size {
        if h[i - 1][j] > v && h[i + 1][j] > v && h[i][j - 1] > v {
            return v + 1;
        }
    }

    // 0 0 0
    // 0 0 0
    // 0 0 v
    if (i + 1) == x_size && (j + 1) == y_size {
        if h[i - 1][j] > v && h[i][j - 1] > v {
            return v + 1;
        }
    }

    return 0;
}

#[cfg(test)]
mod day09 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests.txt");
        let input = crate::parse(input);

        assert_eq!(15, part1(&input));
    }
}
