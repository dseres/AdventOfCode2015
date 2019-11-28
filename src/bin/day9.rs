use regex::Regex;
extern crate aoc2015;
use aoc2015::graph::*;

fn main() {
    let contents =
        std::fs::read_to_string("./input/input9.txt").expect("Unable to open input file.");
    let graph = parse(&contents);
    println!("Graph:\n{}", graph);
    let (s1, s2) = graph.find_shortest_and_longest_path_length();
    println!("Solution1: {}", s1);
    println!("Solution2: {}", s2);
}

fn parse(contents: &str) -> Graph {
    let mut graph = Graph::new();
    for line in contents.lines() {
        let (from,to,dist) = parse_line(&line);
        graph.insert_edge_or_increase_weight(&from, &to, dist);
    }
    graph
}

fn parse_line(line: &str)->(String, String, i32){
    lazy_static::lazy_static! {
        static ref REG: Regex = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();
    }
    let caps = REG.captures(&line).unwrap();
    let from: String = caps.get(1).map(|m| String::from(m.as_str())).unwrap();
    let to: String = caps.get(2).map(|m| String::from(m.as_str())).unwrap();
    let dist: i32 = caps
        .get(3)
        .map(|m| i32::from_str_radix(m.as_str(), 10).unwrap())
        .unwrap();
    (from,to,dist)
}
