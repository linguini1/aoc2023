use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

fn parse_node(line: &str) -> (&str, (&str, &str)) {
    let pattern = Regex::new("([A-Z]{3})").unwrap();
    let captures: Vec<_> = pattern.find_iter(line).collect();
    let name = captures.get(0).expect("Expected node name.");
    let left = captures.get(1).expect("Expected left node.");
    let right = captures.get(2).expect("Expected right node.");
    (name.as_str(), (left.as_str(), right.as_str()))
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected puzzle input file.");

    let contents = fs::read_to_string(filename).expect("Could not open file.");

    let mut lines = contents.lines();
    let mut directions = lines.next().expect("Expected directions.").chars().cycle();
    lines.next().expect("Expected blank line.");

    let mut graph = HashMap::new();
    for line in lines {
        let (node, nodes) = parse_node(line);
        graph.insert(node, nodes);
    }

    let mut node = START_NODE;
    let mut steps = 0;
    while node != END_NODE {
        let nodes = graph.get(node).unwrap();
        match directions.next() {
            Some('R') => node = nodes.1,
            Some('L') => node = nodes.0,
            _ => panic!("Expected Right or Left direction."),
        }
        steps += 1;
    }
    println!("{steps}");
}
