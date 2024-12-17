use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day16_data.txt")?;

    let maze = Maze::from_str(&input)?;
    let score = maze.find_end();

    println!("Part 1: {}", score);

    Ok(())
}

use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use anyhow::anyhow;

struct Maze {
    grid: Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
}

impl Maze {
    fn find_end(&self) -> u32 {
        let mut queue = BTreeMap::from([(0, vec![((self.start, (1, 0)))])]);
        let mut seen = HashSet::new();
        while let Some((cost, positions)) = queue.pop_first() {
            for (position, direction) in positions {
                if position == self.end {
                    return cost;
                }
                if !seen.insert((position, direction)) {
                    continue;
                }
                if self.grid[position.1 as usize][position.0 as usize] == '#' {
                    continue;
                }
                let forward = Maze::forward(position, direction);
                queue
                    .entry(cost + 1)
                    .and_modify(|v| v.push((forward, direction)))
                    .or_insert_with(|| vec![(forward, direction)]);

                for turn in [Maze::rotate_left(direction), Maze::rotate_right(direction)] {
                    queue
                        .entry(cost + 1_000)
                        .and_modify(|v| v.push((position, turn)))
                        .or_insert_with(|| vec![(position, turn)]);
                }
            }
        }
        panic!("end not found");
    }

    fn forward(position: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
        (position.0 + direction.0, position.1 + direction.1)
    }

    fn rotate_right(direction: (i32, i32)) -> (i32, i32) {
        (-direction.1, direction.0)
    }

    fn rotate_left(direction: (i32, i32)) -> (i32, i32) {
        (direction.1, -direction.0)
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut grid = vec![];
        let mut end = None;
        let mut start = None;
        for (y, line) in s.lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                let e = match c {
                    'E' => {
                        end = Some((x as i32, y as i32));
                        '.'
                    }
                    'S' => {
                        start = Some((x as i32, y as i32));
                        '.'
                    }
                    _ => c,
                };
                row.push(e);
            }
            grid.push(row);
        }
        let s = start.ok_or_else(|| anyhow!("start missing"))?;
        let e = end.ok_or_else(|| anyhow!("end missing"))?;
        Ok(Maze {
            grid,
            start: s,
            end: e,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day16_data.txt")?;

        // Act
        let maze = Maze::from_str(&input)?;
        let score = maze.find_end();

        // Assert
        assert_eq!(score, 7036);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }
}
