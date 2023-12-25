use std::collections::HashSet;
use std::{env, fs};

struct Card {
    nums: HashSet<u32>,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let binding = line.split(':').collect::<Vec<_>>();
        let card = binding.first().expect("No card identifier!");
        Card {
            nums: get_numbers(binding.get(1).expect("No winning numbers!")),
        }
    }
}

fn get_numbers(text: &str) -> HashSet<u32> {
    text.split_whitespace()
        .map(|n| n.parse().expect("No number in number string."))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).unwrap();

    let contents = fs::read_to_string(filename).expect("Could not read file.");

    let points: u32 = contents
        .lines()
        .map(|l| {
            let binding = l.split('|').collect::<Vec<_>>();
            let card_contents = binding.first().expect("Expected winning numbers.");
            let nums = get_numbers(binding.get(1).expect("Expected user numbers."));
            let card = Card::from_line(card_contents);

            let winners = card.nums.intersection(&nums).count();
            if winners > 0 {
                2_u32.pow(winners as u32 - 1)
            } else {
                0
            }
        })
        .sum();
    println!("{points}");
}
