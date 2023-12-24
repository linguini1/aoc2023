use regex::Regex;
use std::cmp;
use std::env;
use std::fs;

const TARGET_SET: (u32, u32, u32) = (12, 13, 14);

enum Block {
    Red(u32),
    Green(u32),
    Blue(u32),
}

struct Game {
    id: u32,
    maxes: (u32, u32, u32),
}

impl Game {
    fn new(id: u32) -> Game {
        Game { id, maxes: (0, 0, 0) }
    }

    fn update_max(&mut self, count: Block) {
        match count {
            Block::Red(n) => self.maxes = (cmp::max(n, self.maxes.0), self.maxes.1, self.maxes.2),
            Block::Green(n) => self.maxes = (self.maxes.0, cmp::max(n, self.maxes.1), self.maxes.2),
            Block::Blue(n) => self.maxes = (self.maxes.0, self.maxes.1, cmp::max(n, self.maxes.2)),
        }
    }

    fn possible(&self) -> bool {
        self.maxes.0 <= TARGET_SET.0 && self.maxes.1 <= TARGET_SET.1 && self.maxes.2 <= TARGET_SET.2
    }

    fn power(&self) -> u32 {
        self.maxes.0 * self.maxes.1 * self.maxes.2
    }
}

fn colour_count(set: &str) -> (Block, Block, Block) {
    let red: Regex = Regex::new("(\\d+) red").unwrap();
    let green: Regex = Regex::new("(\\d+) green").unwrap();
    let blue: Regex = Regex::new("(\\d+) blue").unwrap();

    let mut counts: [u32; 3] = [0, 0, 0];
    for (i, pattern) in [red, green, blue].iter().enumerate() {
        if let Some(result) = pattern.captures(set) {
            counts[i] = result.get(1).unwrap().as_str().parse().unwrap();
        } else {
            counts[i] = 0;
        }
    }
    (Block::Red(counts[0]), Block::Green(counts[1]), Block::Blue(counts[2]))
}

fn main() {
    // Get filename
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2); // Only a filename
    let filename = &args[1];

    // Read file
    let contents = fs::read_to_string(filename).expect("Could not read file contents.");

    // Patterns
    let game_id = Regex::new("Game (\\d+):").unwrap();

    // Parse into games
    let games: Vec<_> = contents
        .lines()
        .map(|line| -> Game {
            let mut game = Game::new(
                game_id
                    .captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            );

            for set in line.split(';') {
                let (r, g, b) = colour_count(set);
                game.update_max(r);
                game.update_max(g);
                game.update_max(b);
            }

            game
        })
        .collect();

    // Part A
    let total: u32 = games.iter().filter(|g| g.possible()).map(|g| g.id).sum();
    println!("{total}");

    // Part B
    let power_sum: u32 = games.iter().map(|g| g.power()).sum();
    println!("{power_sum}")
}
