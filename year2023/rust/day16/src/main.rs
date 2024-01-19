use std::{collections::HashSet, fs, usize};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day16_data.txt")?;
    let grid = grid(&input);
    let position = Position::new(0, 0, 0);
    let visited = position.energize(&grid);

    println!("Part 1: {}", visited);

    let max = Position::max_energize(&grid);
    println!("Part 2: {}", max);
    Ok(())
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: i32,
    col: i32,
    // 0: right, 1: down, 2: left, 3: up
    direction: usize,
}

impl Position {
    // right, down, left, up
    const ROW_DIRECTION: [i32; 4] = [0, 1, 0, -1];
    const COLUMN_DIRECTION: [i32; 4] = [1, 0, -1, 0];
    const SLASH_DIRECTION: [usize; 4] = [3, 2, 1, 0];
    const BACKSLASH_DIRECTION: [usize; 4] = [1, 0, 3, 2];

    fn new(row: i32, col: i32, direction: usize) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }

    fn tile(&self, grid: &[Vec<char>]) -> char {
        grid[self.row as usize][self.col as usize]
    }

    fn above_limits(&self, grid: &[Vec<char>]) -> bool {
        self.row < 0
            || self.row >= grid.len() as i32
            || self.col < 0
            || self.col >= grid[0].len() as i32
    }

    fn inc(&self, direction: usize) -> Position {
        Position::new(
            self.row + Position::ROW_DIRECTION[direction],
            self.col + Position::COLUMN_DIRECTION[direction],
            direction,
        )
    }

    fn next(&self, c: char) -> Vec<Position> {
        match c {
            '.' => vec![self.inc(self.direction)],
            '/' => {
                let direction = Position::SLASH_DIRECTION[self.direction];
                vec![self.inc(direction)]
            }
            '\\' => {
                let direction = Position::BACKSLASH_DIRECTION[self.direction];
                vec![self.inc(direction)]
            }
            '|' => {
                if self.direction == 1 || self.direction == 3 {
                    vec![self.inc(self.direction)]
                } else {
                    vec![self.inc(1), self.inc(3)]
                }
            }
            '-' => {
                if self.direction == 0 || self.direction == 2 {
                    vec![self.inc(self.direction)]
                } else {
                    vec![self.inc(0), self.inc(2)]
                }
            }
            _ => panic!("{}", c),
        }
    }

    fn max_energize(grid: &Vec<Vec<char>>) -> usize {
        let mut max = usize::MIN;
        for r in 0..grid.len() {
            max = std::cmp::max(max, Position::new(r as i32, 0, 0).energize(grid)); // >
            max = std::cmp::max(
                max,
                Position::new(r as i32, grid[0].len() as i32 - 1, 2).energize(grid),
            ) // <
        }
        for c in 0..grid[0].len() {
            max = std::cmp::max(max, Position::new(0, c as i32, 1).energize(grid)); // >
            max = std::cmp::max(
                max,
                Position::new(grid.len() as i32 - 1, c as i32, 3).energize(grid),
            ) // <
        }
        max
    }

    fn energize(&self, grid: &[Vec<char>]) -> usize {
        let mut positions = vec![*self];
        let mut visited = HashSet::new();
        let mut cache = HashSet::new();

        loop {
            if positions.is_empty() {
                break;
            }
            let mut next = vec![];

            for position in &positions {
                if position.above_limits(grid) {
                    continue;
                }
                if !cache.insert(*position) {
                    continue;
                }
                visited.insert((position.row, position.col));

                let tile = position.tile(grid);
                let mut movements = position.next(tile);
                next.append(&mut movements);
            }
            positions = next;
        }
        visited.len()
    }
}

fn grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for column in line.chars() {
            row.push(column);
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day16_data_test.txt")?;
        let grid = grid(&input);
        let position = Position::new(0, 0, 0);

        // Act
        let visited = position.energize(&grid);

        // Assert
        assert_eq!(visited, 46);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day16_data_test.txt")?;
        let grid = grid(&input);

        // Act
        let visited = Position::max_energize(&grid);

        // Assert
        assert_eq!(visited, 51);
        Ok(())
    }
}
