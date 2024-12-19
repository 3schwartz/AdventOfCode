use anyhow::{anyhow, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day18_data.txt")?;

    let memory = MemorySpace::new(&input, 1024, 70, 70)?;
    let steps = memory.steps().unwrap();

    println!("Part 1: {}", steps);

    let coord = MemorySpace::find_non_reachable(&input, 70, 70)?;

    println!("Part 2: {coord}");
    Ok(())
}

use std::collections::{HashSet, VecDeque};

struct MemorySpace {
    x: i32,
    y: i32,
    corrupt: HashSet<(i32, i32)>,
}

impl MemorySpace {
    const N: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn find_non_reachable(input: &str, x: i32, y: i32) -> Result<String> {
        let coords = input.lines().collect::<Vec<&str>>();
        for i in 1..=coords.len() {
            let memory = MemorySpace::new(&input, i, x, y)?;
            if memory.steps().is_some() {
                continue;
            };
            return Ok(coords[i - 1].to_string());
        }
        Err(anyhow!("no result found"))
    }

    fn new(input: &str, size: usize, x: i32, y: i32) -> Result<Self> {
        let mut corrupt = HashSet::new();
        for (i, line) in input.lines().enumerate() {
            if i == size {
                break;
            }
            let parts = line
                .split(',')
                .map(|n| n.parse())
                .collect::<Result<Vec<i32>, _>>()?;
            assert_eq!(parts.len(), 2);
            corrupt.insert((parts[0], parts[1]));
        }
        Ok(Self { x, y, corrupt })
    }

    fn steps(&self) -> Option<u32> {
        let mut queue = VecDeque::from([(0, (0, 0))]);
        let mut seen = HashSet::new();

        while let Some((steps, (x, y))) = queue.pop_front() {
            if !seen.insert((x, y)) {
                continue;
            }
            if x == self.x && y == self.y {
                return Some(steps);
            }
            if self.corrupt.contains(&(x, y)) {
                continue;
            }
            for neighbor in Self::N {
                let n = (neighbor.0 + x, neighbor.1 + y);
                if n.0 < 0 || n.0 > self.x || n.1 < 0 || n.1 > self.y {
                    continue;
                }
                queue.push_back((steps + 1, n));
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day18_test_data.txt")?;

        // Act
        let memory = MemorySpace::new(&input, 12, 6, 6)?;
        let steps = memory.steps();

        // Assert
        assert_eq!(steps.unwrap(), 22);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day18_test_data.txt")?;

        // Act
        let coord = MemorySpace::find_non_reachable(&input, 6, 6)?;

        // Assert
        assert_eq!(coord, "6,1");
        Ok(())
    }
}
