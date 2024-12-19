use anyhow::Ok;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day19_data.txt")?;

    let onsen = Onsen::from(input.as_str());

    let (possible, total) = onsen.possible_designs();

    println!("Part 1: {possible}");
    println!("Part 2: {total}");

    Ok(())
}

struct Onsen<'a> {
    patterns: HashSet<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> Onsen<'a> {
    fn possible_designs(&self) -> (u64, u64) {
        let mut possible = 0;
        let mut total_possible = 0;
        for design in &self.designs {
            let mut cache = HashMap::new();
            let p = self.check_design(design, &mut cache);
            total_possible += p;
            if p != 0 {
                possible += 1;
            }
        }
        (possible, total_possible)
    }

    fn check_design(&self, design: &'a str, cache: &mut HashMap<&'a str, u64>) -> u64 {
        if design.is_empty() {
            return 1;
        }
        if let Some(c) = cache.get(design) {
            return *c;
        }
        let mut possible = 0;
        for i in 1..=design.len() {
            let sub_design = &design[0..i];
            if !self.patterns.contains(sub_design) {
                continue;
            }
            let e = self.check_design(&design[i..], cache);
            cache.insert(&design[i..], e);

            possible += e;
        }
        possible
    }
}

impl<'a> From<&'a str> for Onsen<'a> {
    fn from(value: &'a str) -> Self {
        let parts = value.trim().split("\n\n").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);

        let patterns = parts[0].split(", ").collect::<HashSet<&str>>();
        let designs = parts[1].lines().collect();

        Self { patterns, designs }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day19_test_data.txt")?;

        // Act
        let onsen = Onsen::from(input.as_str());
        let possible = onsen.possible_designs();

        // Assert
        assert_eq!(possible.0, 6);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day19_test_data.txt")?;

        // Act
        let onsen = Onsen::from(input.as_str());
        let (possible, total) = onsen.possible_designs();

        // Assert
        assert_eq!(possible, 6);
        assert_eq!(total, 16);
        Ok(())
    }
}
