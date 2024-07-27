use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day10_data.txt")?;
    let lengths = input
        .split(',')
        .map(|l| l.trim().parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    let mut list: Vec<u8> = (0..=255).collect();
    let list_length = 256;

    let mut cursor = 0;

    for (skip_size, length) in lengths.iter().copied().enumerate() {
        let mut to_reverse: Vec<u8> = Vec::with_capacity(length);
        for j in 0..length {
            to_reverse.push(list[(j + cursor) % list_length])
        }
        to_reverse.reverse();
        for (j, reverse) in to_reverse.iter().enumerate() {
            list[(j + cursor) % list_length] = *reverse;
        }

        cursor = (cursor + skip_size + length) % list_length;
    }

    println!("Part 1: {}", list[0] as u32 * list[1] as u32);
    Ok(())
}
