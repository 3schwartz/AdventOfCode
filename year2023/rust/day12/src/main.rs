use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{fs, collections::BTreeMap};
    use anyhow::{anyhow, Ok};

    use super::*;

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


        /// When no groups are left check if there are none
        /// damage left.
        fn group_empty(&self) -> Option<i32> {
            if self.groups.is_empty() {
                return Some(self.sequence.contains("#") as i32)
            }
            None
        }
        /// Check group size is below total length.
        /// Also account for spacing between damaged groups.
        fn is_check_group_size_above_remaining(&self) -> bool {
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
            if let Some(r) = self.group_empty() {
                return r;
            }
            if self.is_check_group_size_above_remaining() {
                return 0;
            }
            let next_slice_damaged_or_unknown = self.next_slice_damaged_or_unknown();
            if self.sequence.len() as i32 == self.groups[0] {
                return next_slice_damaged_or_unknown as i32;
            }

            // When first isn't '#' it can be either '.' or a '?' which
            // can be used as a '?'.
            let first = if &self.sequence[0..1] != "#" {
                let c = &self.sequence[1..];
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
                let c = &self.sequence[self.groups[0] as usize + 1..];
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


    #[test]
    fn vec_is_equal() {
        assert_eq!(vec![1,5,2], vec![1,5,2]);
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day12_data_test.txt")?;
        let mut cache = BTreeMap::new();
        // Act
        let mut combinations = 0;
        for line in input.lines() {
            let state = State::from(&line)?;
            combinations += state.find_next_combinations(&mut cache);
        }

        // Assert
        assert_eq!(combinations, 21);
        Ok(())
    }
}
