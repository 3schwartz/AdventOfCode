use std::{fs, collections::HashMap};

use anyhow::{Result, anyhow};

#[derive(Debug)]
struct Deer {
    move_distance: u64,
    rest_time: u64,
    move_time: u64,
}

impl Deer {
    fn new (parts: Vec<&str>) -> Result<Self> {
        let move_distance: u64 = parts.get(3).ok_or_else(|| anyhow!("move distance: {:?}", parts))?
            .parse()?;
        let move_time: u64 = parts.get(6).ok_or_else(|| anyhow!("move time: {:?}", parts))?
            .parse()?;
        let rest_time: u64 = parts.get(13).ok_or_else(|| anyhow!("rest time: {:?}", parts))?
            .parse()?;
        Ok(Self { move_distance, rest_time, move_time })
    }

    fn get_distance(&self, seconds: u64) -> u64 {
        let cycles = seconds / (self.move_time + self.rest_time);
        let remainder = seconds % (self.move_time + self.rest_time);

        let cycles_movement = cycles * (self.move_distance * self.move_time);

        let remainder_time = if self.move_time > remainder { remainder } else { self.move_time };
        
        return  cycles_movement + remainder_time * self.move_distance;
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day14_data.txt")?;

    let mut deers = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        deers.push(Deer::new(parts)?);
    }

    let part_1 = deers
            .iter()
            .map(|d| d.get_distance(2503))
            .max()
            .ok_or_else(|| anyhow!("not able to get max: {:?}", deers))?;

    println!("Part 1: {}", part_1);

    let mut points = HashMap::new();

    for i in 1..=2503u64 {
        let distances: Vec<u64> = deers
            .iter()
            .map(|d| d.get_distance(i))
            .collect();

        let max = distances.iter()
            .max()
            .ok_or_else(|| anyhow!("not able to get max: {:?}", deers))?;

        for (usize, distance) in distances.iter().enumerate() {
            if max == distance {
                let point = points.entry(usize)
                    .or_insert(0);
                *point+=1;
            }
        }
    }

    let part_2 = points
        .values()
        .max()
        .ok_or_else(|| anyhow!("not able to get max: {:?}", deers))?;

    println!("Part 2: {}", part_2);

    Ok(())
}
