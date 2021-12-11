//! # Advent of Code - Day 10 - Part Two

use std::collections::HashMap;

pub fn part2(chunks: &[String]) -> usize {
    let mut scores: Vec<usize> = chunks.iter()
        .map(|chunk| {
            let mut stack: Vec<char> = Vec::new();

            for c in chunk.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    ')' | ']' | '}' | '>' => {
                        if let Some(val) = stack.pop() {
                            if val != c {
                                stack.clear();
                                break;
                            }
                        } else {
                            break;
                        }
                    },
                    _ => (),
                }
            }
            stack.reverse();
            stack
        })
        .filter(|missing_chars| missing_chars.len() != 0)
        .map(|missing_chars| {
            compute_score(&missing_chars)
        })
        .collect();

        scores.sort();
        scores[scores.len() / 2]
}

fn compute_score(chars: &[char]) -> usize {
    let score_map: HashMap<char, usize> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

    chars.iter()
        .fold(0, |score, c| {
            let s = score_map.get(&c).unwrap();
            score * 5 + s
        })
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../tests.txt");
        let input = crate::parse(input);

        assert_eq!(288957, part2(&input));
    }
}
