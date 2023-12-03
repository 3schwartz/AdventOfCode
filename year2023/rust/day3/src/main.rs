use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{anyhow, Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day3_data.txt")?;
    let (row_max, column_max, coords) = create_map(&input);

    let sum_of_parts = find_sum_of_parts(row_max, column_max, &coords)?;
    println!("Part 1: {}", sum_of_parts);

    let numbers = create_number_map(&coords, row_max, column_max)?;
    let sum_of_ratios = find_sum_gear_ratio(row_max, column_max, &coords, &numbers)?;

    println!("Part 2: {}", sum_of_ratios);
    Ok(())
}

fn create_map(input: &str) -> (i32, i32, HashMap<(i32, i32), char>) {
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
    (row_max, column_max, coords)
}

fn create_number_map(
    coords: &HashMap<(i32, i32), char>,
    row_max: i32,
    column_max: i32,
) -> Result<HashMap<(i32, i32), Number>> {
    let mut number_map = HashMap::new();
    let mut idx: u32 = 0;
    for r in 0..=row_max {
        let mut number_coords = vec![];
        let mut number = vec![];
        idx += 1;
        for c in 0..=column_max {
            let p = coords.get(&(r, c)).ok_or(anyhow!("({},{})", r, c))?;
            if p.is_numeric() {
                number.push(*p);
                number_coords.push((r, c));
                continue;
            }
            if !number.is_empty() {
                let parsed_number: u32 = number.iter().collect::<String>().parse()?;
                for number_coord in number_coords {
                    number_map.insert(number_coord, Number::new(parsed_number, idx));
                }
            }
            number_coords = vec![];
            number = vec![];
            idx += 1;
        }
        if !number.is_empty() {
            let parsed_number: u32 = number.iter().collect::<String>().parse()?;
            for number_coord in number_coords {
                number_map.insert(number_coord, Number::new(parsed_number, idx));
            }
        }
    }
    Ok(number_map)
}

struct Number {
    number: u32,
    idx: u32,
}

impl Number {
    fn new(number: u32, idx: u32) -> Self {
        Self { number, idx }
    }
}

fn find_sum_gear_ratio(
    row_max: i32,
    column_max: i32,
    coords: &HashMap<(i32, i32), char>,
    numbers: &HashMap<(i32, i32), Number>,
) -> Result<u32> {
    let mut ratios = vec![];
    for r in 0..=row_max {
        for c in 0..=column_max {
            let p = coords.get(&(r, c)).ok_or(anyhow!("({},{})", r, c))?;
            if p != &'*' {
                continue;
            }
            if let Some(ratio) = get_gear_ratio(&(r, c), numbers) {
                ratios.push(ratio);
            }
        }
    }
    Ok(ratios.iter().sum())
}

fn get_gear_ratio(coord: &(i32, i32), numbers: &HashMap<(i32, i32), Number>) -> Option<u32> {
    let mut idx = HashSet::new();
    let mut numbs = vec![];
    for r in coord.0 - 1..=coord.0 + 1 {
        for c in coord.1 - 1..=coord.1 + 1 {
            if r == coord.0 && c == coord.1 {
                continue;
            }
            if let Some(n) = numbers.get(&(r, c)) {
                if idx.insert(n.idx) {
                    numbs.push(n.number);
                }
            }
        }
    }
    // check if is gear
    if numbs.len() != 2 {
        return None;
    }
    let mult: u32 = numbs.iter().product();
    Some(mult)
}

fn find_sum_of_parts(
    row_max: i32,
    column_max: i32,
    coords: &HashMap<(i32, i32), char>,
) -> Result<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for r in 0..=row_max {
        let mut has_symbol = false;
        let mut number = vec![];
        for c in 0..=column_max {
            let p = coords.get(&(r, c)).ok_or(anyhow!("({},{})", r, c))?;
            if p.is_numeric() {
                number.push(*p);
            }
            if p.is_numeric() && !has_symbol {
                has_symbol = has_symbol_around(&(r, c), coords);
            }
            if !p.is_numeric() {
                if has_symbol && !number.is_empty() {
                    let parsed_number = number.iter().collect::<String>().parse()?;
                    numbers.push(parsed_number)
                }
                has_symbol = false;
                number = vec![];
            }
        }
        if has_symbol && !number.is_empty() {
            let parsed_number = number.iter().collect::<String>().parse()?;
            numbers.push(parsed_number)
        }
    }

    let sum_of_parts: u32 = numbers.iter().sum();
    Ok(sum_of_parts)
}

fn has_symbol_around(coord: &(i32, i32), coords: &HashMap<(i32, i32), char>) -> bool {
    for r in coord.0 - 1..=coord.0 + 1 {
        for c in coord.1 - 1..=coord.1 + 1 {
            if r == coord.0 && c == coord.1 {
                continue;
            }
            let p = coords.get(&(r, c));
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
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day3_data_test.txt")?;
        let (row_max, column_max, coords) = create_map(&input);

        // Act
        let sum_of_parts = find_sum_of_parts(row_max, column_max, &coords)?;

        // Assert
        assert_eq!(4361, sum_of_parts);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day3_data_test.txt")?;
        let (row_max, column_max, coords) = create_map(&input);

        // Act
        let numbers = create_number_map(&coords, row_max, column_max)?;
        let sum_of_ratios = find_sum_gear_ratio(row_max, column_max, &coords, &numbers)?;

        // Assert
        assert_eq!(467835, sum_of_ratios);
        Ok(())
    }
}
