use std::collections::HashMap;
use std::env;
use std::fs;
use std::u32;

fn get_calibration(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|x| x.to_digit(10)).collect();
    if digits.is_empty() {
        return 0;
    }
    return digits.first().unwrap() * 10 + digits.last().unwrap();
}

fn main() {
    // Compose mapping from words to numerical values
    let digit_words: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "th3ee"),
        ("four", "f4ur"),
        ("five", "f5ve"),
        ("six", "s6x"),
        ("seven", "se7en"),
        ("eight", "ei8ht"),
        ("nine", "n9ne"),
    ]);

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2); // Only one argument: file name of input puzzle
    let filename = &args[1];

    // Get file contents
    let contents = fs::read_to_string(filename).expect("Could not read file.");

    // Part A
    let sum1: u32 = contents.lines().map(get_calibration).sum();
    println!("{}", sum1);

    // Part B

    // Replace digit words with real digits
    let mut new_contents = contents;
    for (digit, value) in digit_words.iter() {
        new_contents = str::replace(&new_contents, digit, value);
    }
    let sum2: u32 = new_contents.lines().map(get_calibration).sum();
    println!("{}", sum2)
}
