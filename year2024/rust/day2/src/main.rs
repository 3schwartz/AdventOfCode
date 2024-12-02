use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day2_data.txt")?;

    let mut total_safe = 0;

    for line in input.lines() {
        let parts = line
            .split(" ")
            .map(|n| n.parse())
            .collect::<Result<Vec<i32>, _>>()?;

        let mut is_safe = true;
        let mut is_negative = None;

        for i in 1..parts.len() {
            // if i > parts.len() - 1
            //     && (parts[i - 1] < parts[i] && parts[i] > parts[i + 1]
            //         || parts[i - 1] < parts[i] && parts[i] > parts[i + 1])
            // {
            //     is_safe = false;
            //     break;
            // }

            let direction = parts[i - 1] - parts[i];
            if let Some(prior_direction) = is_negative {
                if prior_direction != (direction < 0) {
                    is_safe = false;
                    break;
                }
            } else {
                is_negative = Some(direction < 0);
            }

            if parts[i - 1] == parts[i] || (parts[i - 1] - parts[i]).abs() > 3 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            total_safe += 1;
        }
    }

    println!("Part 1: {}", total_safe);

    Ok(())
}
