use std::{num::ParseIntError, fs, collections::BTreeMap};

use anyhow::{Result, Context, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day23_data.txt")?;

    let mut nanobots = vec![];
    for line in input.lines() {
        nanobots.push(Nanobot::from(line)?)
    }
    let max = Nanobot::find_max(&nanobots)?;
    let in_range: u32 = nanobots
        .iter()
        .map(|other| max.is_in_range(other) as u32)
        .sum();
    
    println!("Part 1: {}", in_range);

    let part2 = find_best_location(&nanobots)?;

    println!("Part 2: {}", part2);

    Ok(())
}

/// Map 3-dim down to one by only consider the range of each bot.
/// At `bot_manhattan - radius` the bot is reachable and one exits its area at
/// `bot_manhattan + radius + 1`
/// Map all bots down to one dimension and then iterate from high to low accumulating
/// enterred bots in reachable area.
/// The area with highest amount of entered bots is the area one should be positioned.
fn find_best_location(bots: &Vec<Nanobot>) -> Result<i32> {
    let mut borders_count = BTreeMap::new();
    // Transform from 3-dim to 1 by consider min / max
    for bot in bots {
        let bot_manhattan = bot.x.abs() + bot.y.abs() + bot.z.abs();
        // enter into reachable
        *borders_count.entry(bot_manhattan - bot.r).or_insert(0) += 1;
        // exited reachable area
        *borders_count.entry(bot_manhattan + bot.r + 1).or_insert(0) -= 1;
    }

    // for each distance find the sum of entered / exited
    let graph = borders_count
        .iter()
        .scan(0, |within_area, (border, &enterred)| {
            *within_area += enterred;
            Some((border, *within_area))
        })
        .collect::<Vec<_>>();
    
    // Find the max enterred count. Within these borders we should search for a
    // solution.
    let max_entered = graph.iter()
        .map(|&(_, n)| n)
        .max()
        .ok_or_else(|| anyhow!("not able to find max in {:?}", graph))?;

    // Find areas between border which has max enterred count.
    let areas_between_borders = graph
        .iter()
        .zip(graph.iter().skip(1))
        .filter_map(
            |(&(low, enterred), &(high, _))| {
                if enterred == max_entered {
                    Some((*low, *high - 1))
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    // if any border with max enterred is on opposite side of origin then origin is place.
    let result = if areas_between_borders.iter().any(|&(low, high)| low <= 0 && high >= 0) {
        0
    } else {
        // Find the solution which is nearest origin.
        areas_between_borders
            .iter()
            .map(|&(low, high)| if high < 0 { -high } else { low })
            .min()
            .ok_or_else(|| anyhow!("not able to find min solution in {:?}", areas_between_borders))?
            .abs()
    };
    Ok(result)
}

#[derive(Debug)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

impl Nanobot {
    fn from(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split(">, r=").collect();
        let r: i32 = parts[1].parse()?;
        let coords = parts[0].trim_start_matches("pos=<");
        let coords_split = coords.split(',')
            .map(|i| i.parse::<i32>())
            .collect::<Vec<Result<i32, ParseIntError>>>()
            .into_iter()
            .collect::<Result<Vec<i32>, _>>()
            .with_context(|| format!("Failed to parse {:#}", coords))?;

        Ok(Self { x: coords_split[0], y: coords_split[1], z: coords_split[2], r })
    }

    fn find_max(bots: &Vec<Nanobot>) -> Result<&Nanobot> {
        bots
            .iter()
            .max_by_key(|obj| obj.r)
            .ok_or_else(|| anyhow!("not able to find max: {:?}", bots))
    }

    fn is_in_range(&self, other: &Nanobot) -> bool {
        self.is_in_range_coord(other.x, other.y, other.z)
    }

    fn is_in_range_coord(&self, x: i32, y: i32, z: i32) -> bool {
        let manhattan_distance = (self.x - x).abs() +
            (self.y - y).abs() + 
            (self.z - z).abs();
        manhattan_distance <= self.r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day23_test_data.txt")?;

        // Act
        let mut nanobots = vec![];
        for line in input.lines() {
            nanobots.push(Nanobot::from(line)?)
        }
        let max = Nanobot::find_max(&nanobots)?;
        let in_range: u32 = nanobots
            .iter()
            .map(|other| max.is_in_range(other) as u32)
            .sum();

        assert_eq!(7, in_range);
        Ok(())        
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day23_test2_data.txt")?;

        // Act
        let mut nanobots = vec![];
        for line in input.lines() {
            nanobots.push(Nanobot::from(line)?)
        }
        
        let part2 = find_best_location(&nanobots)?;

        assert_eq!(36, part2);
        Ok(())        
    }
}