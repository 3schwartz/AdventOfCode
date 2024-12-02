use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day2_data.txt")?;

    let mut total_safe_part_1 = 0;
    let mut total_safe_part_2 = 0;

    for line in input.lines() {
        let parts = line
            .split(" ")
            .map(|n| n.parse())
            .collect::<Result<Vec<i32>, _>>()?;

        if is_line_save(&parts) {
            total_safe_part_1 += 1;
            total_safe_part_2 += 1;
            continue;
        }

        for i in 0..parts.len() {
            let copied: Vec<i32> = parts
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != i)
                .map(|(_, &value)| value)
                .collect();

            if is_line_save(&copied) {
                total_safe_part_2 += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", total_safe_part_1);
    println!("Part 2: {}", total_safe_part_2);

    Ok(())
}

fn is_line_save(parts: &Vec<i32>) -> bool {
    let mut is_negative = None;

    for i in 1..parts.len() {
        let direction = parts[i - 1] - parts[i];
        if let Some(prior_direction) = is_negative {
            if prior_direction != (direction < 0) {
                return false;
            }
        } else {
            is_negative = Some(direction < 0);
        }

        if parts[i - 1] == parts[i] || (parts[i - 1] - parts[i]).abs() > 3 {
            return false;
        }
    }
    return true;
}
