use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day11_data.txt")?;

    let part_1 = find_distances_sum(&input, 2)?;
    println!("Part 1: {}", part_1);

    let part_2 = find_distances_sum(&input, 1_000_000)?;
    println!("Part 2: {}", part_2);

    // let shortest = find_shortest(&input)?;
    // println!("Shortest path: {}", shortest);

    Ok(())
}

fn find_distances(input: &str, larger_by: i64) -> Result<HashMap<CoordPair, i64>> {
    let (rows, columns) = generate_rows_and_columns(input);
    let galaxy = generate_galaxy_map(input, rows, columns, larger_by)?;
    let pairs = CoordPair::generate_pairs(galaxy);
    let distances = CoordPair::generate_distance_map(&pairs);
    Ok(distances)
}

fn find_distances_sum(input: &str, larger_by: i64) -> Result<i64> {
    let distances = find_distances(input, larger_by)?;
    let distance_sum: i64 = distances.values().sum();
    Ok(distance_sum)
}

#[allow(dead_code)]
fn find_shortest(input: &str) -> Result<i64> {
    let distances = find_distances(input, 1)?;
    find_shortest_distance(&distances)
}

fn find_shortest_distance(distances: &HashMap<CoordPair, i64>) -> Result<i64> {
    let mut coords = HashSet::new();
    let mut lookup: HashMap<Coord, HashMap<Coord, i64>> = HashMap::new();
    for (k, v) in distances {
        let f = lookup.entry(k.first).or_insert(HashMap::new());
        f.insert(k.second, *v);
        let s = lookup.entry(k.second).or_insert(HashMap::new());
        s.insert(k.first, *v);
        coords.insert(k.first);
        coords.insert(k.second);
    }

    let mut min = i64::MAX;
    let mut cache = BTreeMap::new();
    for n in &coords {
        let mut clone = coords.clone();
        clone.remove(n);
        let state = State {
            current: *n,
            missing: clone,
            steps: 0,
            current_min: min,
        };
        let path = dfs(state, &lookup, &mut cache)?;
        if path < min {
            min = path;
        }
    }

    Ok(min)
}

struct State {
    current: Coord,
    missing: HashSet<Coord>,
    steps: i64,
    current_min: i64,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct CacheState {
    current: Coord,
    missing: BTreeSet<Coord>,
}

impl CacheState {
    fn from(state: &State) -> Self {
        let missing: BTreeSet<Coord> = state.missing.iter().copied().collect();
        Self {
            current: state.current,
            missing,
        }
    }
}

impl State {
    fn update(&self, next: Coord, dist: i64, current_min: i64) -> Self {
        let mut new_missing = self.missing.clone();
        new_missing.remove(&next);
        Self {
            current: next,
            missing: new_missing,
            steps: self.steps + dist,
            current_min,
        }
    }
}

fn dfs(
    state: State,
    lookup: &HashMap<Coord, HashMap<Coord, i64>>,
    cache: &mut BTreeMap<CacheState, i64>,
) -> Result<i64> {
    let mut current_min = state.current_min;

    for next in &state.missing {
        let dist = lookup
            .get(&state.current)
            .ok_or(anyhow!("{:?} not in from", state.current))?
            .get(next)
            .ok_or(anyhow!("{:?} not in to", next))?;
        let total_dist = state.steps + dist;
        // Check if next length above min
        if total_dist >= current_min {
            continue;
        }
        // Validate below and final
        if state.missing.len() == 1 && state.steps < total_dist {
            return Ok(total_dist);
        }
        // Check cache
        let state_updated = state.update(*next, *dist, current_min);
        let cache_state = CacheState::from(&state_updated);
        let cache_entry = cache.entry(cache_state).or_insert(total_dist);

        if total_dist > *cache_entry {
            continue;
        }
        *cache_entry = total_dist;

        let path_min = dfs(state_updated, lookup, cache)?;
        if path_min < current_min {
            current_min = path_min;
        }
    }
    Ok(current_min)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct CoordPair {
    first: Coord,
    second: Coord,
}

impl CoordPair {
    fn generate_pairs(galaxy_set: HashSet<Coord>) -> HashSet<CoordPair> {
        let mut pairs = HashSet::new();
        for first in &galaxy_set {
            for second in &galaxy_set {
                if first == second {
                    continue;
                }
                pairs.insert(CoordPair::new(*first, *second));
            }
        }
        pairs
    }

    fn new(one: Coord, two: Coord) -> Self {
        if one.x < two.x {
            return Self {
                first: one,
                second: two,
            };
        }
        if one.x > two.x {
            return Self {
                first: two,
                second: one,
            };
        }
        if one.y < two.y {
            return Self {
                first: one,
                second: two,
            };
        }
        Self {
            first: two,
            second: one,
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.first.manhattan_distance(&self.second)
    }

    fn generate_distance_map(pairs: &HashSet<CoordPair>) -> HashMap<CoordPair, i64> {
        let mut distances = HashMap::new();
        for pair in pairs {
            distances.insert(*pair, pair.manhattan_distance());
        }
        distances
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Coord) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn generate_rows_and_columns(
    input: &str,
) -> (HashMap<usize, Vec<char>>, HashMap<usize, Vec<char>>) {
    let mut rows = HashMap::new();
    let mut columns = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            rows.entry(row)
                .and_modify(|v: &mut Vec<char>| v.push(c))
                .or_insert(Vec::from([c]));
            columns
                .entry(column)
                .and_modify(|v: &mut Vec<char>| v.push(c))
                .or_insert(Vec::from([c]));
        }
    }
    (rows, columns)
}

fn generate_galaxy_map(
    input: &str,
    rows: HashMap<usize, Vec<char>>,
    columns: HashMap<usize, Vec<char>>,
    mut larger_by: i64,
) -> Result<HashSet<Coord>> {
    let empty = '.';
    let mut row_incs = 0;
    let mut galazy_set = HashSet::new();
    larger_by -= 1;
    for (row, line) in input.lines().enumerate() {
        let mut column_incs = 0;
        let empty_row = rows
            .get(&row)
            .ok_or_else(|| anyhow!("{} missing row", row))?
            .iter()
            .all(|p| *p == empty);
        if empty_row {
            row_incs += larger_by;
            continue;
        }
        for (column, c) in line.chars().enumerate() {
            if c != '.' {
                galazy_set.insert(Coord::new(
                    row as i64 + row_incs,
                    column as i64 + column_incs,
                ));
                continue;
            }
            let empty_column = columns
                .get(&column)
                .ok_or_else(|| anyhow!("{} column not present", column))?
                .iter()
                .all(|p| *p == empty);
            if empty_column {
                column_incs += larger_by;
            }
        }
    }
    Ok(galazy_set)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_shortest_path() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day11_data_test.txt")?;

        // Act
        let actual = find_shortest(&input)?;

        // Assert
        assert_eq!(35, actual);
        Ok(())
    }

    #[test]
    fn test_solutions() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day11_data_test.txt")?;

        for (expected, inc) in vec![(374, 2), (1030, 10), (8410, 100)] {
            // Act
            let actual = find_distances_sum(&input, inc)?;

            // Assert
            assert_eq!(expected, actual);
        }
        Ok(())
    }
}
