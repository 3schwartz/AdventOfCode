use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day3_data.txt")?;

    for line in input.lines() {
        println!("{}", line);
    }

    Ok(())
}
