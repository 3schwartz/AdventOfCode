use anyhow::Ok;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day19_data.txt")?;

    let onsen = Onsen::from_str(&input)?;
    let possible = onsen.possible_designs();

    println!("Part 1: {possible}");

    Ok(())
}

use std::{collections::HashSet, str::FromStr};

struct Onsen {
    patterns: HashSet<String>,
    designs: Vec<String>,
}

impl Onsen {
    fn possible_designs(&self) -> u32 {
        let mut possible = 0;
        for design in &self.designs {
            if self.check_design(design) {
                possible += 1;
            }
        }
        possible
    }

    fn check_design(&self, design: &str) -> bool {
        if design == "" {
            return true;
        }
        for i in 1..=design.len() {
            let sub_design = &design[0..i];
            if !self.patterns.contains(sub_design) {
                continue;
            }
            if self.check_design(&design[i..]) {
                return true;
            }
        }

        false
    }
}

impl FromStr for Onsen {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.trim().split("\n\n").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);

        let patterns = parts[0]
            .split(", ")
            .map(|n| n.to_string())
            .collect::<HashSet<String>>();
        let designs = parts[1].lines().map(|n| n.to_string()).collect();

        Ok(Self { patterns, designs })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_splice() {
        let input = "hello";
        let part = &input[0..1];

        assert_eq!(part, "h");
        assert_eq!(&input[4..5], "o");
        assert_eq!(&input[5..5], "");
        assert_eq!(&input[5..], "");
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day19_test_data.txt")?;

        // Act
        let onsen = Onsen::from_str(&input)?;
        let possible = onsen.possible_designs();

        // Assert
        assert_eq!(possible, 6);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }
}
