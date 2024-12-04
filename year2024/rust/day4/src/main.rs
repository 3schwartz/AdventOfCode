use anyhow::Result;
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day4_data.txt")?;

    let total_match = find_total_xmas(&input);

    println!("Part 1: {}", total_match);

    Ok(())
}

fn find_total_xmas(input: &str) -> u32 {
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

    for x in 0..=x_max {
        for y in 0..=y_max {
            let c = grid.get(&(y, x)).unwrap();
            if *c != 'X' {
                continue;
            }
            for direction in directions {
                let mut is_match = true;

                for i in 0..to_match.len() {
                    let shift = direction[i];
                    let n = to_match[i];
                    if let Some(m) = grid.get(&(y + shift.0, x + shift.1)) {
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
        let input = fs::read_to_string("../../data/day4_test_data.txt").unwrap();

        // Act
        let total_match = find_total_xmas(&input);

        // Assert
        assert_eq!(total_match, 18)
    }
}
