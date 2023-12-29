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

fn next_step<'a>(node: &str, direction: Option<char>, graph: &'a HashMap<&'a str, (&'a str, &'a str)>) -> &'a str {
    let nodes = graph.get(node).unwrap();
    match direction {
        Some('R') => nodes.1,
        Some('L') => nodes.0,
        _ => panic!("Expected Right or Left direction."),
    }
}

fn steps_until_destination(
    start: &str,
    directions: &str,
    graph: &HashMap<&str, (&str, &str)>,
    condition: fn(&str) -> bool,
) -> u64 {
    let mut steps: u64 = 0;
    let mut direction_cycle = directions.chars().cycle();
    let mut node = start;
    while !condition(node) {
        node = next_step(node, direction_cycle.next(), graph);
        steps += 1;
    }
    steps
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        (x, y) = (y, x % y)
    }
    x
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected puzzle input file.");

    let contents = fs::read_to_string(filename).expect("Could not open file.");

    let mut lines = contents.lines();
    let directions = lines.next().expect("Expected directions.");
    lines.next().expect("Expected blank line.");

    let mut graph = HashMap::new();
    for line in lines {
        let (node, nodes) = parse_node(line);
        graph.insert(node, nodes);
    }

    // Part A
    let steps = steps_until_destination(START_NODE, directions, &graph, |n: &str| n == END_NODE);
    println!("{steps}");

    // Part B
    let start_nodes = Vec::from_iter(graph.keys().filter(|k| k.ends_with('A')).copied());
    let is_end_node = |n: &str| n.ends_with('Z'); // Test if a node is destination
    let multi_steps: Vec<_> = start_nodes
        .iter()
        .map(|n| steps_until_destination(n, directions, &graph, is_end_node))
        .collect();

    let mut cycles_collide = 1;
    for step in multi_steps {
        cycles_collide = cycles_collide * step / gcd(cycles_collide, step);
    }

    println!("{cycles_collide}");
}
