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

        second_map
            .entry(parts[1])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    first.sort();
    second.sort();

    let mut diff = 0;
    let mut similaraity = 0;
    for i in 0..first.len() {
        diff += (first[i] - second[i]).abs();
        similaraity += first[i] * second_map.get(&first[i]).unwrap_or(&0)
    }

    println!("Part 1: {}", diff);
    println!("Part 2: {}", similaraity);

    Ok(())
}
