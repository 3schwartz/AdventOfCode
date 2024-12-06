use anyhow::Result;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day6_data.txt")?;

    let mut start = (0, 0);
    let mut rocks = HashSet::new();
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
                _ => (),
            }
        }
    }

    let mut visited = HashSet::from([start]);
    let mut current = start;
    let mut direction: (i32, i32) = (0, -1);

    loop {
        if current.0 < 0 || current.0 > row_max || current.1 < 0 || current.1 > column_max {
            break;
        }
        visited.insert(current);
        let next = (current.0 + direction.0, current.1 + direction.1);
        if rocks.contains(&next) {
            direction = (-1 * direction.1, direction.0)
        } else {
            current = next
        };
    }

    println!("Part 1: {}", visited.len());

    Ok(())
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
