use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected puzzle input file.");
    let contents = fs::read_to_string(filename).expect("Could not read file.");
    println!("{contents}");
}
