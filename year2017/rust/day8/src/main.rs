use std::{collections::HashMap, fs};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day8_data.txt")?;

    let mut instructions = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 7 {
            return Err(anyhow!(line.to_string()));
        }
        instructions.push(Instruction {
            registry: parts[0].to_string(),
            action: Action::from(parts[1])?,
            value: parts[2].parse()?,
            registry_condition: parts[4].to_string(),
            condition: Condition::from(parts[5])?,
            value_condition: parts[6].parse()?,
        });
    }

    let mut registries: HashMap<String, i32> = HashMap::new();
    let mut max_all = i32::MIN;
    for instruction in instructions {
        if !instruction.satisfy_conditon(&registries) {
            continue;
        }
        instruction.update_registry(&mut registries);
        let current_max = *registries.values().max().unwrap();
        max_all = std::cmp::max(current_max, max_all);
    }

    let last_max = registries.values().max().unwrap();
    println!("Part 1 {}", last_max);
    println!("Part 2 {}", max_all);

    Ok(())
}

enum Condition {
    Below,
    BelowOrEqual,
    Above,
    AboveOrEqual,
    NotEqual,
    Equal,
}

impl Condition {
    fn from(input: &str) -> Result<Self> {
        let condition = match input {
            "<" => Condition::Below,
            "<=" => Condition::BelowOrEqual,
            ">" => Condition::Above,
            ">=" => Condition::AboveOrEqual,
            "!=" => Condition::NotEqual,
            "==" => Condition::Equal,
            _ => return Err(anyhow!(input.to_string())),
        };
        return Ok(condition);
    }
}

enum Action {
    Inc,
    Dec,
}

impl Action {
    fn from(input: &str) -> Result<Self> {
        let action = match input {
            "inc" => Action::Inc,
            "dec" => Action::Dec,
            _ => return Err(anyhow!(input.to_string())),
        };
        Ok(action)
    }
}

struct Instruction {
    registry: String,
    action: Action,
    value: i32,
    registry_condition: String,
    condition: Condition,
    value_condition: i32,
}

impl Instruction {
    fn update_registry(&self, registries: &mut HashMap<String, i32>) {
        let registry = registries.entry(self.registry.clone()).or_insert(0);
        let delta = match self.action {
            Action::Inc => self.value,
            Action::Dec => -self.value,
        };
        *registry += delta;
    }

    fn satisfy_conditon(&self, registries: &HashMap<String, i32>) -> bool {
        let condition_registry = *registries.get(&self.registry_condition).unwrap_or(&0);

        match self.condition {
            Condition::Below => condition_registry < self.value_condition,
            Condition::BelowOrEqual => condition_registry <= self.value_condition,
            Condition::Above => condition_registry > self.value_condition,
            Condition::AboveOrEqual => condition_registry >= self.value_condition,
            Condition::NotEqual => condition_registry != self.value_condition,
            Condition::Equal => condition_registry == self.value_condition,
        }
    }
}
