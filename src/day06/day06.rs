use core::iter::zip;
use std::env;
use std::fs;

fn parse_numbers(text: &str) -> Vec<u32> {
    text.split_whitespace()
        .map(|n| n.parse().expect("Expected number."))
        .collect()
}

fn distance(hold_time: u32, total_time: u32) -> u32 {
    hold_time * (total_time - hold_time)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected puzzle input.");

    let contents = fs::read_to_string(filename).expect("Could not read file.");
    let mut lines = contents.lines();

    let times = parse_numbers(&lines.next().expect("Expected times.").replace("Time: ", ""));
    let distances = parse_numbers(&lines.next().expect("Expected distances.").replace("Distance: ", ""));

    let winning_options = zip(&times, &distances)
        .map(|(t, d)| (0..*t).filter(|time| distance(*time, *t) > *d).count() as u32)
        .reduce(|acc, e| acc * e)
        .unwrap();
    println!("{winning_options}");
}
