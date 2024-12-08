use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day8_data.txt")?;

    let antinodes = find_antinodes(&input);

    println!("Part 1: {}", antinodes);

    let antinodes = find_all_antinodes(&input);

    println!("Part 2: {}", antinodes);

    Ok(())
}

fn find_all_antinodes(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut antinodes = HashSet::new();

    for coords in grid.map.values() {
        let pairs = find_pairs(coords);

        for x in 0..=grid.x_max {
            for y in 0..=grid.y_max {
                for pair in &pairs {
                    let dfx = pair.first.0 - x;
                    let dfy = pair.first.1 - y;
                    let dsx = pair.second.0 - x;
                    let dsy = pair.second.1 - y;

                    // dfx / dfy == dsx / dsy
                    if dfx * dsy == dsx * dfy {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn find_antinodes(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut antinodes = HashSet::new();

    for coords in grid.map.values() {
        let pairs = find_pairs(coords);
        for pair in pairs {
            let first = pair.first;
            let second = pair.second;
            assert!(first != second);
            let projection = (second.0 - first.0, second.1 - first.1);
            let shifts = [
                (first.0 + projection.0, first.1 + projection.1),
                (first.0 - projection.0, first.1 - projection.1),
                (second.0 + projection.0, second.1 + projection.1),
                (second.0 - projection.0, second.1 - projection.1),
            ];
            let mut added = 0;
            for shift in shifts {
                if shift != first
                    && shift != second
                    && shift.0 >= 0
                    && shift.1 >= 0
                    && shift.0 <= grid.x_max
                    && shift.1 <= grid.y_max
                {
                    antinodes.insert(shift);
                    added += 1;
                }
            }
            assert!(added <= 2);
        }
    }

    antinodes.len()
}

struct Pair {
    first: (i32, i32),
    second: (i32, i32),
}
fn find_pairs(coords: &[(i32, i32)]) -> Vec<Pair> {
    let mut pairs = vec![];
    for (x, i) in coords.iter().enumerate() {
        for j in coords.iter().skip(x + 1) {
            pairs.push(Pair {
                first: *i,
                second: *j,
            });
        }
    }
    pairs
}

struct Grid {
    map: HashMap<char, Vec<(i32, i32)>>,
    x_max: i32,
    y_max: i32,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        let mut y_max = 0;
        let mut x_max = 0;
        for (y, line) in input.lines().enumerate() {
            y_max = y as i32;
            for (x, c) in line.chars().enumerate() {
                x_max = y as i32;
                if c == '.' {
                    continue;
                }
                map.entry(c)
                    .and_modify(|v| v.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        }
        Self { map, x_max, y_max }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day8_test_data.txt")?;

        // Act
        let antinodes = find_antinodes(&input);

        // Assert
        assert_eq!(antinodes, 14);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day8_test_data.txt")?;

        // Act
        let antinodes = find_all_antinodes(&input);

        // Assert
        assert_eq!(antinodes, 34);
        Ok(())
    }
}
