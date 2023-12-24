use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

const SURROUNDING: [(i32, i32); 8] = [(-1, 1), (-1, 0), (-1, -1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

struct Number {
    value: u32,
    start: Coords,
    end: Coords,
}

type Coords = (u32, u32);

impl Number {
    fn contains(&self, coords: Coords) -> bool {
        let rows = self.start.0..self.end.0 + 1;
        let cols = self.start.1..self.end.1;
        rows.contains(&coords.0) && cols.contains(&coords.1)
    }

    fn surrounding(&self, bounds: Coords) -> HashSet<Coords> {
        let mut cells = HashSet::new();
        for col in self.start.1..self.end.1 {
            // Check all surrounding
            for vector in SURROUNDING {
                if let Some(translated) = translate((self.start.0, col), vector, bounds) {
                    if !self.contains(translated) {
                        cells.insert(translated);
                    }
                }
            }
        }
        cells
    }
}

fn translate(coords: Coords, translation: (i32, i32), bounds: Coords) -> Option<Coords> {
    if (coords.0 == 0 && translation.0 < 0)
        || (coords.0 == bounds.0 && translation.0 > 0)
        || (coords.1 == 0 && translation.1 < 0)
        || (coords.1 == bounds.1 && translation.1 > 0)
    {
        return Option::None;
    }
    Option::Some((
        coords.0.wrapping_add_signed(translation.0),
        coords.1.wrapping_add_signed(translation.1),
    ))
}

fn main() {
    // Get input
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).unwrap();

    // Read file
    let contents = fs::read_to_string(filename).expect("File could not be read.");

    // Get symbol locations
    let cols: u32 = contents.find('\n').unwrap() as u32 + 1;
    let rows: u32 = contents.len() as u32 / cols;
    let symbols: Vec<Coords> = contents
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != '.' && !c.is_ascii_digit() && !c.is_whitespace())
        .map(|(i, _)| (i as u32 / cols, i as u32 % cols))
        .collect();

    // Get number locations
    let numeric = Regex::new("\\d+").unwrap();
    let numbers: Vec<_> = numeric
        .captures_iter(&contents)
        .map(|c| {
            let re_match = c.get(0).unwrap();
            Number {
                value: re_match.as_str().parse::<u32>().expect("Expected digit."),
                start: (re_match.start() as u32 / cols, re_match.start() as u32 % cols),
                end: (re_match.end() as u32 / cols, re_match.end() as u32 % cols),
            }
        })
        .collect();

    let total: u32 = numbers
        .iter()
        .filter(|n| -> bool {
            let surrounding = n.surrounding((rows, cols));
            for s in &symbols {
                if surrounding.contains(s) {
                    return true;
                }
            }
            false
        })
        .map(|n| n.value)
        .sum();

    println!("{total}");
}
