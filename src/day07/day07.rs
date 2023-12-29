use std::cmp::{Eq, Ordering};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::zip;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Number(u8),
    TCard,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(bid: u32, cards: Vec<Card>) -> Self {
        Self { bid, cards }
    }

    fn hand_type(&self) -> HandType {
        let mut card_counts = HashMap::new();
        for card in &self.cards {
            card_counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }
        match card_counts.keys().len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if *card_counts.values().max().unwrap() == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if *card_counts.values().max().unwrap() == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Too many cards in hand!"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type() == other.hand_type() {
            for (c1, c2) in zip(&self.cards, &other.cards) {
                if c1 == c2 {
                    continue;
                } else {
                    return c1.cmp(c2);
                }
            }
        }
        self.hand_type().cmp(&other.hand_type())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected a filename.");

    let contents = fs::read_to_string(filename).expect("Could not read file.");

    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').expect("Expected hand and bid!");
            Hand::new(
                bid.parse().unwrap(),
                cards
                    .chars()
                    .map(|c| match c {
                        'A' => Card::Ace,
                        'K' => Card::King,
                        'Q' => Card::Queen,
                        'J' => Card::Jack,
                        'T' => Card::TCard,
                        _ => Card::Number(c.to_digit(10).expect("Expected digit.").try_into().unwrap()),
                    })
                    .collect(),
            )
        })
        .collect();
    hands.sort();

    let winnings: u32 = hands.iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bid).sum();

    println!("{winnings}");
}
