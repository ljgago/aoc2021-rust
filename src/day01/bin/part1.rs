//! # Advent of Code - Day 1 - Part One

#[allow(dead_code)]
pub fn count_higher_measure(measurements: &[i32]) -> Result<i32, &'static str> {
    let mut count: i32 = 0;

    for i in 0..(measurements.len() - 1) {
        if measurements[i+1] > measurements[i] {
            count += 1;
        }
    }
    Ok(count)
}

pub fn count_higher_measure_functional(measurements: &[i32]) -> Result<i32, &'static str> {
    let count: i32 = measurements
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum();

    return Ok(count)
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_count_larger_measure() {
        let measurements: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(Ok(7), count_higher_measure(&measurements));
    }

    #[test]
    fn test_count_larger_measure_functional() {
        let measurements: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(Ok(7), count_higher_measure_functional(&measurements));
    }
}
