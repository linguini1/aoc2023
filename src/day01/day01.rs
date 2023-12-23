use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2); // Only one argument: file name of input puzzle
    let filename = &args[1];

    // Get file contents
    let contents = fs::read_to_string(filename).expect("Could not read file.");

    let mut sum = 0;
    for line in contents.lines() {
        let numbers: Vec<u32> = line
            .chars()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x.to_digit(10).expect("Non-digits should have been filtered."))
            .collect();
        let calibration =
            numbers.first().expect("Expected one number.") * 10 + numbers.last().expect("Expected second number.");
        sum += calibration;
    }
    println!("{}", sum);
}
