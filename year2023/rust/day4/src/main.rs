use std::{collections::HashSet, fs};

use anyhow::{Result, Ok};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day4_data.txt")?;
    let total = get_points(&input)?;

    println!("Part 1: {}", total);

    Ok(())
}

fn get_points(input: &str) -> Result<u32> {
    let mut total = 0;
    for line in input.lines() {
        let trimmed: Vec<&str> = line.split(": ").collect();
        let card_split: Vec<&str> = trimmed[1].split(" | ").collect();
        let win: HashSet<u32> = card_split[0]
            .split_whitespace()
            .map(|c| c.parse())
            .collect::<Result<HashSet<u32>, _>>()?;
        let numbers: HashSet<u32> = card_split[1]
            .split_whitespace()
            .map(|c| c.parse())
            .collect::<Result<HashSet<u32>, _>>()?;
        let intersection = numbers.intersection(&win)
            .collect::<HashSet<&u32>>();
        if intersection.is_empty() {
            continue;
        }
        let points = 2_u32.pow(intersection.len() as u32 - 1);
        total += points;
    }
    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day4_data test.txt")?;

        // Act
        let total = get_points(&input)?;

        // Assert
        assert_eq!(13, total);
        Ok(())
    }
}
