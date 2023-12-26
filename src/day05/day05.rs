use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Range;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum MapType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl MapType {
    fn from(string: &str) -> Self {
        match string {
            "seed" => MapType::Seed,
            "soil" => MapType::Soil,
            "fertilizer" => MapType::Fertilizer,
            "water" => MapType::Water,
            "light" => MapType::Light,
            "temperature" => MapType::Temperature,
            "humidity" => MapType::Humidity,
            "location" => MapType::Location,
            _ => panic!("Unknown map type!"),
        }
    }
}

#[derive(Debug)]
struct Map {
    from: MapType,
    to: MapType,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn map(&self, origin: u64) -> (MapType, u64) {
        for range in &self.ranges {
            if range.1.contains(&origin) {
                let difference = origin - range.1.start;
                return (self.to.clone(), difference + range.0.start);
            }
        }

        (self.to.clone(), origin)
    }
}

fn map_until_location(maps: &HashMap<MapType, Map>, seed: &u64) -> u64 {
    let mut cur_type = MapType::Seed;
    let mut value = *seed;
    while cur_type != MapType::Location {
        let map = maps.get(&cur_type).unwrap();
        let result = map.map(value);
        cur_type = result.0;
        value = result.1;
    }
    value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let filename = args.get(1).expect("Expected passed filename.");

    let contents = fs::read_to_string(filename).expect("Could not open file.");

    // Parse seeds
    let seeds: Vec<u64> = contents
        .lines()
        .next()
        .expect("No seeds.") // Get seed line
        .split(": ")
        .collect::<Vec<_>>()
        .get(1) // Get space separated seeds
        .unwrap()
        .split(' ') // Split seed numbers
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // Parse maps
    let map_label = Regex::new("([a-z]+)-to-([a-z]+) map:").unwrap();
    let mut maps: HashMap<MapType, Map> = HashMap::new();
    let mut current_map: Option<MapType> = None;
    for line in contents.lines() {
        // Skip blank lines
        if line.is_empty() {
            continue;
        }

        // Parse map label
        if map_label.is_match(line) {
            let labels = map_label.captures(line).unwrap().extract::<2>().1;
            let from = MapType::from(labels[0]);
            maps.insert(
                from.clone(),
                Map {
                    from: from.clone(),
                    to: MapType::from(labels[1]),
                    ranges: vec![],
                },
            );
            current_map = Some(from.clone());
        }
        // Add the number lists to the current map
        else if let Some(ref maptype) = current_map {
            let ranges: Vec<u64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            let from = ranges.first().unwrap();
            let to = ranges.get(1).unwrap();
            let amount = ranges.get(2).unwrap();

            maps.entry(maptype.clone())
                .and_modify(|m| m.ranges.push((*from..*from + *amount, *to..*to + *amount)));
        }
    }

    let locations: Vec<u64> = seeds.iter().map(|seed| map_until_location(&maps, seed)).collect();
    println!("{}", locations.iter().min().expect("No locations found."));

    // Part B
    let min_location: u64 = seeds
        .windows(2)
        .enumerate()
        .filter(|(i, _)| i % 2 == 0) // Skips windows with overlaps to just get unique pairs
        .map(|(_, slice)| {
            // Gets minimum location of each seed range
            (slice[0]..slice[0] + slice[1])
                .map(|seed| map_until_location(&maps, &seed))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    println!("{min_location}");
}
