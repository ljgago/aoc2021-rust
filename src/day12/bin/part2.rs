//! # Advent of Code - Day 12 - Part Two

use crate::Graph;
use std::collections::HashSet;

pub fn part2(caves: &[(String, String)]) -> usize {
    let mut graph: Graph = Graph::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut count: usize = 0;
    let mut twice: bool = false;

    for cave in caves {
        graph.add_edge(&cave.0, &cave.1);
    }

    compute_dfs(&graph, &mut visited, "start", &mut twice, &mut count);

    return count;
}

fn compute_dfs(graph: &Graph, visited: &mut HashSet<String>, vertex: &str, twice: &mut bool, count: &mut usize) {
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
                compute_dfs(graph, visited, &neighbor, twice, count);
                visited.remove(neighbor);
            } else if *twice == false && neighbor != "start" {
                *twice = true;
                compute_dfs(graph, visited, &neighbor, twice, count);
                *twice = false;
            }
        }
    }
}

#[cfg(test)]
mod day12 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../test1.txt");
        let input = crate::parse(input);
        assert_eq!(36, part2(&input));

        let input = include_str!("../test2.txt");
        let input = crate::parse(input);
        assert_eq!(103, part2(&input));

        let input = include_str!("../test3.txt");
        let input = crate::parse(input);
        assert_eq!(3509, part2(&input));
    }
}
