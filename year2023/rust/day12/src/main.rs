use std::{fs, collections::BTreeMap};
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day12_data.txt")?;
    let combinations = find_combinations(&input)?;

    println!("Part 1: {}", combinations);

    Ok(())
}

fn find_combinations(input: &str) -> Result<i32> {
    let mut cache = BTreeMap::new();
    
    let mut combinations = 0;
    for line in input.lines() {
        let state = State::from(&line)?;
        combinations += state.find_next_combinations(&mut cache);
    }

    Ok(combinations)
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone)]
struct State<'a> {
    // chars: Vec<char>,
    sequence: &'a str,
    groups: Vec<i32>,
}

impl<'a> State<'a> {
    fn from(input: &'a str) -> Result<Self> {
        let parts: Vec<&str> = input.split(" ")
            .collect();
        if parts.len() != 2 {
            return Err(anyhow!("{} not able to split by space", input));
        };
        let groups = parts[1]
            .split(",")
            .map(|c| c.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Self { sequence: parts[0], groups }
        )
    }

    /// Check group size is below total length.
    /// Also account for spacing between damaged groups.
    fn is_group_size_below_remaining(&self) -> bool {
        let left = self.sequence.len() as i32 - self.groups.iter().sum::<i32>() - self.groups.len() as i32 + 1;
        left < 0
    }

    /// Check if the next [self.chars[..self.groups[0]]] contains any
    /// operational, '.' elements.
    /// If it doesn't they can be used as a damage group.
    fn next_slice_damaged_or_unknown(&self) -> bool {
        !self.sequence[..self.groups[0] as usize].contains(".")
    }

    fn find_next_combinations<'b>(&self, cache: &'b mut BTreeMap<State<'a>, i32>) -> i32 {
        if let Some(c) = cache.get(self) {
            return *c;
        };
        // When no groups are left check if there are none
        // damage left.
        if self.groups.is_empty() {
            return !self.sequence.contains("#") as i32
        }
        if self.is_group_size_below_remaining() {
            return 0;
        }
        let next_slice_damaged_or_unknown = self.next_slice_damaged_or_unknown();
        if self.sequence.len() as i32 == self.groups[0] {
            return next_slice_damaged_or_unknown as i32;
        }

        // When first isn't '#' it can be either '.' or a '?' which
        // can be used as a '?'.
        let first = if &self.sequence[0..1] != "#" {
            let c = &self
                .sequence[1..]
                .trim_start_matches(".");
            let n = State{
                sequence: c,
                groups: self.groups.clone()
            };
            n.find_next_combinations(cache)
        } else {
            0
        };
        // When the first is damage, '#' or unknown '?' then we can 
        // check the combinations with the subsequent slice with lenght
        // of the first group.
        // In this case self.chars[self.groups[0]] needs to be '.' or '?' as a 
        // '.' to have a break between groups.
        let second = if next_slice_damaged_or_unknown &&
         &self.sequence[self.groups[0] as usize..self.groups[0] as usize + 1] != "#" {
            let c = &self.sequence[self.groups[0] as usize + 1..]
                .trim_start_matches(".");
            let v = self.groups
                .iter()
                .skip(1)
                .copied()
                .collect();
            let s = State {
                sequence: c,
                groups: v
            };
            s.find_next_combinations(cache)
        } else {
            0
        };
        let combinations = first + second;
        cache.insert(self.clone(), combinations);
        combinations

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_is_equal() {
        assert_eq!(vec![1,5,2], vec![1,5,2]);
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day12_data_test.txt")?;
        
        // Act
        let combinations = find_combinations(&input)?;

        // Assert
        assert_eq!(combinations, 21);
        Ok(())
    }

    #[test]
    fn test_lines() -> Result<()> {
        // Arrange
        let input = "?###???????? 3,2,1";
        let mut cache = BTreeMap::new();
        
        // Act
        let state = State::from(&input)?;
        let actual = state.find_next_combinations(&mut cache);

        // Assert
        assert_eq!(actual, 10);
        Ok(())

    }
}
