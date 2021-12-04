//! # Advent of Code - Day 3 - Part Two

fn compute_rating(input: &[String], index: usize, mode: &str) -> Vec<String> {
    if input.len() == 1 {
        return input.to_vec();
    }

    let num_len = input[0].len();

    let num_input: Vec<Vec<i32>> = input
        .iter()
        .map(|num_str| num_str
            .chars()
            .map(|c| if c == '1' { 1 } else { -1 })
            .collect::<Vec<i32>>()
        )
        .collect();

    let mask: Vec<i32> = num_input
        .iter()
        .fold(vec![0; num_len], |acc, vec| {
            acc.iter()
                .zip(vec.iter())
                .map(|t| t.0 + t.1)
                .collect()
        });

    let filtered_input: Vec<String> = num_input
        .iter()
        .filter(|num| {
            if mode == "most_common" {
                if mask[index] >= 0 {
                    num[index] > 0
                } else {
                    num[index] < 0
                }
            } else {
                if mask[index] >= 0 {
                    num[index] < 0
                } else {
                    num[index] > 0
                }
            }
        })
        .map(|num| num.iter()
            .map(|&d| if d == 1 { '1' } else { '0' })
            .collect::<Vec<char>>()
        )
        .map(|x| String::from_iter(x))
        .collect();

    return compute_rating(&filtered_input, index + 1, mode);
}

pub fn compute_life_support(input: &[String]) -> Result<i32, &'static str> {

    let most_common: Vec<String> = compute_rating(&input, 0, "most_common");
    let least_common: Vec<String> = compute_rating(&input, 0, "least_common");

    let most_common_value: i32 = i32::from_str_radix(&most_common[0], 2).unwrap();
    let least_common_value: i32 = i32::from_str_radix(&least_common[0], 2).unwrap();

    Ok(most_common_value * least_common_value)
}


#[cfg(test)]
mod day03 {
    use super::*;

    #[test]
    fn test_compute_life_support() {
        let input: Vec<String> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ]
        .iter()
        .map(|&x| x.to_owned())
        .collect();

        assert_eq!(Ok(230), compute_life_support(&input));
    }

    #[test]
    fn test_compute_rating() {
        let input: Vec<String> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ]
        .iter()
        .map(|&x| x.to_owned())
        .collect();

        assert_eq!(vec!["10111"], compute_rating(&input, 0, "most_common"));
        assert_eq!(vec!["01010"], compute_rating(&input, 0, "least_common"));
    }
}
