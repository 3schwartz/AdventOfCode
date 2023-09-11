use std::{collections::BTreeSet, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day4_data.txt")?;

    let part_1 = get_valid_passphrases(&input, same);
    println!("Part 1: {}", part_1);

    let part_2 = get_valid_passphrases_rearrange(&input);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn same(word: &str) -> &str {
    word
}

fn get_valid_passphrases<F>(input: &str, condition: F) -> u32
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let mut valid_passphrases = 0;
    for line in input.lines() {
        let mut seen = BTreeSet::new();
        let mut valid = true;
        for word in line.split_whitespace() {
            let lookup = condition(word);
            if !seen.insert(lookup) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_passphrases += 1;
        }
    }

    valid_passphrases
}

fn get_valid_passphrases_rearrange(input: &str) -> u32 {
    let mut valid_passphrases = 0;
    for line in input.lines() {
        let mut seen = BTreeSet::new();
        let mut valid = true;
        for word in line.split_whitespace() {
            let lookup: BTreeSet<char> = word.chars().collect();
            if !seen.insert(lookup) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_passphrases += 1;
        }
    }

    valid_passphrases
}
