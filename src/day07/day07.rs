use std::cmp::{Eq, Ordering};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::zip;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    Joker,
    Number(u8),
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
        let joker_count = *card_counts.get(&Card::Joker).unwrap_or(&0);
        match card_counts.keys().len() {
            1 => HandType::FiveOfAKind,
            2 => {
                // Early return five of a kind
                if joker_count > 0 {
                    return HandType::FiveOfAKind;
                }

                // Otherwise normal logic
                if *card_counts.values().max().unwrap() == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if *card_counts.values().max().unwrap() == 3 {
                    if joker_count > 0 {
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else if joker_count == 2 {
                    HandType::FourOfAKind
                } else if joker_count == 1 {
                    HandType::FullHouse
                } else {
                    HandType::TwoPair
                }
            }
            4 => {
                if joker_count > 0 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            5 => {
                if joker_count > 0 {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
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
                    return (*c1).cmp(c2);
                }
            }
            Ordering::Equal
        } else {
            self.hand_type().cmp(&other.hand_type())
        }
    }
}

fn char_to_card(c: char, wildcards: bool) -> Card {
    match c {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => {
            if wildcards {
                Card::Joker
            } else {
                Card::Jack
            }
        }
        'T' => Card::Number(10),
        _ => Card::Number(c.to_digit(10).expect("Expected digit.").try_into().unwrap()),
    }
}

fn calc_winnings(hands: &[Hand]) -> u32 {
    hands.iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bid).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected a filename.");

    let contents = fs::read_to_string(filename).expect("Could not read file.");

    // Part A
    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').expect("Expected hand and bid!");
            Hand::new(
                bid.parse().unwrap(),
                cards.chars().map(|c| char_to_card(c, false)).collect(),
            )
        })
        .collect();
    hands.sort();

    let winnings: u32 = calc_winnings(&hands);
    println!("{winnings}");

    // Part B
    let mut wild_card_hands: Vec<Hand> = contents
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').expect("Expected hand and bid!");
            Hand::new(
                bid.parse().unwrap(),
                cards.chars().map(|c| char_to_card(c, true)).collect(),
            )
        })
        .collect();
    wild_card_hands.sort();

    let wild_card_winnings = calc_winnings(&wild_card_hands);
    println!("{wild_card_winnings}");
}
