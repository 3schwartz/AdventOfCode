use anyhow::{anyhow, Result};
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    str::FromStr,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day25_data.txt")?;

    let state = State::from_str(&input)?;

    let pairs = state.find_unique_lock_key_pairs();

    println!("Part 1: {pairs}");

    Ok(())
}

struct State {
    locks: Vec<[usize; 5]>,
    keys: BTreeMap<usize, BTreeMap<usize, HashSet<usize>>>,
}

impl State {
    fn find_unique_lock_key_pairs(&self) -> usize {
        let mut unique = 0;
        for lock in &self.locks {
            unique += self.find_keys_which_fit(lock);
        }
        unique
    }

    fn find_keys_which_fit(&self, lock: &[usize; 5]) -> usize {
        let mut keys = HashSet::new();
        for (c, lock_height) in lock.iter().enumerate() {
            let max_key_height = 5 - lock_height;
            let key_heights_for_column = self
                .keys
                .get(&c)
                .unwrap_or_else(|| panic!("expect height at {c}"));
            let mut key_height_which_fits = HashSet::new();
            for (key_height, keys) in key_heights_for_column {
                if *key_height > max_key_height {
                    break;
                }
                for k in keys {
                    key_height_which_fits.insert(*k);
                }
            }
            for key in &key_height_which_fits {
                if c == 0 {
                    keys.insert(*key);
                }
            }
            keys.retain(|k| key_height_which_fits.contains(k));
        }
        keys.len()
    }
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.trim().split("\n\n").collect::<Vec<&str>>();

        let mut locks: Vec<[usize; 5]> = vec![];
        let mut keys: BTreeMap<usize, BTreeMap<usize, HashSet<usize>>> = BTreeMap::new();
        for (i, part) in parts.iter().enumerate() {
            let grid = make_grid(part);
            let s = grid[0][0];
            let height_count = count_height(grid);
            match s {
                '#' => locks.push(height_count),
                '.' => update_keys(&mut keys, i, height_count),
                _ => return Err(anyhow!("{s} not mapped")),
            }
        }
        Ok(State { locks, keys })
    }
}

fn update_keys(
    keys: &mut BTreeMap<usize, BTreeMap<usize, HashSet<usize>>>,
    key_index: usize,
    key: [usize; 5],
) {
    for (c, k) in key.iter().enumerate() {
        let entry = keys.entry(c).or_default();
        entry
            .entry(*k)
            .and_modify(|s| {
                s.insert(key_index);
            })
            .or_insert_with(|| HashSet::from([key_index]));
    }
}

fn count_height(grid: Vec<Vec<char>>) -> [usize; 5] {
    let mut s = [0; 5];
    for row in grid {
        for (c, col) in row.iter().enumerate() {
            if *col == '#' {
                s[c] += 1;
            }
        }
    }
    for e in &mut s {
        *e -= 1;
    }
    s
}

fn make_grid(input: &str) -> Vec<Vec<char>> {
    let mut rows = vec![];
    for line in input.lines() {
        let mut cols = vec![];
        for c in line.chars() {
            cols.push(c);
        }
        assert_eq!(cols.len(), 5);
        rows.push(cols);
    }
    rows
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day25_test_data.txt")?;

        // Act
        let state = State::from_str(&input)?;

        let pairs = state.find_unique_lock_key_pairs();

        // Assert
        assert_eq!(pairs, 3);
        Ok(())
    }
}
