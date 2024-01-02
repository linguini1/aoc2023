use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs;

const DIRECTIONS: [(Direction, (i32, i32)); 4] = [
    (Direction::North, (-1, 0)),
    (Direction::South, (1, 0)),
    (Direction::East, (0, 1)),
    (Direction::West, (0, -1)),
];

type Coords = (u32, u32);

#[derive(PartialEq, Eq, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ground,
            'S' => Self::Start,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            _ => panic!("Unknown pipe type {value}!"),
        }
    }
}

impl Pipe {
    fn ends(&self) -> Vec<Direction> {
        match self {
            Self::Ground => vec![],
            Self::Start => vec![Direction::North, Direction::South, Direction::East, Direction::West],
            Self::Vertical => vec![Direction::North, Direction::South],
            Self::Horizontal => vec![Direction::East, Direction::West],
            Self::NorthEast => vec![Direction::North, Direction::East],
            Self::NorthWest => vec![Direction::North, Direction::West],
            Self::SouthWest => vec![Direction::South, Direction::West],
            Self::SouthEast => vec![Direction::South, Direction::East],
        }
    }

    fn connects(&self, other: &Self, position: Direction) -> bool {
        for end in self.ends() {
            if other.ends().contains(&end.opposite()) && position == end {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Connection {
    direction: Direction,
    loc: Coords,
}

impl Connection {
    fn new(direction: Direction, loc: Coords) -> Self {
        Connection { direction, loc }
    }
}

type Connections = Vec<Connection>;

fn translate(coord: Coords, vector: (i32, i32), bounds: Coords) -> Option<Coords> {
    if coord.0 == 0 && vector.0 < 0
        || coord.1 == 0 && vector.1 < 0
        || coord.0 == bounds.0 && vector.0 > 0
        || coord.1 == bounds.1 && vector.1 > 0
    {
        None
    } else {
        Some((
            coord.0.checked_add_signed(vector.0).expect("Overflow occurred."),
            coord.1.checked_add_signed(vector.1).expect("Overlow occurred."),
        ))
    }
}

fn loop_length(graph: &HashMap<Coords, Connections>, start: &Coords) -> u32 {
    let mut counter = 1;
    let mut prev_pos = *start;
    let mut cur_pos = graph.get(start).unwrap().first().unwrap().loc;

    loop {
        for pos in graph.get(&cur_pos).unwrap() {
            if pos.loc != prev_pos {
                prev_pos = cur_pos;
                cur_pos = pos.loc;
                break;
            }
        }

        if cur_pos == *start {
            break;
        }
        counter += 1;
    }
    counter
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected puzzle input file.");

    let contents = fs::read_to_string(filename).expect("Could not read file.");

    let mut counter = contents.lines();
    let bounds = (
        counter.next().expect("Expected at least one line.").len() as u32,
        counter.count() as u32 + 1,
    );

    let mut grid: HashMap<Coords, Pipe> = HashMap::new();
    let mut start = (0, 0);
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid.insert((i as u32, j as u32), c.into());
            if Pipe::from(c) == Pipe::Start {
                start = (i as u32, j as u32)
            }
        }
    }

    // Build graph of pipes which connect
    let mut graph: HashMap<Coords, Connections> = HashMap::new();
    for (loc, pipe) in &grid {
        for (direction, vector) in &DIRECTIONS {
            if let Some(new_coords) = translate(*loc, *vector, bounds) {
                if let Some(p) = grid.get(&new_coords) {
                    if pipe.connects(p, direction.clone()) {
                        graph
                            .entry(*loc)
                            .and_modify(|c| c.push(Connection::new(direction.clone(), new_coords)))
                            .or_insert(vec![Connection::new(direction.clone(), new_coords)]);
                    }
                }
            }
        }
    }

    // Seems like the start node will only have two valid connections, so a position on the loop is
    // always known
    //
    // It also seems that each node on the loop has two connections: where you came from and where
    // you can go. There are no "sub-loops". So you can traverse the graph until you reach the
    // start node again and label each spot with a distance value, and then take the maximum
    //
    // Loops have a unique property such that the furthest point on the loop from any other point
    // is equal to the loop length divided by two

    // Part A
    let length = loop_length(&graph, &start);
    if length % 2 == 0 {
        println!("{}", length / 2);
    } else {
        println!("{}", length / 2 + 1);
    }
}
