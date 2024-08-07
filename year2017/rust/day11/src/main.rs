use anyhow::{anyhow, Result};
use std::{cmp::Ordering, collections::BTreeSet, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day11_data.txt")?;

    let mut visited: BTreeSet<Coordinate> = BTreeSet::new();

    let final_coord = input
        .split(',')
        .try_fold(Coordinate::init(0, 0), |acc, dir| {
            let updated = acc.movement(dir.trim())?;
            visited.insert(updated.clone());
            Ok::<Coordinate, anyhow::Error>(updated)
        })?;

    let furthest = visited
        .iter()
        .max()
        .ok_or_else(|| anyhow!("not able to find max"))?;

    println!("Part 1: {}", final_coord.manhattan());
    println!("Part 2: {}", furthest.manhattan());
    Ok(())
}

#[derive(Clone, PartialEq, Eq)]
struct Coordinate {
    col: i32,
    row: i32,
}

impl Coordinate {
    fn init(col: i32, row: i32) -> Self {
        Self { col, row }
    }

    fn movement(&self, direction: &str) -> Result<Coordinate> {
        let updated = match direction {
            "ne" => Coordinate::init(self.col + 1, self.row),
            "se" => Coordinate::init(self.col + 1, self.row - 1),
            "s" => Coordinate::init(self.col, self.row - 1),
            "sw" => Coordinate::init(self.col - 1, self.row),
            "nw" => Coordinate::init(self.col - 1, self.row + 1),
            "n" => Coordinate::init(self.col, self.row + 1),
            _ => return Err(anyhow!("{} not known", direction)),
        };

        Ok(updated)
    }

    fn manhattan(&self) -> u32 {
        if (self.col ^ self.row) >= 0 {
            (self.col.abs() + self.row.abs()) as u32
        } else {
            std::cmp::max(self.col.abs(), self.row.abs()) as u32
        }
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.manhattan().cmp(&other.manhattan())
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coordinate_movements() {
        let test_cases = vec![
            (vec!["ne", "ne", "ne"], 3),
            (vec!["ne", "ne", "sw", "sw"], 0),
            (vec!["ne", "ne", "s", "s"], 2),
            (vec!["se", "sw", "se", "sw", "sw"], 3),
        ];

        for (directions, expected_distance) in test_cases {
            let mut coord = Coordinate::init(0, 0);
            for dir in directions {
                coord = coord.movement(dir).unwrap();
            }
            let actual_distance = coord.manhattan();
            assert_eq!(actual_distance, expected_distance);
        }
    }
}
