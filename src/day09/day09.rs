use std::env;
use std::fs;

#[derive(Debug)]
struct Readings {
    raw: Vec<i64>,
}

impl Readings {
    fn next_val(&self) -> i64 {
        let mut finals: Vec<i64> = vec![];
        let mut working_set: Vec<i64> = self.raw.to_vec();
        while !working_set.iter().all(|n| *n == 0) {
            finals.push(*working_set.last().expect("Expected at least one digit."));
            working_set = working_set.windows(2).map(|w| w[1] - w[0]).collect();
        }
        finals.iter().sum()
    }

    fn prev_val(&self) -> i64 {
        let mut prevs: Vec<i64> = vec![];
        let mut working_set: Vec<i64> = self.raw.to_vec();
        while !working_set.iter().all(|n| *n == 0) {
            prevs.push(*working_set.first().expect("Expected at least one digit."));
            working_set = working_set.windows(2).map(|w| w[1] - w[0]).collect();
        }
        prevs.push(0);
        prevs.iter().rev().copied().reduce(|acc, e| e - acc).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected filename.");

    let contents = fs::read_to_string(filename).expect("Could not read file.");

    let readings: Vec<Readings> = contents
        .lines()
        .map(|l| Readings {
            raw: l.split(' ').map(|n| n.parse().expect("Expected number.")).collect(),
        })
        .collect();

    // Part A
    let next_value_sum: i64 = readings.iter().map(|r| r.next_val()).sum();
    println!("{next_value_sum}");

    // Part B
    let prev_value_sum: i64 = readings.iter().map(|r| r.prev_val()).sum();
    println!("{prev_value_sum}");
}
