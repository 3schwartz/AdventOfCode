use std::{fs, collections::HashSet};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day4_data.txt")?;
    let mut valid_passphrases = 0;

    for line in input.lines() {
        let mut seen = HashSet::new();
        let mut valid = true;
        for word in line.split_whitespace() {
            if !seen.insert(word) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_passphrases += 1;
        }
    }
    
    println!("Part 1: {}", valid_passphrases);

    Ok(())
}
