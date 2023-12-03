use std::{fs, collections::HashMap};

use anyhow::{Result, anyhow, Ok};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day3_data.txt")?;
    let sum_of_parts = find_sum_of_parts(&input)?;
    println!("Part 1: {}", sum_of_parts);
    Ok(())
}

fn find_sum_of_parts(input: &str) -> Result<u32> {
    let mut coords = HashMap::new();
    let mut row_max = 0;
    let mut column_max = 0;
    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            coords.insert((row as i32, column as i32), c);
            column_max = column as i32;
        }
        row_max = row as i32;
    }

    let mut numbers: Vec<u32> = Vec::new();
    for r in 0..=row_max {
        let mut has_symbol = false;
        let mut number = String::new();
        for c in 0..=column_max {
            let p = coords.get(&(r, c)).ok_or(anyhow!("({},{})", r,c))?;
            if p.is_numeric() {
                number.push(*p);
            }
            if p.is_numeric() && !has_symbol {
                has_symbol = has_symbol_around(&(r,c), &coords);
            }
            if !p.is_numeric() {
                if has_symbol && number.len() > 0 {
                    numbers.push(number.parse()?)
                }
                has_symbol = false;
                number = String::new();
            }
        }
        if has_symbol && number.len() > 0 {
            numbers.push(number.parse()?)
        }
    }

    let sum_of_parts: u32 = numbers.iter().sum();
    Ok(sum_of_parts)
}

fn has_symbol_around(coord: &(i32, i32), coords: &HashMap<(i32, i32), char>) -> bool {
    for r in coord.0-1..=coord.0+1 {
        for c in coord.1-1..=coord.1+1 {
            if r == coord.0 && c == coord.1 {
                continue;
            }
            let p = coords.get(&(r,c));
            if p.is_none() {
                continue;
            }
            let p = p.unwrap();
            if p.is_numeric() {
                continue;
            }
            if p == &'.' {
                continue;
            }
            return true;
        }   
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day3_data_test.txt")?;

        // Act
        let sum_of_parts = find_sum_of_parts(&input)?;

        // Assert
        assert_eq!(4361, sum_of_parts);
        Ok(())
    }
}
