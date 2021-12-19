//! # Advent of Code - Day 12 - Part One

use crate::Graph;
use std::collections::HashSet;

pub fn part1(caves: &[(String, String)]) -> usize {
    let mut graph: Graph = Graph::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut count: usize = 0;

    for cave in caves {
        graph.add_edge(&cave.0, &cave.1);
    }

    compute_dfs(&graph, &mut visited, "start", &mut count);

    return count;
}

fn compute_dfs(graph: &Graph, visited: &mut HashSet<String>, vertex: &str, count: &mut usize) {
    if vertex == "end" {
        *count += 1;
        return;
    }

    if vertex.to_owned() == vertex.to_lowercase() {
        visited.insert(vertex.to_owned());
    }

    if let Some(neighbors) = graph.get_neighbors(vertex) {
        for neighbor in neighbors.iter() {
            if visited.get(neighbor) == None {
                compute_dfs(graph, visited, &neighbor, count);
                visited.remove(neighbor);
            }
        }
    }
}

#[cfg(test)]
mod day12 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test1.txt");
        let input = crate::parse(input);
        assert_eq!(10, part1(&input));

        let input = include_str!("../test2.txt");
        let input = crate::parse(input);
        assert_eq!(19, part1(&input));

        let input = include_str!("../test3.txt");
        let input = crate::parse(input);
        assert_eq!(226, part1(&input));
    }
}
