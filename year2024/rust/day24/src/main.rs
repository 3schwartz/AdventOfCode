use anyhow::Result;
use anyhow::{anyhow, Ok};
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Write;
use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;

    let output = solve_1(&input)?;

    println!("Part 1: {}", output);

    make_graph(&input);

    let output = solve_2_v3(&input)?;

    println!("Part 2: {}", output);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

fn make_graph(input: &str) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);

    let mut file = File::create("output.txt").expect("Unable to create file");

    writeln!(file, "strict digraph {{").expect("Unable to write to file");
    for line in parts[1].lines() {
        if line.trim().is_empty() {
            continue; // Skip empty lines
        }
        let gate = Gate::try_from(line).unwrap();
        writeln!(
            file,
            "  {} -> {} [label={:?}]",
            gate.first, gate.output, gate.operator
        )
        .expect("Unable to write to file");
        writeln!(
            file,
            "  {} -> {} [label={:?}]",
            gate.second, gate.output, gate.operator
        )
        .expect("Unable to write to file");
    }
    writeln!(file, "}}").expect("Unable to write to file");
}

fn solve_1(input: &str) -> Result<u64> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let (values, updates) = make_values(parts[0])?;
    let subscriptions = make_subscriptions(parts[1])?;
    let z_count = start_with('z', &subscriptions);

    find_z(&subscriptions, &updates, &values, z_count)
}

fn solve_2_v3(input: &str) -> Result<String> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let (values, updates) = make_values(parts[0])?;
    let subscriptions = make_subscriptions(parts[1])?;
    let x = find_values_from_char('x', &values)?;
    let y = find_values_from_char('y', &values)?;

    let z_count = start_with('z', &subscriptions);
    let mut gates = make_gates_from_subscriptions(&subscriptions);

    let mut changes = vec![];
    let mut swaps: Vec<(String, String)> = vec![];

    let s = x + y;
    loop {
        let update_subscriptions = make_updated_subscriptions_from_swaps(&swaps, &subscriptions);
        let z = find_z(&update_subscriptions, &updates, &values, z_count)?;
        if z == s {
            break;
        }
        for i in 0..z_count {
            let actual = (z >> i) & 1;
            let expected = (s >> i) & 1;
            if actual != expected || changes.len() == 1 {
                let z_out = format!("z{i:02}");

                let gate = gates.get(z_out.as_str()).unwrap();
                if gate.operator == Operator::Xor {
                    if (gate.first.starts_with('x') || gate.first.starts_with('y'))
                        && (gate.second.starts_with('x') || gate.second.starts_with('y'))
                    {
                        continue;
                    }
                    let first_gate = gates.get(gate.first).unwrap();
                    let second_gate = gates.get(gate.second).unwrap();

                    if first_gate.operator == Operator::Xor
                        && (first_gate.first.starts_with('x') || first_gate.first.starts_with('y'))
                    {
                        if second_gate.operator != Operator::Or {
                            changes.push(gate.second.to_string());
                        } else {
                            let second_gate_first = gates.get(second_gate.first).unwrap();
                            if second_gate_first.operator != Operator::And {
                                changes.push(second_gate.first.to_string());
                            }
                            let second_gate_second = gates.get(second_gate.second).unwrap();
                            if second_gate_second.operator != Operator::And {
                                changes.push(second_gate.second.to_string());
                            }
                        }
                    } else {
                        if first_gate.operator != Operator::Or {
                            changes.push(gate.first.to_string());
                        } else {
                            let first_gate_first = gates.get(first_gate.first).unwrap();
                            if first_gate_first.operator != Operator::And {
                                changes.push(first_gate.first.to_string());
                            }
                            let first_gate_second = gates.get(first_gate.second).unwrap();
                            if first_gate_second.operator != Operator::And {
                                changes.push(first_gate.second.to_string());
                            }
                        }
                    }
                } else {
                    changes.push(z_out);
                }
            }
            if changes.len() == 2 {
                break;
            }
        }
        let g_1 = *gates.get(changes[0].as_str()).unwrap();
        let g_2 = *gates.get(changes[1].as_str()).unwrap();
        gates.insert(
            g_1.output,
            Gate {
                first: g_2.first,
                second: g_2.second,
                output: g_1.output,
                operator: g_2.operator,
            },
        );
        gates.insert(
            g_2.output,
            Gate {
                first: g_1.first,
                second: g_1.second,
                output: g_2.output,
                operator: g_1.operator,
            },
        );
        swaps.push((changes[0].clone(), changes[1].clone()));
        changes.clear();
    }
    let mut set = BTreeSet::new();
    for swap in swaps {
        set.insert(swap.0);
        set.insert(swap.1);
    }
    Ok(set
        .iter()
        .map(|n| n.as_str())
        .collect::<Vec<&str>>()
        .join(","))
}

fn make_updated_subscriptions_from_swaps<'a>(
    swaps: &'a Vec<(String, String)>,
    subscriptions: &'a BTreeMap<&str, Vec<Gate>>,
) -> BTreeMap<&'a str, Vec<Gate<'a>>> {
    let mut update_subscriptions = BTreeMap::new();
    for (k, v) in subscriptions {
        let mut k_sub = vec![];
        for g in v {
            let mut insert = true;
            for swap in swaps {
                if g.output == swap.0 {
                    k_sub.push(Gate {
                        first: g.first,
                        second: g.second,
                        operator: g.operator,
                        output: swap.1.as_str(),
                    });
                    insert = false;
                    break;
                }
                if g.output == swap.1 {
                    k_sub.push(Gate {
                        first: g.first,
                        second: g.second,
                        operator: g.operator,
                        output: swap.0.as_str(),
                    });
                    insert = false;
                    break;
                }
            }
            if insert {
                k_sub.push(*g);
            }
        }
        update_subscriptions.insert(*k, k_sub);
    }
    update_subscriptions
}

fn make_gates_from_subscriptions<'a>(
    subscriptions: &'a BTreeMap<&str, Vec<Gate>>,
) -> BTreeMap<&'a str, Gate<'a>> {
    let mut gates = BTreeMap::new();
    for o in subscriptions.values() {
        for g in o {
            gates.insert(g.output, *g);
        }
    }
    gates
}

fn start_with(start: char, subscriptions: &BTreeMap<&str, Vec<Gate>>) -> usize {
    let mut start_with = HashSet::new();
    for v in subscriptions.values() {
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
    updates: &[&str],
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
}
