use std::fs;

use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day11_data.txt")?;

    let (rows, columns) = generate_rows_and_columns(&input);
    let galaxy = generate_galaxy_map(&input, rows, columns)?;
    let pairs = CoordPair::generate_pairs(galaxy);
    let distances = CoordPair::generate_distance_map(&pairs);
    let distance_sum: i32 = distances.values().sum();

    print!("Part 1: {}", distance_sum);
    Ok(())
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

    fn manhattan_distance(&self) -> i32 {
        self.first.manhattan_distance(&self.second)
    }

    fn generate_distance_map(pairs: &HashSet<CoordPair>) -> HashMap<CoordPair, i32> {
        let mut distances = HashMap::new();
        for pair in pairs {
            distances.insert(*pair, pair.manhattan_distance());
        }
        distances
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Coord) -> i32 {
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
) -> Result<HashSet<Coord>> {
    let empty = '.';
    let mut row_incs = 0;
    let mut galazy_set = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        let mut column_incs = 0;
        let empty_row = rows
            .get(&row)
            .ok_or_else(|| anyhow!("{} missing row", row))?
            .iter()
            .all(|p| *p == empty);
        if empty_row {
            row_incs += 1;
            continue;
        }
        for (column, c) in line.chars().enumerate() {
            if c != '.' {
                galazy_set.insert(Coord::new(
                    row as i32 + row_incs,
                    column as i32 + column_incs,
                ));
                continue;
            }
            let empty_column = columns
                .get(&column)
                .ok_or_else(|| anyhow!("{} column not present", column))?
                .iter()
                .all(|p| *p == empty);
            if empty_column {
                column_incs += 1;
            }
        }
    }
    Ok(galazy_set)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day11_data_test.txt")?;

        // Act
        let (rows, columns) = generate_rows_and_columns(&input);
        let galaxy = generate_galaxy_map(&input, rows, columns)?;
        let pairs = CoordPair::generate_pairs(galaxy);
        let distances = CoordPair::generate_distance_map(&pairs);
        let distance_sum: i32 = distances.values().sum();

        // Assert
        assert_eq!(374, distance_sum);
        Ok(())
    }
}
