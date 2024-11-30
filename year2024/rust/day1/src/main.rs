use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day0_test.txt")?;

    println!("{}", input);

    Ok(())
}
