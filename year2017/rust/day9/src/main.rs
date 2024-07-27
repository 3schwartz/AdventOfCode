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
        match (discard, is_garbage, c) {
            // Case when discard is true
            (true, _, _) => {
                discard = false;
            }
            // Case when character is '!'
            (false, _, '!') => {
                discard = true;
            }
            // Case when is_garbage is true and character is not '>'
            (false, true, ch) if ch != '>' => {
                cancelled += 1;
            }
            // Case when is_garbage is true and character is '>'
            (false, true, '>') => {
                is_garbage = false;
            }
            // Case when is_garbage is false and character is '<'
            (false, false, '<') => {
                is_garbage = true;
            }
            // Case when character is '{'
            (false, false, '{') => {
                debt += 1;
            }
            // Case when character is '}'
            (false, false, '}') => {
                sum += debt;
                debt -= 1;
            }
            // Default case to handle other characters
            _ => {}
        }
        continue;
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", cancelled);
    Ok(())
}
