use anyhow::Result;
use std::{collections::BTreeMap, fs, str::FromStr};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day13_data.txt")?;

    let firewall = Firewall::from_str(&input)?;

    let severity = firewall.severity(0)?;

    println!("Part 1: {severity}"); // 1928

    let wait = firewall.wait()?;

    println!("Part 2: {wait}");

    Ok(())
}

#[derive(Clone)]
struct Layer {
    range: i32,
}

impl Layer {
    fn new(range: i32) -> Self {
        Self { range }
    }

    fn get_position(&self, debt: i32, shift: i32) -> Result<i32> {
        let total_debt = debt + shift;
        let turns = total_debt / self.range;
        let pow: u32 = turns.try_into()?;
        let direction = (-1_i32).pow(pow);
        let remainder = total_debt % self.range;
        let final_position = if direction == -1 {
            self.range - remainder
        } else {
            remainder
        };
        Ok(final_position)
    }
}

#[derive(Clone)]
struct Firewall {
    layers: BTreeMap<i32, Layer>,
}

impl Firewall {
    fn wait(&self) -> Result<i32> {
        let mut wait = 0;
        loop {
            let mut caught = false;
            for (debt, layer) in &self.layers {
                let final_position = layer.get_position(*debt, wait)?;
                if final_position == 0 {
                    caught = true;
                    break;
                }
            }
            if !caught {
                break;
            }
            wait += 1;
        }
        Ok(wait)
    }

    fn severity(&self, shift: i32) -> Result<i32> {
        let mut severity = 0;

        for (debt, layer) in &self.layers {
            let final_position = layer.get_position(*debt, shift)?;
            if final_position == 0 {
                severity += debt * (layer.range + 1);
            }
        }

        Ok(severity)
    }
}

impl FromStr for Firewall {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut firewall = BTreeMap::new();
        for line in s.lines() {
            let parts: Vec<i32> = line
                .split(": ")
                .map(|n| n.parse())
                .collect::<Result<Vec<i32>, _>>()?;
            assert_eq!(parts.len(), 2);
            firewall.insert(parts[0], Layer::new(parts[1] - 1));
        }
        Ok(Firewall { layers: firewall })
    }
}
