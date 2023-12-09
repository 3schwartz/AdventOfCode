use std::{collections::HashMap, fs};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day5_data.txt")?;

    let info = Info::from(&input)?;
    let min_location = info.find_min_location()?;

    println!("Part 1: {}", min_location);
    Ok(())
}

struct Interval {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

impl Interval {
    fn map(&self, source: u64) -> Option<u64> {
        if source < self.source_start || source > self.source_start + self.range {
            return None;
        }
        let destination = self.destination_start + source - self.source_start;
        Some(destination)
    }
}

struct Info<'a> {
    maps: HashMap<&'a str, Vec<Interval>>,
    seeds: Vec<u64>,
    lookup: HashMap<&'a str, &'a str>,
}

impl<'a> Info<'a> {

    fn find_destination(source: u64, ranges: &Vec<Interval>) -> u64 {
        for interval in ranges {
            if let Some(destination) = interval.map(source) {
                return destination;
            }
        }
        source
    }

    fn find_location(
        &self,
        kind: &str,
        source: u64,
    ) -> Result<u64> {
        let ranges = *&self.maps
            .get(kind)
            .ok_or_else(|| anyhow!("{} should be in maps", kind))?;

        let dest = Info::find_destination(source, ranges);
    
        let new_kind = *self.lookup
            .get(kind)
            .ok_or_else(|| anyhow!("{} should be in lookups", kind))?;
    
        if new_kind == "location" {
            return Ok(dest);
        }
    
        self.find_location(new_kind, dest)
    }

    fn find_min_location(&self) -> Result<u64> {
        let mut min_location = u64::MAX;
        for seed in &self.seeds {
            let location = self.find_location("seed", *seed)?;
            min_location = std::cmp::min(min_location, location);
        }
        Ok(min_location)
    }

    fn from(input: &str) -> Result<Info> {
        let mut lookup: HashMap<&str, &str> = HashMap::new();
        let mut maps: HashMap<&str, Vec<Interval>> = HashMap::new();
        let mut seeds: Vec<u64> = vec![];
        let mut from = None;
        for (idx, line) in input.lines().enumerate() {
            if idx == 0 {
                seeds = line
                    .split(": ")
                    .map(|l| {
                        l.split_whitespace()
                            .map(|c| c.parse::<u64>())
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
            let parts: Vec<u64> = line
                .split_whitespace()
                .map(|c| c.parse::<u64>())
                .into_iter()
                .collect::<Result<Vec<u64>, _>>()?;
            let destination = parts[0];
            let source = parts[1];
            let range = parts[2];
            let from_entry = maps
                .entry(from.ok_or_else(|| anyhow!("from should be set"))?)
                .or_insert_with(|| vec![]);
            from_entry.push(Interval { source_start: source, destination_start: destination, range });
        }
        Ok(Info {
            maps,
            seeds,
            lookup,
        })
    }
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