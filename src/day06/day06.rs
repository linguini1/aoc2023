use core::iter::zip;
use std::env;
use std::fs;

fn parse_numbers(text: &str) -> Vec<u64> {
    text.split_whitespace()
        .map(|n| n.parse().expect("Expected number."))
        .collect()
}

fn distance(hold_time: u64, total_time: u64) -> u64 {
    hold_time * (total_time - hold_time)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected puzzle input.");

    let contents = fs::read_to_string(filename).expect("Could not read file.");
    let mut lines = contents.lines();

    let raw_times = &lines.next().expect("Expected times.").replace("Time: ", "");
    let raw_distances = &lines.next().expect("Expected distances.").replace("Distance: ", "");

    // Part A
    let times = parse_numbers(raw_times);
    let distances = parse_numbers(raw_distances);

    let winning_options = zip(&times, &distances)
        .map(|(t, d)| (0..*t).filter(|time| distance(*time, *t) > *d).count() as u64)
        .reduce(|acc, e| acc * e)
        .unwrap();

    println!("{winning_options}");

    // Part B
    let mut time_str = raw_times.clone();
    time_str.retain(|c| c.is_ascii_digit());
    let time: u64 = time_str.parse().unwrap();

    let mut distance_str = raw_distances.clone();
    distance_str.retain(|c| c.is_ascii_digit());
    let dist: u64 = distance_str.parse().unwrap();

    let option_count = (0..time).filter(|t| distance(*t, time) > dist).count() as u64;
    println!("{option_count}");
}
