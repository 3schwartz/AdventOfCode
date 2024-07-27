use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day9_data.txt")?;

    let mut is_garbage = false;
    let mut discard = false;
    let mut debt = 0;
    let mut sum = 0;
    let mut cancelled = 0;

    for c in input.chars() {
        if discard {
            discard = false;
            continue;
        }
        if c == '!' {
            discard = true;
            continue;
        }
        if is_garbage && c != '>' {
            cancelled += 1;
            continue;
        }
        if is_garbage && c == '>' {
            is_garbage = false;
            continue;
        }
        if !is_garbage && c == '<' {
            is_garbage = true;
            continue;
        }
        if c == '{' {
            debt += 1;
            continue;
        }
        if c == '}' {
            sum += debt;
            debt -= 1;
        }
        continue;
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", cancelled);
    Ok(())
}
