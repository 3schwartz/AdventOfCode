use std::{collections::HashMap, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day1_data.txt")?;

    let mut first = vec![];
    let mut second = vec![];
    let mut second_map = HashMap::new();

    for line in input.lines() {
        let parts = line
            .split("   ")
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        first.push(parts[0]);
        second.push(parts[1]);

        *second_map.entry(parts[1]).or_insert(0) += 1;
    }

    first.sort();
    second.sort();

    let diff = first
        .iter()
        .zip(&second)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    let similarity = first
        .iter()
        .map(|v| v * second_map.get(v).unwrap_or(&0))
        .sum::<i32>();

    println!("Part 1: {}", diff);
    println!("Part 2: {}", similarity);

    Ok(())
}
