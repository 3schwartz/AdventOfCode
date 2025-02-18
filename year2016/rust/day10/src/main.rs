use anyhow::{anyhow, Result};
use std::{
    collections::{BTreeSet, HashMap},
    fs,
    str::FromStr,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day10_data.txt")?;
    let boots = parse(&input)?;

    let actual = run(boots, 17, 61)?;

    println!("Part 1: {}", actual);
    Ok(())
}

fn run(mut boots: HashMap<u8, Boot>, low_compare: i32, high_compare: i32) -> Result<u8> {
    let boot_idx: Vec<u8> = boots.keys().map(|k| *k).collect();

    loop {
        for idx in &boot_idx {
            let mut boot = boots
                .get(&idx)
                .ok_or_else(|| anyhow!("missing boot: {}", idx))?
                .clone();
            if boot.values.len() > 2 {
                return Err(anyhow!("boot {} had bad length {}", idx, boot.values.len()));
            }
            if boot.values.len() != 2 {
                continue;
            }
            let low_value = boot
                .values
                .pop_first()
                .ok_or_else(|| anyhow!("missing first"))?;
            let high_value = boot
                .values
                .pop_first()
                .ok_or_else(|| anyhow!("missing second"))?;

            if low_value == low_compare && high_value == high_compare {
                return Ok(*idx);
            }

            boots.entry(boot.low).and_modify(|b| {
                b.values.insert(low_value);
            });

            boots.entry(boot.high).and_modify(|b| {
                b.values.insert(high_value);
            });

            boots.insert(*idx, boot);
        }
    }
}

fn parse(input: &str) -> Result<HashMap<u8, Boot>> {
    let mut boots = HashMap::new();
    let mut initializations = vec![];
    for line in input.lines() {
        if line.starts_with("value") {
            initializations.push(line);
            continue;
        }
        let boot = Boot::from_str(line)?;
        boots.insert(boot.id, boot);
    }
    for initialization in initializations {
        // value 5 goes to bot 2
        let parts: Vec<&str> = initialization.split(' ').collect();
        if parts.len() != 6 {
            return Err(anyhow!("wrong length of init: {}", initialization));
        }
        let value: i32 = parts[1].parse()?;
        let boot: u8 = parts[5].parse()?;
        boots.entry(boot).and_modify(|b| {
            b.values.insert(value);
        });
    }
    Ok(boots)
}

#[derive(Clone)]
struct Boot {
    id: u8,
    high: u8,
    low: u8,
    values: BTreeSet<i32>,
}

impl FromStr for Boot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // bot 2 gives low to bot 1 and high to bot 0
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 12 {
            return Err(anyhow!("wrong length: {}", s));
        }
        let id: u8 = parts[1].parse()?;
        let high: u8 = parts[6].parse()?;
        let low: u8 = parts[11].parse()?;
        Ok(Self {
            id,
            high,
            low,
            values: BTreeSet::new(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day10_test_data.txt")?;
        let boots = parse(&input)?;
        // Act
        let actual = run(boots, 2, 5)?;

        // Assert
        assert_eq!(actual, 2);
        Ok(())
    }
}

// --- Day 10: Balance Bots ---

// You come upon a factory in which many robots are zooming around handing small microchips to each other.

// Upon closer examination, you notice that each bot only proceeds when it has two microchips, and once it does, it gives each one to a different bot or puts it in a marked "output" bin. Sometimes, bots take microchips from "input" bins, too.

// Inspecting one of the microchips, it seems like they each contain a single number; the bots must use some logic to decide what to do with each chip. You access the local control computer and download the bots' instructions (your puzzle input).

// Some of the instructions specify that a specific-valued microchip should be given to a specific bot; the rest of the instructions indicate what a given bot should do with its lower-value or higher-value chip.

// For example, consider the following instructions:

// value 5 goes to bot 2
// bot 2 gives low to bot 1 and high to bot 0
// value 3 goes to bot 1
// bot 1 gives low to output 1 and high to bot 0
// bot 0 gives low to output 2 and high to output 0
// value 2 goes to bot 2
// Initially, bot 1 starts with a value-3 chip, and bot 2 starts with a value-2 chip and a value-5 chip.
// Because bot 2 has two microchips, it gives its lower one (2) to bot 1 and its higher one (5) to bot 0.
// Then, bot 1 has two microchips; it puts the value-2 chip in output 1 and gives the value-3 chip to bot 0.
// Finally, bot 0 has two microchips; it puts the 3 in output 2 and the 5 in output 0.
// In the end, output bin 0 contains a value-5 microchip, output bin 1 contains a value-2 microchip, and output bin 2 contains a value-3 microchip. In this configuration, bot number 2 is responsible for comparing value-5 microchips with value-2 microchips.

// Based on your instructions, what is the number of the bot that is responsible for comparing value-61 microchips with value-17 microchips?
