use std::collections::{HashMap, HashSet};
use std::{env, fs};

struct Card {
    id: u32,
    winning_nums: HashSet<u32>,
    user_nums: HashSet<u32>,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let binding = line.split(':').collect::<Vec<_>>();

        // Card ID
        let card = binding.first().expect("No card identifier!");
        let mut id = card.replace("Card", "");
        id.retain(|c| !c.is_whitespace());

        // Numbers
        let numbers = binding.get(1).expect("No numbers!");
        let number_sets = numbers.split('|').collect::<Vec<_>>();
        let winners = number_sets.first().expect("Expected winning numbers");
        let users = number_sets.get(1).expect("Expected user numbers");

        Card {
            id: id.parse().expect("Expected card ID."),
            winning_nums: get_numbers(winners),
            user_nums: get_numbers(users),
        }
    }

    fn next_n_cards(&self) -> u32 {
        self.winning_nums.intersection(&self.user_nums).count() as u32
    }

    fn points(&self) -> u32 {
        let winning_count = self.next_n_cards();
        if winning_count > 0 {
            2_u32.pow(winning_count - 1)
        } else {
            0
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

    // Get cards
    let cards: Vec<_> = contents.lines().map(Card::from_line).collect();

    // Part A
    let points: u32 = cards.iter().map(|c| c.points()).sum();
    println!("{points}");

    // Part B
    let mut card_counts: HashMap<u32, u32> = HashMap::new();

    // Always one card initially
    for card in &cards {
        card_counts.insert(card.id, 1);
    }

    // Go through all cards
    for card in &cards {
        // Go through all card copies
        for copy in card.id + 1..card.id + card.next_n_cards() + 1 {
            let parent_card_count = *card_counts.get(&card.id).unwrap();
            card_counts.entry(copy).and_modify(|count| *count += parent_card_count);
        }
    }

    let total_cards: u32 = card_counts.values().sum();
    println!("{total_cards}");
}
