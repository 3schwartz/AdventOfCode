use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day10_data.txt")?;

    let grid = Grid::new(&input);

    let (total_hikes, total_tops) = grid.find_hikes();

    println!("Part 1: {}", total_tops);
    println!("Part 2: {}", total_hikes);

    Ok(())
}

struct Position {
    coord: (i32, i32),
    value: u8,
}

impl Position {
    fn new(coord: (i32, i32), value: u8) -> Self {
        Self { coord, value }
    }
}

struct Grid {
    grid: HashMap<(i32, i32), u8>,
    x_max: i32,
    y_max: i32,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut grid: HashMap<(i32, i32), u8> = HashMap::new();
        let mut x_max: i32 = 0;
        let mut y_max: i32 = 0;
        for (y, line) in input.lines().enumerate() {
            y_max = y as i32;
            for (x, c) in line.chars().enumerate() {
                grid.insert((x as i32, y as i32), c as u8 - b'0');
                x_max = x as i32;
            }
        }
        Self { grid, x_max, y_max }
    }

    fn neighbors() -> [(i32, i32); 4] {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
    }

    fn find_hikes(&self) -> (u32, u32) {
        let mut total_hikes = 0;
        let mut total_tops = 0;
        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                let coord = *self
                    .grid
                    .get(&(x, y))
                    .unwrap_or_else(|| panic!("{:?}", (x, y)));
                if coord != 0 {
                    continue;
                }
                let mut queue = vec![Position::new((x, y), 0)];
                let mut total = 0;
                let mut final_top = HashSet::new();
                while let Some(next) = queue.pop() {
                    for neighbor in Grid::neighbors() {
                        let neighbor_coord = (next.coord.0 + neighbor.0, next.coord.1 + neighbor.1);
                        let n = match self.grid.get(&neighbor_coord) {
                            Some(n) => *n,
                            None => continue,
                        };
                        if n != next.value + 1 {
                            continue;
                        }

                        if n == 9 {
                            final_top.insert(neighbor_coord);
                            total += 1;
                        } else {
                            queue.push(Position::new(neighbor_coord, n));
                        }
                        assert!(n <= 9);
                    }
                }
                total_tops += final_top.len() as u32;
                total_hikes += total;
            }
        }
        (total_hikes, total_tops)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_and_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day10_test_data.txt")?;

        // Act
        let grid = Grid::new(&input);
        let (total_hikes, total_tops) = grid.find_hikes();

        // Assert
        assert_eq!(total_hikes, 81);
        assert_eq!(total_tops, 36);
        Ok(())
    }
}
