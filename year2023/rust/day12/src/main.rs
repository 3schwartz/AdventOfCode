use std::{fs, collections::BTreeMap};
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day12_data.txt")?;
    let part_1 = find_combinations(&input, 1)?;

    println!("Part 1: {}", part_1);

    let part_2 = find_combinations(&input, 5)?;
    println!("Part 2: {}", part_2);

    Ok(())
}

fn find_combinations(input: &str, count: u32) -> Result<i64> {
    let mut cache = BTreeMap::new();
    
    let mut combinations = 0;
    for line in input.lines() {
        let state = State::from(&line, count)?;
        combinations += state.find_next_combinations(&mut cache);
    }

    Ok(combinations)
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone)]
struct State {
    sequence: String,
    groups: Vec<i64>,
}

impl State {
    fn from(input: &str, repeats: u32) -> Result<Self> {
        let parts: Vec<&str> = input.split(" ")
            .collect();
        if parts.len() != 2 {
            return Err(anyhow!("{} not able to split by space", input));
        };
        let group = parts[1]
            .split(",")
            .map(|c| c.parse())
            .collect::<Result<Vec<i64>, _>>()?;
        let mut sequences = vec![];
        let mut g = vec![];
        let mut count = 0;
        while count < repeats {
            g.push(group.clone());
            sequences.push(parts[0]);
            count+=1;
        }

        let groups = g.iter().flatten().map(|c| *c).collect();
        let sequence = sequences.join("?");

        Ok(Self { sequence, groups }
        )
    }

    /// Check group size is below total length.
    /// Also account for spacing between damaged groups.
    fn is_group_size_below_remaining(&self) -> bool {
        let left = self.sequence.len() as i64 - self.groups.iter().sum::<i64>() - self.groups.len() as i64 + 1;
        left < 0
    }

    /// Check if the next [self.chars[..self.groups[0]]] contains any
    /// operational, '.' elements.
    /// If it doesn't they can be used as a damage group.
    fn next_slice_damaged_or_unknown(&self) -> bool {
        !self.sequence[..self.groups[0] as usize].contains(".")
    }

    fn find_next_combinations<'b>(&self, cache: &'b mut BTreeMap<State, i64>) -> i64 {
        if let Some(c) = cache.get(self) {
            return *c;
        };
        // When no groups are left check if there are none
        // damage left.
        if self.groups.is_empty() {
            return !self.sequence.contains("#") as i64
        }
        if self.is_group_size_below_remaining() {
            return 0;
        }
        let next_slice_damaged_or_unknown = self.next_slice_damaged_or_unknown();
        if self.sequence.len() as i64 == self.groups[0] {
            return next_slice_damaged_or_unknown as i64;
        }

        // When first isn't '#' it can be either '.' or a '?' which
        // can be used as a '?'.
        let first = if &self.sequence[0..1] != "#" {
            let c = &self
                .sequence[1..]
                .trim_start_matches(".");
            let n = State{
                sequence: c.to_string(),
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
                sequence: c.to_string(),
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
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day12_data_test.txt")?;
        
        // Act
        let combinations = find_combinations(&input, 5)?;

        // Assert
        assert_eq!(combinations, 525152);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day12_data_test.txt")?;
        
        // Act
        let combinations = find_combinations(&input, 1)?;

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
        let state = State::from(&input, 1)?;
        let actual = state.find_next_combinations(&mut cache);

        // Assert
        assert_eq!(actual, 10);
        Ok(())

    }
}
