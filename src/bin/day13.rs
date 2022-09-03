use regex::Regex;

extern crate aoc2015;
use aoc2015::graph::*;

fn main() {
    let contents = std::fs::read_to_string("./input/input13.txt").unwrap();
    let mut graph = create_graph(&contents);
    println!("Graph: {}", graph);
    let (_min, max) = graph.find_shortest_and_longest_loop_length();
    println!("Solution1: {}", max);
    for vertex in graph.vertices.clone() {
        graph.insert_edge_or_increase_weight("Me", &vertex.name, 0);
    }
    let (_min, max) = graph.find_shortest_and_longest_loop_length();
    println!("Solution2: {}", max);
}

fn create_graph(lines: &str) -> Graph {
    let mut graph = Graph::new();
    for line in lines.lines() {
        let (from, to, dist) = parse_line(line);
        graph.insert_edge_or_increase_weight(&from, &to, dist);
    }
    graph
}

fn parse_line(line: &str) -> (String, String, i32) {
    lazy_static::lazy_static! {
        static ref REG: Regex = Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$").unwrap();
    }
    let caps = REG.captures(line).unwrap();
    let from: String = caps.get(1).map(|m| String::from(m.as_str())).unwrap();
    let gain_or_lose: String = caps.get(2).map(|m| String::from(m.as_str())).unwrap();
    let gain_or_lose = if gain_or_lose == "gain" { 1 } else { -1 };
    let dist: i32 = caps
        .get(3)
        .map(|m| gain_or_lose * m.as_str().parse::<i32>().unwrap())
        .unwrap();
    let to: String = caps.get(4).map(|m| String::from(m.as_str())).unwrap();
    (from, to, dist)
}
