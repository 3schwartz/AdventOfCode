use anyhow::Result;
use day10::{execute_round, make_knot_hash};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day10_data.txt")?;
    let lengths = input
        .split(',')
        .map(|l| l.trim().parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    let mut list: Vec<u8> = (0..=255).collect();

    let mut cursor = 0;
    let mut skip_size = 0;

    execute_round(&mut list, &lengths, &mut cursor, &mut skip_size);
    println!("Part 1: {}", list[0] as u32 * list[1] as u32);

    let hex_string = make_knot_hash(&input);

    println!("Part 2: {}", hex_string);

    Ok(())
}
