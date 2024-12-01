use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day2_data.txt")?;

    for line in input.lines() {
        println!("{}", line);
    }

    Ok(())
}
