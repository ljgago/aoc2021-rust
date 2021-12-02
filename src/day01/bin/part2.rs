//! # Advent of Code - Day 1 - Part Two

#[allow(dead_code)]
fn gen_window(measurements: &[i32]) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();

    for i in 0..(measurements.len() - 2) {
        let sum = measurements[i] + measurements[i+1] + measurements[i+2];
        out.push(sum);
    }

    return out;
}

#[allow(dead_code)]
pub fn count_higher_measure(measurements: &[i32]) -> Result<i32, &'static str> {
    let window_measurements = gen_window(measurements);

    let mut count: i32 = 0;
    for i in 0..(window_measurements.len() - 1) {
        if window_measurements[i+1] > window_measurements[i] {
            count += 1;
        }
    }
    Ok(count)
}

pub fn count_higher_measure_functional(measurements: &[i32], window: usize) -> Result<i32, &'static str> {
    let measure_window: Vec<i32> = measurements
        .windows(window)
        .map(|x| x.iter().sum())
        .collect();

    let count: i32 = measure_window[..]
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum();

    return Ok(count)
}


#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_gen_window() {
        let measurements: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected: Vec<i32> = vec![607, 618, 618, 617, 647, 716, 769, 792];
        assert_eq!(expected, gen_window(&measurements));
    }

    #[test]
    fn test_count_higher_measure() {
        let measurements: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(Ok(5), count_higher_measure(&measurements));
    }

    #[test]
    fn test_count_higher_measure_functional() {
        let measurements: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(Ok(5), count_higher_measure_functional(&measurements, 3));
    }
}
