use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day4_data.txt")?;
    let total = get_points(&input)?;

    println!("Part 1: {}", total);

    let total_scratchcards = get_total_scratchcards(&input)?;

    println!("Part 2: {}", total_scratchcards);

    Ok(())
}

fn get_total_scratchcards(input: &str) -> Result<u32> {
    let mut card_map = HashMap::new();
    let mut idx_max = 0;
    for (idx, line) in input.lines().enumerate() {
        idx_max = idx;
        let current = *card_map.entry(idx).or_insert(1u32);
        let intersection = get_intersection(line)?;
        for i in 1..=intersection.len() {
            let entry = card_map.entry(i + idx).or_insert(1u32);
            *entry += current;
        }
    }
    let sum: u32 = card_map
        .iter()
        .filter(|(&k, _)| k <= idx_max)
        .map(|(_, &v)| v)
        .sum();
    Ok(sum)
}

fn get_points(input: &str) -> Result<u32> {
    let mut total = 0;
    for line in input.lines() {
        let intersection = get_intersection(line)?;
        if intersection.is_empty() {
            continue;
        }
        let points = 2_u32.pow(intersection.len() as u32 - 1);
        total += points;
    }
    Ok(total)
}

fn get_intersection(line: &str) -> Result<HashSet<u32>> {
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
    let intersection = numbers
        .intersection(&win)
        .copied()
        .collect::<HashSet<u32>>();
    Ok(intersection)
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

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day4_data test.txt")?;

        // Act
        let total = get_total_scratchcards(&input)?;

        // Assert
        assert_eq!(30, total);
        Ok(())
    }
}
