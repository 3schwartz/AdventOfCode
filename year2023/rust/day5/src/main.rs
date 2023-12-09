use std::{collections::HashMap, fs};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day5_data.txt")?;

    let info = Info::from(&input)?;
    let min_location = info.find_min_location()?;

    println!("Part 1: {}", min_location);
    Ok(())
}

struct Info<'a> {
    maps: HashMap<&'a str, HashMap<u32, u32>>,
    seeds: Vec<u32>,
    lookup: HashMap<&'a str, &'a str>,
}

impl<'a> Info<'a> {
    fn find_min_location(&self) -> Result<u32> {
        let mut min_location = u32::MAX;
        for seed in &self.seeds {
            let location = find_location("seed", *seed, &self.maps, &self.lookup)?;
            min_location = std::cmp::min(min_location, location);
        }
        Ok(min_location)
    }

    fn from(input: &str) -> Result<Info> {
        let mut lookup: HashMap<&str, &str> = HashMap::new();
        let mut maps: HashMap<&str, HashMap<u32, u32>> = HashMap::new();
        let mut seeds: Vec<u32> = vec![];
        let mut from = None;
        for (idx, line) in input.lines().enumerate() {
            if idx == 0 {
                seeds = line
                    .split(": ")
                    .map(|l| {
                        l.split_whitespace()
                            .map(|c| c.parse::<u32>())
                            .filter_map(|c| c.ok())
                    })
                    .flatten()
                    .collect();
                continue;
            }
            if line.is_empty() {
                continue;
            }
            if line.ends_with(" map:") {
                let parts: Vec<&str> = line.trim_end_matches(" map:").split("-to-").collect();
                lookup.insert(parts[0], parts[1]);
                from = Some(parts[0]);
                continue;
            }
            let parts: Vec<u32> = line
                .split_whitespace()
                .map(|c| c.parse::<u32>())
                .into_iter()
                .collect::<Result<Vec<u32>, _>>()?;
            let destination = parts[0];
            let source = parts[1];
            let range = parts[2];
            let from_entry = maps
                .entry(from.ok_or_else(|| anyhow!("from should be set"))?)
                .or_insert_with(|| HashMap::new());
            for i in 0..range {
                let d = destination + i;
                let s = source + i;
                from_entry.insert(s, d);
            }
        }
        Ok(Info {
            maps,
            seeds,
            lookup,
        })
    }
}

fn find_location(
    kind: &str,
    source: u32,
    maps: &HashMap<&str, HashMap<u32, u32>>,
    lookup: &HashMap<&str, &str>,
) -> Result<u32> {
    let range = maps
        .get(kind)
        .ok_or_else(|| anyhow!("{} should be in maps", kind))?;

    let dest = *range.get(&source).unwrap_or(&source);

    let new_kind = *lookup
        .get(kind)
        .ok_or_else(|| anyhow!("{} should be in lookups", kind))?;

    if new_kind == "location" {
        return Ok(dest);
    }

    find_location(new_kind, dest, maps, lookup)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day5_data_test.txt")?;
        let info = Info::from(&input)?;

        // Act
        let min_location = info.find_min_location()?;

        // Assert
        assert_eq!(min_location, 35);
        Ok(())
    }
}