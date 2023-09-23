use std::fs;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day5_data.txt")?;
    let lines = input.lines();
    let mut instructions: Vec<i32> = Vec::with_capacity(lines.count());
    for line in input.lines() {
        let instruction = line.parse::<i32>()?;
        instructions.push(instruction);
    }

    let part_1 = find_jump_count(instructions.clone(), false);
    println!("Part 1: {}", part_1);

    let part_2 = find_jump_count(instructions.clone(), true);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn find_jump_count(mut instructions: Vec<i32>, part_2: bool) -> u32 {
    let mut jumps = 0;
    let mut idx = 0;
    loop {
        let Some(instruction) = instructions.get_mut(idx) else {
            break;
        };
        jumps += 1;
        let next = idx as i32 + *instruction;
        if next < 0 {
            break;
        }
        idx = next as usize;
        if part_2 && *instruction >= 3 {
            *instruction += -1;
        } else {
            *instruction += 1;
        }
    }
    jumps
}
