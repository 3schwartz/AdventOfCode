use anyhow::Result;
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day4_data.txt")?;

    let grid = Grid::new(&input);

    let total_match = find_total_xmas(&grid);

    println!("Part 1: {}", total_match);

    let total_match = find_x_mas(&grid);

    println!("Part 2: {}", total_match);

    Ok(())
}

struct Grid {
    grid: HashMap<(i32, i32), char>,
    x_max: i32,
    y_max: i32,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut grid: HashMap<(i32, i32), char> = HashMap::new();
        let mut x_max: i32 = 0;
        let mut y_max: i32 = 0;
        for (y, line) in input.lines().enumerate() {
            y_max = y as i32;
            for (x, c) in line.chars().enumerate() {
                grid.insert((y as i32, x as i32), c);
                x_max = x as i32;
            }
        }
        Self { grid, x_max, y_max }
    }
}

fn find_x_mas(grid: &Grid) -> u32 {
    let x_mas_cross_opportunities = [
        [[(-1, -1), (0, 0), (1, 1)], [(1, 1), (0, 0), (-1, -1)]],
        [[(-1, 1), (0, 0), (1, -1)], [(1, -1), (0, 0), (-1, 1)]],
    ];
    let term_to_match = ['M', 'A', 'S'];
    let mut total_crosses = 0;

    for x in 0..=grid.x_max {
        for y in 0..=grid.y_max {
            let c = grid.grid.get(&(y, x)).unwrap();
            if *c != 'A' {
                continue;
            }
            let mut is_cross_match = true;
            for diagonal in x_mas_cross_opportunities {
                let mut is_match_in_one_diagonal = false;
                for diagonal_opportunity in diagonal {
                    let mut opportunity_match = true;
                    for i in 0..diagonal_opportunity.len() {
                        let shift = diagonal_opportunity[i];
                        let term = term_to_match[i];
                        if let Some(m) = grid.grid.get(&(y + shift.0, x + shift.1)) {
                            if *m != term {
                                opportunity_match = false;
                                break;
                            }
                        } else {
                            opportunity_match = false;
                            break;
                        }
                    }
                    if opportunity_match {
                        is_match_in_one_diagonal = true;
                        break;
                    }
                }
                if !is_match_in_one_diagonal {
                    is_cross_match = false;
                    break;
                }
            }
            if is_cross_match {
                total_crosses += 1;
            }
        }
    }
    total_crosses
}

fn find_total_xmas(grid: &Grid) -> u32 {
    let directions = [
        [(0, 1), (0, 2), (0, 3)],
        [(0, -1), (0, -2), (0, -3)],
        [(1, 0), (2, 0), (3, 0)],
        [(-1, 0), (-2, 0), (-3, 0)],
        [(-1, 1), (-2, 2), (-3, 3)],
        [(1, 1), (2, 2), (3, 3)],
        [(1, -1), (2, -2), (3, -3)],
        [(-1, -1), (-2, -2), (-3, -3)],
    ];
    let to_match = ['M', 'A', 'S'];

    let mut total_match = 0;

    for x in 0..=grid.x_max {
        for y in 0..=grid.y_max {
            let c = grid.grid.get(&(y, x)).unwrap();
            if *c != 'X' {
                continue;
            }
            for direction in directions {
                let mut is_match = true;

                for i in 0..to_match.len() {
                    let shift = direction[i];
                    let n = to_match[i];
                    if let Some(m) = grid.grid.get(&(y + shift.0, x + shift.1)) {
                        if *m != n {
                            is_match = false;
                            break;
                        }
                    } else {
                        is_match = false;
                        break;
                    }
                }
                if is_match {
                    total_match += 1;
                }
            }
        }
    }
    total_match
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        let input = fs::read_to_string("../data/day4_test_data.txt").unwrap();
        let grid = Grid::new(&input);

        // Act
        let total_match = find_total_xmas(&grid);

        // Assert
        assert_eq!(total_match, 18)
    }

    #[test]
    fn test_part_2() {
        // Arrange
        let input = fs::read_to_string("../data/day4_test_data.txt").unwrap();
        let grid = Grid::new(&input);

        // Act
        let total_match = find_x_mas(&grid);

        // Assert
        assert_eq!(total_match, 9)
    }
}
