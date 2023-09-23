use std::{fs, collections::BTreeSet};

use anyhow::Result;
fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day6_data.txt")?;

    let mut banks = vec![];
    for c in input.split_whitespace() {
        let int: u32 = c.parse()?;
        banks.push(int);
    }

    let mut seen = BTreeSet::new();
    let bank_length = banks.len() as u32;
    let mut redistribution_cycles = 0;
    loop {
        if !seen.insert(banks.clone()) {
            break;
        }
        redistribution_cycles += 1;
        let next_idx = max_idx(&banks);
        let next_bank = banks.get_mut(next_idx).unwrap();
        let count = *next_bank;
        *next_bank = 0;

        let to_all = count / bank_length;
        for ele in &mut banks {
            *ele += to_all;
        }

        let one_more = count % bank_length;
        for i in 1..=one_more as usize {
            let idx = (i + next_idx) % bank_length as usize;
            let one_more_bank = banks.get_mut(idx).unwrap();
            *one_more_bank += 1;
        }
    }

    println!("Part 1: {}", redistribution_cycles);

    Ok(())
}

fn max_idx(vector: &Vec<u32>) -> usize {
    let mut idx = 0;
    let mut max = u32::MIN;
    for (i, int) in vector.iter().enumerate() {
        if *int > max {
            max = *int;
            idx = i;
        }
    }
    idx
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_in_btree_set() {
        // Arrange
        let first = vec![1,2];
        let second = vec![1,2];
        let mut set = BTreeSet::new();
        set.insert(first);

        // Act
        let inserted = set.insert(second);

        // Assert
        assert!(!inserted);
    }
}
