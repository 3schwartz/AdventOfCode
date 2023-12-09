use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day5_data.txt")?;

    let info = Info::from(&input)?;

    let min_location = info.find_min_location()?;
    println!("Part 1: {}", min_location);

    let range_min_location = info.find_range_min_location()?;

    println!("Part 2: {}", range_min_location);

    Ok(())
}

#[derive(Debug)]
struct Interval {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

#[derive(Debug)]
struct IntervalMapping {
    mapped: Option<(u64, u64)>,
    outside: BTreeSet<(u64, u64)>,
}

impl Interval {
    fn map(&self, source: u64) -> Option<u64> {
        if source < self.source_start || source > self.source_start + self.range {
            return None;
        }
        let destination = self.destination_start + source - self.source_start;
        Some(destination)
    }

    fn map_range(&self, range: (u64, u64)) -> Result<IntervalMapping> {
        // Outside
        // )(
        if range.1 < self.source_start || self.source_start + self.range < range.0 {
            return Ok(IntervalMapping {
                outside: BTreeSet::from([range]),
                mapped: None,
            });
        }
        let shift = range.0 as i64 - self.source_start as i64;
        let source_range = range.1 - range.0;
        // []
        if self.source_start <= range.0 && range.1 <= self.source_start + self.range {
            let start = self.destination_start + shift as u64;
            let end = start + source_range;
            return Ok(IntervalMapping {
                mapped: Some((start, end)),
                outside: BTreeSet::new(),
            });
        }
        // (->) + []
        if range.0 < self.source_start && range.1 <= self.source_start + self.range {
            return Ok(IntervalMapping {
                mapped: Some((
                    self.destination_start,
                    self.destination_start + source_range,
                )),
                outside: BTreeSet::from([(range.0, self.source_start - 1)]),
            });
        }
        // [] + (<-)
        if self.source_start <= range.0 && self.source_start + self.range < range.1 {
            let start = self.destination_start + shift as u64;
            return Ok(IntervalMapping {
                mapped: Some((start, self.destination_start + self.range)),
                outside: BTreeSet::from([(self.source_start + self.range + 1, range.1)]),
            });
        }
        // (->) + [] + (<-)
        if range.0 < self.source_start && self.source_start + self.range < range.1 {
            return Ok(IntervalMapping {
                mapped: Some((self.destination_start, self.destination_start + self.range)),
                outside: BTreeSet::from([
                    (range.0, self.source_start - 1),
                    (self.source_start + self.range + 1, range.1),
                ]),
            });
        }
        Err(anyhow!("({:?})", range))
    }
}

struct Info<'a> {
    maps: HashMap<&'a str, Vec<Interval>>,
    seeds: Vec<u64>,
    lookup: HashMap<&'a str, &'a str>,
}

impl<'a> Info<'a> {
    fn find_min_location(&self) -> Result<u64> {
        let mut min_location = u64::MAX;
        let mut cache = HashMap::new();
        for seed in &self.seeds {
            let location = self.find_location("seed", *seed, &mut cache)?;
            min_location = std::cmp::min(min_location, location);
        }
        Ok(min_location)
    }

    fn find_range_min_location(&self) -> Result<u64> {
        let mut min_location = u64::MAX;
        let mut cache = HashMap::new();
        for i in 0..self.seeds.len() / 2 {
            let idx = i * 2;
            println!("Position {}", idx);
            for shift in 0..self.seeds[idx + 1] {
                let seed = self.seeds[idx] + shift;
                let location = self.find_location("seed", seed, &mut cache)?;
                min_location = std::cmp::min(min_location, location);
            }
        }
        Ok(min_location)
    }

    fn find_range_min_location_optimal(&self) -> Result<u64> {
        let seed_ranges = self.make_ranges();
        println!("{:?}", seed_ranges);

        let mut min_location = u64::MAX;
        for seed in seed_ranges {
            let location = self.find_location_optimal("seed", BTreeSet::from([seed]))?;
            min_location = std::cmp::min(min_location, location);
        }
        Ok(min_location)
    }

    fn make_ranges(&self) -> BTreeSet<(u64, u64)> {
        let mut ranges = BTreeSet::new();
        for i in 0..self.seeds.len() / 2 {
            let idx = i * 2;
            let count = self.seeds[idx + 1] - 1;
            let start = self.seeds[idx];
            ranges.insert((start, start + count));
        }
        return Info::make_non_overlap(ranges);
    }

    fn make_non_overlap(mut ranges: BTreeSet<(u64, u64)>) -> BTreeSet<(u64, u64)> {
        loop {
            let mut temp: BTreeSet<(u64, u64)> = BTreeSet::new();
            for range in &ranges {
                let mut drop = false;
                for r in &ranges {
                    if range == r {
                        continue;
                    }
                    if r.0 <= range.0 && range.1 <= r.1 {
                        temp.insert(*r);
                        drop = true;
                        break;
                    }
                    if r.0 <= range.0 && range.0 <= r.1 {
                        temp.insert((r.0, range.1));
                        drop = true;
                        break;
                    }
                }
                if !drop {
                    temp.insert(*range);
                }
            }
            if ranges == temp {
                return temp;
            }
            ranges = temp;
        }
    }

    fn find_location_optimal(&self, kind: &str, source: BTreeSet<(u64, u64)>) -> Result<u64> {
        println!("############### {} ###########", kind);
        let ranges = self
            .maps
            .get(kind)
            .ok_or_else(|| anyhow!("{} should be in maps", kind))?;

        let dest = Info::find_destination_ranges(source, ranges)?;

        let new_kind = *self
            .lookup
            .get(kind)
            .ok_or_else(|| anyhow!("{} should be in lookups", kind))?;

        if new_kind == "location" {
            println!("This is it: {:?}", dest);
            let dest_min = dest
                .iter()
                .map(|(s, _)| *s)
                .min()
                .ok_or_else(|| anyhow!("there should be a min"))?;
            return Ok(dest_min);
        }

        self.find_location_optimal(new_kind, dest)
    }

    fn find_destination_ranges(
        ranges: BTreeSet<(u64, u64)>,
        intervals: &Vec<Interval>,
    ) -> Result<BTreeSet<(u64, u64)>> {
        let mut outside = ranges;
        let mut mapped = BTreeSet::new();
        loop {
            let mut temp = BTreeSet::new();
            for range in &outside {
                for interval in intervals {
                    let mut mapping = interval.map_range(*range)?;
                    println!("{:?}", range);
                    println!("{:?}", interval);
                    println!("{:?}", mapping);
                    if let Some(m) = mapping.mapped {
                        mapped.insert(m);
                    }
                    temp.append(&mut mapping.outside);
                }
            }
            if temp == outside {
                mapped.append(&mut outside);
                break;
            }
            outside = temp;
        }
        println!("{:?}", mapped);
        Ok(Info::make_non_overlap(mapped))
    }

    fn find_location<'b>(&'a self, kind: &'a str, source: u64, cache: &'b mut HashMap<(&'a str, u64), u64>) -> Result<u64> {
        let ranges = self
            .maps
            .get(kind)
            .ok_or_else(|| anyhow!("{} should be in maps", kind))?;

        let dest = Info::find_destination(source, ranges);

        let new_kind = *self
            .lookup
            .get(kind)
            .ok_or_else(|| anyhow!("{} should be in lookups", kind))?;

        if new_kind == "location" {
            return Ok(dest);
        }

        let location = self.find_location(new_kind, dest, cache)?;
        cache.insert((kind, source), location);
        Ok(location)
    }

    fn find_destination(source: u64, ranges: &Vec<Interval>) -> u64 {
        for interval in ranges {
            if let Some(destination) = interval.map(source) {
                return destination;
            }
        }
        source
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
                    .flat_map(|l| {
                        l.split_whitespace()
                            .map(|c| c.parse::<u64>())
                            .filter_map(|c| c.ok())
                    })
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
                .collect::<Result<Vec<u64>, _>>()?;
            let destination = parts[0];
            let source = parts[1];
            let range = parts[2];
            let from_entry = maps
                .entry(from.ok_or_else(|| anyhow!("from should be set"))?)
                .or_insert_with(Vec::new);
            from_entry.push(Interval {
                source_start: source,
                destination_start: destination,
                range: range - 1,
            });
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

    // #[test]
    fn test_find_destination_ranges_water_to_light() -> Result<()> {
        // Act
        let actual = Info::find_destination_ranges(
            BTreeSet::from([(51,69)]),
            &vec![
                Interval {
                source_start: 18,
                destination_start: 88,
                range: 6,
            },
            Interval {
                source_start: 25,
                destination_start: 18,
                range: 69,
            }],
        )?;

        // Assert
        assert_eq!(actual, BTreeSet::from([(44,62)]));
        Ok(())
    }

    #[test]
    fn test_find_destination_ranges_fertilizer_to_water() -> Result<()> {
        // Act
        let actual = Info::find_destination_ranges(
            BTreeSet::from([(55,69)]),
            &vec![
                Interval {
                source_start: 53,
                destination_start: 49,
                range: 7,
            },
            Interval {
                source_start: 11,
                destination_start: 0,
                range: 41,
            },
            Interval {
                source_start: 0,
                destination_start: 42,
                range: 6,
            },
            Interval {
                source_start: 7,
                destination_start: 57,
                range: 3,
            }],
        )?;

        // Assert
        assert_eq!(actual, BTreeSet::from([(51,69)]));
        Ok(())
    }

    #[test]
    fn test_find_destination_ranges_soil_to_fertilizer() -> Result<()> {
        // Act
        let actual = Info::find_destination_ranges(
            BTreeSet::from([(55,69)]),
            &vec![
                Interval {
                source_start: 15,
                destination_start: 0,
                range: 36,
            },
            Interval {
                source_start: 52,
                destination_start: 37,
                range: 1,
            },
            Interval {
                source_start: 0,
                destination_start: 39,
                range: 14,
            }],
        )?;

        // Assert
        assert_eq!(actual, BTreeSet::from([(55,69)]));
        Ok(())
    }

    #[test]
    fn test_find_destination_ranges_seed_to_soil() -> Result<()> {
        // Act
        let actual = Info::find_destination_ranges(
            BTreeSet::from([(55, 67)]),
            &vec![
                Interval {
                source_start: 98,
                destination_start: 50,
                range: 1,
            },
            Interval {
                source_start: 50,
                destination_start: 52,
                range: 47,
            }],
        )?;

        // Assert
        assert_eq!(actual, BTreeSet::from([(55,69)]));
        Ok(())
    }

    #[test]
    fn test_non_overlap() {
        // Arrange
        let ranges = BTreeSet::from([(1, 5), (3, 9), (2, 4), (11, 67)]);

        // Act
        let actual = Info::make_non_overlap(ranges);

        // Assert
        assert_eq!(actual, BTreeSet::from([(1, 9), (11, 67)]));
    }

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

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day5_data_test.txt")?;
        let info = Info::from(&input)?;

        // Act
        let min_location = info.find_range_min_location()?;

        // Assert
        assert_eq!(min_location, 46);
        Ok(())
    }

    // #[test]
    fn test_part_2_optimal() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day5_data_test.txt")?;
        let info = Info::from(&input)?;

        // Act
        let min_location = info.find_range_min_location_optimal()?;

        // Assert
        assert_eq!(min_location, 46);
        Ok(())
    }
}
