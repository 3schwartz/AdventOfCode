use std::fs;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day2_data.txt")?;

    let part_1 = find_checksum_min_max(&input)?;
    println!("{}", part_1);

    let part_2 = find_checksum_evenly(&input)?;
    println!("{}", part_2);

    Ok(())
}

fn find_checksum_evenly(input: &String) -> Result<u32> {
    let mut checksum = 0;
    for line in input.lines() {
        let evenly = find_evenly(line)?;
        checksum += evenly;
    }
    Ok(checksum)
}

fn find_evenly(input: &str) -> Result<u32> {
    let numbers = find_numbers(input)?;
    for (idx_o, outer) in numbers.iter().enumerate() {
        for (idx_i, inner) in numbers.iter().enumerate() {
            if idx_o == idx_i {
                continue;
            }
            if outer % inner == 0 {
                return Ok(outer / inner);
            }
        }
    }
    return Err(anyhow!("{input}"));
}

fn find_checksum_min_max(input: &String) -> Result<u32> {
    let mut checksum = 0;
    for line in input.lines() {
        let (min, max) = find_min_max(line)?;
        checksum += max - min;
    }
    Ok(checksum)
}

fn find_min_max(input: &str) -> Result<(u32, u32)> {
    let numbers = find_numbers(input)?;

    let min = numbers.iter().min().expect("min should be there");
    let max = numbers.iter().max().expect("max should be there");

    Ok((*min, *max))
}

fn find_numbers(input: &str) -> Result<Vec<u32>> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().map_err(|e| anyhow!(e)))
        .into_iter()
        .collect::<Result<Vec<u32>>>()
}
