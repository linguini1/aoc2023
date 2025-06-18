use std::env;
use std::fs;

/// Represents a galaxy in the puzzle input
#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    /// Calculates the Manhattan distance between two galaxies
    fn manhattan_distance(&self, other: &Galaxy) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

/// Takes a list of galaxies and maps them to an expanded galaxy list
fn expanded_galaxies(
    galaxies: &[Galaxy],
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_factor: usize,
) -> Vec<Galaxy> {
    assert!(expansion_factor > 1);
    galaxies
        .iter()
        .map(|g| Galaxy {
            y: g.y + empty_rows.iter().filter(|r| g.y > **r).count() * (expansion_factor - 1),
            x: g.x + empty_cols.iter().filter(|c| g.x > **c).count() * (expansion_factor - 1),
        })
        .collect()
}

/// Returns a list of the Manhattan distances between each pair of galaxies
fn galaxy_distances(galaxies: &[Galaxy]) -> Vec<usize> {
    let mut distances: Vec<usize> = Vec::new();
    for i in 0..galaxies.len() {
        let g1 = &galaxies[i];
        for g2 in galaxies.iter().skip(i + 1) {
            distances.push(g1.manhattan_distance(g2));
        }
    }
    distances
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected puzzle input file.");
    let contents = fs::read_to_string(filename).expect("Could not read file.");

    let n_rows = contents.lines().count();
    let n_cols = contents.lines().next().expect("At least one line").chars().count();

    // Parse galaxies from the puzzle input
    let galaxies: Vec<_> = contents
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Galaxy { x, y })
        })
        .collect();

    // Find the rows and columns which are empty (due to the lack of galaxies)
    let empty_rows: Vec<_> = (0..n_rows)
        .filter(|r| galaxies.iter().map(|g| g.y).filter(|y| y == r).count() == 0)
        .collect();

    let empty_cols: Vec<_> = (0..n_cols)
        .filter(|c| galaxies.iter().map(|g| g.x).filter(|x| x == c).count() == 0)
        .collect();

    // Any galaxy with a row index (y) greater than an empty row's index will have its row index (y)
    // incremented by one. Any galaxy with a column index (x) greater than an empty column's index
    // will have its column index (x) incremented by one.

    let doubled_galaxy = expanded_galaxies(&galaxies, &empty_rows, &empty_cols, 2);

    // Now calculate the shortest distance between each pair of galaxies

    let double_distances = galaxy_distances(&doubled_galaxy);

    println!("{}", double_distances.iter().sum::<usize>());

    // For part two we do the same, but use a bigger expansion factor

    let large_galaxy = expanded_galaxies(&galaxies, &empty_rows, &empty_cols, 1000000);
    let large_distances = galaxy_distances(&large_galaxy);

    println!("{}", large_distances.iter().sum::<usize>());
}
