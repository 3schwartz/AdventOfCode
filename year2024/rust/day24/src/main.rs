use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;

    let z_values = solve(&input)?;
    println!("{:?}", z_values);
    let binary = z_values
        .values()
        .rev()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");

    let output = u64::from_str_radix(&binary, 2).unwrap();

    println!("Part 1: {}", output);

    Ok(())
}

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Ok};

#[derive(Clone, Copy)]
enum Operator {
    And,
    Xor,
    Or,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let operator = match s {
            "AND" => Operator::And,
            "XOR" => Operator::Xor,
            "OR" => Operator::Or,
            _ => return Err(anyhow!("{s} didn't match any operator.")),
        };
        Ok(operator)
    }
}

#[derive(Clone, Copy)]
struct Gate<'a> {
    first: &'a str,
    second: &'a str,
    output: &'a str,
    operator: Operator,
}

impl<'a> Gate<'a> {
    fn update(&self, values: &BTreeMap<&str, u32>) -> Option<(u32, &str)> {
        if values.contains_key(self.output) {
            return None;
        };
        if !values.contains_key(self.first) || !values.contains_key(self.second) {
            return None;
        };
        let first = values.get(self.first).unwrap();
        let second = values.get(self.second).unwrap();
        let output = match self.operator {
            Operator::And => first & second,
            Operator::Xor => first ^ second,
            Operator::Or => first | second,
        };
        Some((output, self.output))
    }
}

impl<'a> TryFrom<&'a str> for Gate<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(" -> ").collect();
        assert_eq!(parts.len(), 2);
        let input: Vec<&str> = parts[0].split_whitespace().collect();
        assert_eq!(input.len(), 3);
        let operator = Operator::from_str(input[1])?;
        Ok(Gate {
            first: input[0],
            second: input[2],
            output: parts[1],
            operator,
        })
    }
}

fn solve(input: &str) -> Result<BTreeMap<String, u32>> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);

    let mut values = BTreeMap::new();
    let mut updates = Vec::new();
    let mut z_set = HashSet::new();
    for line in parts[0].lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(parts.len(), 2);
        let value: u32 = parts[1].parse()?;
        values.insert(parts[0], value);
        updates.push(parts[0]);
        if parts[0].starts_with('z') {
            z_set.insert(parts[0]);
        }
    }

    let mut subscriptions: HashMap<&str, Vec<Gate>> = HashMap::new();
    for line in parts[1].lines() {
        let gate = Gate::try_from(line)?;
        subscriptions
            .entry(gate.first)
            .and_modify(|v| v.push(gate))
            .or_insert_with(|| vec![gate]);
        subscriptions
            .entry(gate.second)
            .and_modify(|v| v.push(gate))
            .or_insert_with(|| vec![gate]);
        if gate.second.starts_with('z') {
            z_set.insert(gate.second);
        }
        if gate.output.starts_with('z') {
            z_set.insert(gate.output);
        }
        if gate.first.starts_with('z') {
            z_set.insert(gate.first);
        }
    }
    let z_count = z_set.len();

    while let Some(next) = updates.pop() {
        if values.keys().filter(|k| k.starts_with('z')).count() == z_count {
            break;
        }
        if let Some(next_sub) = subscriptions.get(next) {
            for sub in next_sub {
                if let Some((value, output)) = sub.update(&values) {
                    values.insert(output, value);
                    updates.push(output);
                }
            }
        }
    }

    let mut z_values = BTreeMap::new();
    for z in values.iter().filter(|(k, _)| k.starts_with('z')) {
        z_values.insert(z.0.to_string(), *z.1);
    }
    Ok(z_values)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day24_test_data.txt")?;

        // Act
        let z_values = solve(&input)?;
        println!("{:?}", z_values);
        let binary = z_values
            .values()
            .rev()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("");

        let output = u32::from_str_radix(&binary, 2).unwrap();

        // Assert
        assert_eq!(output, 2024);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }
}
