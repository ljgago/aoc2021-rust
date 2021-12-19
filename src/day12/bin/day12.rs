//! # Advent of Code - Day 12

mod part1;
mod part2;

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    println!("--- Part One ---");
    println!("Result: {}", part1::part1(&input));

    println!("--- Part Two ---");
    println!("Result: {}", part2::part2(&input));
}

fn parse(s: &str) -> Vec<(String, String)> {
    s.lines()
        .map(|x| {
            let edges: Vec<&str> = x.trim()
                .split("-")
                .collect();

            (edges[0].to_owned(), edges[1].to_owned())
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { edges: HashMap::new(), }
    }

    pub fn add_edge(&mut self, vert1: &str, vert2: &str) {
        if let Some(val) = self.edges.get_mut(&vert1.to_string()) {
            val.insert(vert2.to_string());
        } else {
            let mut v = HashSet::new();
            v.insert(vert2.to_string());
            self.edges.insert(vert1.to_string(), v);
        }

        if let Some(val) = self.edges.get_mut(&vert2.to_string()) {
            val.insert(vert1.to_string());
        } else {
            let mut v = HashSet::new();
            v.insert(vert1.to_string());
            self.edges.insert(vert2.to_string(), v);
        }
    }

    pub fn get_neighbors(&self, vert: &str) -> Option<&HashSet<String>> {
        return self.edges.get(vert);
    }
}
