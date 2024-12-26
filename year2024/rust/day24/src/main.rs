use anyhow::Result;
use std::collections::BTreeSet;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;

    let output = solve_1(&input)?;

    println!("Part 1: {}", output);

    let output = solve_2(&input)?;

    println!("Part 2: {}", output);

    Ok(())
}

use std::{
    collections::{BTreeMap, HashSet},
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

    fn start_with(&self, start: char, set: &mut HashSet<&'a str>) {
        if self.second.starts_with(start) {
            set.insert(self.second);
        }
        if self.output.starts_with(start) {
            set.insert(self.output);
        }
        if self.first.starts_with(start) {
            set.insert(self.first);
        }
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

fn solve_1(input: &str) -> Result<u64> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let (values, updates) = make_values(parts[0])?;
    let subscriptions = make_subscriptions(parts[1])?;
    let z_count = start_with('z', &subscriptions);

    find_z(&subscriptions, &updates, &values, z_count)
}

fn solve_2(input: &str) -> Result<String> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let (values, updates) = make_values(parts[0])?;
    let subscriptions = make_subscriptions(parts[1])?;
    let x = find_values_from_char('x', &values)?;
    let y = find_values_from_char('x', &values)?;

    let mut seen = BTreeSet::new();
    let z_count = start_with('z', &subscriptions);
    for i_1 in subscriptions.keys() {
        for i_2 in subscriptions.keys() {
            if i_1 == i_2 {
                continue;
            }
            for i_3 in subscriptions.keys() {
                if i_1 == i_3 || i_2 == i_3 {
                    continue;
                }
                for i_4 in subscriptions.keys() {
                    if i_1 == i_4 || i_2 == i_4 || i_3 == i_4 {
                        continue;
                    }
                    let mut to_swap = vec![i_1, i_2, i_3, i_4];
                    let mut single_swaps = HashSet::from([i_1, i_2, i_3, i_4]);
                    let mut swaps = BTreeSet::new();

                    while let Some(next) = to_swap.pop() {
                        for s in subscriptions.keys() {
                            if i_1 == s || i_2 == s || i_3 == s || i_4 == s {
                                continue;
                            }
                            if !single_swaps.insert(s) {
                                continue;
                            }
                            swaps.insert((next, s));
                        }
                    }
                    if !seen.insert(swaps.clone()) {
                        continue;
                    }

                    let mut update_subscriptions = BTreeMap::new();
                    for (k, v) in &subscriptions {
                        let mut insert = true;
                        for swap in &swaps {
                            if k == swap.0 {
                                update_subscriptions.insert(*swap.1, v.clone());
                                insert = false;
                                break;
                            }
                            if k == swap.1 {
                                update_subscriptions.insert(*swap.0, v.clone());
                                insert = false;
                                break;
                            }
                        }
                        if insert {
                            update_subscriptions.insert(k, v.clone());
                        }
                    }
                    let z = find_z(&subscriptions, &updates, &values, z_count)?;
                    if z == x + y {
                        let mut set = BTreeSet::new();
                        for swap in swaps {
                            set.insert(*swap.0);
                            set.insert(*swap.1);
                        }
                        return Ok(set.iter().map(|n| *n).collect::<Vec<&str>>().join(","));
                    }
                }
            }
        }
    }
    Err(anyhow!("no result found"))
}

fn start_with(start: char, subscriptions: &BTreeMap<&str, Vec<Gate>>) -> usize {
    let mut start_with = HashSet::new();
    for (_, v) in subscriptions {
        for g in v {
            g.start_with(start, &mut start_with);
        }
    }
    start_with.len()
}

fn make_values(input: &str) -> Result<(BTreeMap<&str, u32>, Vec<&str>)> {
    let mut values = BTreeMap::new();
    let mut updates = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(parts.len(), 2);
        let value: u32 = parts[1].parse()?;
        values.insert(parts[0], value);
        updates.push(parts[0]);
    }
    Ok((values, updates))
}

fn make_subscriptions(input: &str) -> Result<BTreeMap<&str, Vec<Gate>>> {
    let mut subscriptions: BTreeMap<&str, Vec<Gate>> = BTreeMap::new();
    for line in input.lines() {
        let gate = Gate::try_from(line)?;
        subscriptions
            .entry(gate.first)
            .and_modify(|v| v.push(gate))
            .or_insert_with(|| vec![gate]);
        subscriptions
            .entry(gate.second)
            .and_modify(|v| v.push(gate))
            .or_insert_with(|| vec![gate]);
    }
    Ok(subscriptions)
}

fn find_z(
    subscriptions: &BTreeMap<&str, Vec<Gate>>,
    updates: &Vec<&str>,
    values: &BTreeMap<&str, u32>,
    z_count: usize,
) -> Result<u64> {
    let mut updates = updates.to_vec();
    let mut values = values.clone();
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

    find_values_from_char('z', &values)
}

fn find_values_from_char(prefix: char, values: &BTreeMap<&str, u32>) -> Result<u64> {
    let mut z_values = BTreeMap::new();
    for z in values.iter().filter(|(k, _)| k.starts_with(prefix)) {
        z_values.insert(z.0.to_string(), *z.1);
    }

    let binary = z_values
        .values()
        .rev()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");

    Ok(u64::from_str_radix(&binary, 2)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day24_test_data.txt")?;

        // Act
        let output = solve_1(&input)?;

        // Assert
        assert_eq!(output, 2024);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day24_test2_data.txt")?;

        // Act
        let output = solve_2(&input)?;

        // Assert
        assert_eq!(output, "aaa,aoc,bbb,ccc,eee,ooo,z24,z99");
        Ok(())
    }
}
