//! # Advent of Code - Day 10 - Part One

use std::collections::HashMap;

pub fn part1(chunks: &[String]) -> usize {
    let score_map: HashMap<char, usize> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    chunks.iter()
        .map(|chunk| {
            let mut stack: Vec<char> = Vec::new();
            let mut point: usize = 0;

            for c in chunk.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    ')' | ']' | '}' | '>' => {
                        if let Some(val) = stack.pop() {
                            if val != c {
                                point = *score_map.get(&c).unwrap();
                                break;
                            }
                        } else {
                            break;
                        }
                    },
                    _ => ()
                }
            }
            point
        })
        .sum::<usize>()
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests.txt");
        let input = crate::parse(input);

        assert_eq!(26397, part1(&input));
    }
}
