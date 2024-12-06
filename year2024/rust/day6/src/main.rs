use anyhow::Result;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day6_data.txt")?;

    let mut start = (0, 0);
    let mut rocks = HashSet::new();
    let mut free = HashSet::new();
    let mut row_max = 0;
    let mut column_max = 0;
    for (column, line) in input.lines().enumerate() {
        column_max = column as i32;
        for (row, c) in line.chars().enumerate() {
            row_max = row as i32;
            match c {
                '^' => start = (row as i32, column as i32),
                '#' => {
                    rocks.insert((row as i32, column as i32));
                }
                _ => {
                    free.insert((row as i32, column as i32));
                }
            }
        }
    }

    let guard_simple = GuardPlay::play(start, row_max, column_max, &rocks);

    println!("Part 1: {}", guard_simple.visited);

    let stuck_count = GuardPlay::stuck_count(start, row_max, column_max, &free, &rocks);

    println!("Part 2: {}", stuck_count);

    Ok(())
}

struct GuardPlay {
    guard_stuck: bool,
    visited: usize,
}

impl GuardPlay {
    fn stuck_count(
        start: (i32, i32),
        row_max: i32,
        column_max: i32,
        free: &HashSet<(i32, i32)>,
        rocks: &HashSet<(i32, i32)>,
    ) -> u32 {
        let mut total = 0;

        for f in free {
            let mut rocks_copy = rocks.clone();
            rocks_copy.insert(*f);
            let play = GuardPlay::play(start, row_max, column_max, &rocks_copy);
            if play.guard_stuck {
                total += 1;
            }
        }

        total
    }

    fn play(
        start: (i32, i32),
        row_max: i32,
        column_max: i32,
        rocks: &HashSet<(i32, i32)>,
    ) -> GuardPlay {
        let mut visisted_same_direction = HashSet::new();
        let mut visited = HashSet::from([start]);
        let mut current = start;
        let mut direction: (i32, i32) = (0, -1);
        let mut guard_stuck = false;

        loop {
            if current.0 < 0 || current.0 > row_max || current.1 < 0 || current.1 > column_max {
                break;
            }
            if visisted_same_direction.contains(&(current, direction)) {
                guard_stuck = true;
                break;
            }
            visited.insert(current);
            visisted_same_direction.insert((current, direction));

            let next = (current.0 + direction.0, current.1 + direction.1);
            if rocks.contains(&next) {
                direction = (-direction.1, direction.0)
            } else {
                current = next
            };
        }

        Self {
            guard_stuck,
            visited: visited.len(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        // Act
        // Assert
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
