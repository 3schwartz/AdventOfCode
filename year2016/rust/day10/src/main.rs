use anyhow::{anyhow, Result};
use std::{
    collections::{BTreeSet, HashMap},
    fs,
    str::FromStr,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day10_data.txt")?;
    let bots = parse(&input)?;

    let actual = run(bots, 17, 61)?;

    println!("Part 1: {}", actual.part_1);

    let part_2 = actual.part_2()?;
    println!("Part 2: {}", part_2);
    Ok(())
}

struct Solution {
    part_1: u8,
    part_2: HashMap<u8, Vec<i32>>,
}

impl Solution {
    fn part_2(self) -> Result<i32> {
        let idx = [0, 1, 2];
        let mut mult = 1;
        for id in idx {
            let out = self
                .part_2
                .get(&id)
                .ok_or_else(|| anyhow!("missing output: {}", id))?;
            if out.len() != 1 {
                return Err(anyhow!("wrong length for id: {}", id));
            }
            mult *= out[0]
        }
        Ok(mult)
    }
}

fn run(mut bots: HashMap<u8, Bot>, low_compare: i32, high_compare: i32) -> Result<Solution> {
    let bot_idx: Vec<u8> = bots.keys().copied().collect();
    let mut outputs: HashMap<u8, Vec<i32>> = HashMap::new();

    let mut part_1 = None;
    loop {
        let mut updated = false;
        for idx in &bot_idx {
            let mut bot = bots
                .get(idx)
                .ok_or_else(|| anyhow!("missing boot: {}", idx))?
                .clone();
            if bot.values.len() > 2 {
                return Err(anyhow!("boot {} had bad length {}", idx, bot.values.len()));
            }

            if bot.values.len() != 2 {
                continue;
            }
            let low_valid = validate(bot.low, bot.low_out, &bots)?;
            let high_valid = validate(bot.high, bot.high_out, &bots)?;
            if !low_valid || !high_valid {
                continue;
            }
            updated = true;

            let low_value = bot
                .values
                .pop_first()
                .ok_or_else(|| anyhow!("missing first"))?;
            let high_value = bot
                .values
                .pop_first()
                .ok_or_else(|| anyhow!("missing second"))?;

            if low_value > high_value {
                return Err(anyhow!(
                    "something went wrong: {}, {}",
                    low_value,
                    high_value
                ));
            }

            if low_value == low_compare && high_value == high_compare {
                part_1 = Some(*idx);
            }

            update(bot.low, bot.low_out, low_value, &mut outputs, &mut bots);
            update(bot.high, bot.high_out, high_value, &mut outputs, &mut bots);

            bots.insert(*idx, bot);
        }
        if !updated {
            break;
        }
    }
    Ok(Solution {
        part_1: part_1.ok_or_else(|| anyhow!("missing solution for part 1"))?,
        part_2: outputs,
    })
}

fn update(
    id: u8,
    output: Output,
    value: i32,
    outputs: &mut HashMap<u8, Vec<i32>>,
    bots: &mut HashMap<u8, Bot>,
) {
    match output {
        Output::Bot => {
            bots.entry(id).and_modify(|b| {
                b.values.insert(value);
            });
        }
        Output::Out => {
            outputs
                .entry(id)
                .and_modify(|v| v.push(value))
                .or_insert_with(|| vec![value]);
        }
    }
}

#[allow(dead_code)]
fn validate(id: u8, output: Output, bots: &HashMap<u8, Bot>) -> Result<bool> {
    match output {
        Output::Bot => bots
            .get(&id)
            .map(|b| b.values.len() < 2)
            .ok_or_else(|| anyhow!("missing boot {} when validating", id)),
        Output::Out => Ok(true),
    }
}

fn parse(input: &str) -> Result<HashMap<u8, Bot>> {
    let mut bots = HashMap::new();
    let mut initializations = vec![];
    for line in input.lines() {
        if line.starts_with("value") {
            initializations.push(line);
            continue;
        }
        let boot = Bot::from_str(line)?;
        bots.insert(boot.id, boot);
    }
    for initialization in initializations {
        // value 5 goes to bot 2
        let parts: Vec<&str> = initialization.split(' ').collect();
        if parts.len() != 6 {
            return Err(anyhow!("wrong length of init: {}", initialization));
        }
        if parts[4] != "bot" {
            return Err(anyhow!("can't map: {}", parts[4]));
        }
        let value: i32 = parts[1].parse()?;
        let boot: u8 = parts[5].parse()?;
        bots.entry(boot).and_modify(|b| {
            b.values.insert(value);
        });
    }
    Ok(bots)
}

#[derive(Clone, Copy)]
enum Output {
    Bot,
    Out,
}

impl FromStr for Output {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let f = match s {
            "bot" => Output::Bot,
            "output" => Output::Out,
            _ => return Err(anyhow!("can't map: {}", s)),
        };
        Ok(f)
    }
}

#[derive(Clone)]
struct Bot {
    id: u8,
    high_out: Output,
    low_out: Output,
    high: u8,
    low: u8,
    values: BTreeSet<i32>,
}

impl FromStr for Bot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // bot 2 gives low to bot 1 and high to bot 0
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 12 {
            return Err(anyhow!("wrong length: {}", s));
        }
        let id: u8 = parts[1].parse()?;

        let low_out = Output::from_str(parts[5])?;
        let low: u8 = parts[6].parse()?;

        let high_out = Output::from_str(parts[10])?;
        let high: u8 = parts[11].parse()?;

        Ok(Self {
            id,
            high_out,
            low_out,
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
        let actual = run(boots, 2, 5)?.part_1;

        // Assert
        assert_eq!(actual, 2);
        Ok(())
    }
}
