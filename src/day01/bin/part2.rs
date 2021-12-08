//! # Advent of Code - Day 1 - Part Two

#[allow(dead_code)]
pub fn part2(measurements: &[usize]) -> usize {
    let window_measurements = gen_window(measurements);

    let mut count: usize = 0;
    for i in 0..(window_measurements.len() - 1) {
        if window_measurements[i+1] > window_measurements[i] {
            count += 1;
        }
    }

    count
}

#[allow(dead_code)]
fn gen_window(measurements: &[usize]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();

    for i in 0..(measurements.len() - 2) {
        let sum = measurements[i] + measurements[i+1] + measurements[i+2];
        out.push(sum);
    }

    out
}

pub fn part2_functional(measurements: &[usize], window: usize) -> usize {
    let measure_window: Vec<usize> = measurements
        .windows(window)
        .map(|x| x.iter().sum())
        .collect();

    measure_window[..]
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_part2() {
        let input: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(5, part2(&input));
        assert_eq!(5, part2_functional(&input, 3));

        let expected: Vec<usize> = vec![607, 618, 618, 617, 647, 716, 769, 792];
        assert_eq!(expected, gen_window(&input));
    }
}
