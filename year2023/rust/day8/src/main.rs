use std::fs;

use anyhow::{anyhow, Ok, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day8_data.txt")?;

    let network = Network::new(&input);
    let steps = network.get_steps("AAA", "ZZZ")?;

    println!("Part 1: {}", steps);

    let starts: Vec<Vec<u64>> = network
        .nodes
        .values()
        .filter(|n| n.is_start)
        .map(|v| network.get_ghost_paths(v, 5))
        .collect::<Result<Vec<Vec<u64>>, _>>()?
        .iter()
        .map(get_diff)
        .collect();

    println!("{:?}", starts);

    let diffs = network.get_cycles_in_ghost_paths()?;
    let ghost_steps = find_common_lcm(diffs);

    println!("Part 2: {}", ghost_steps);
    Ok(())
}

fn find_common_lcm(v: Vec<u64>) -> u64 {
    let mut out = v[0];
    for i in v.iter().skip(1) {
        out = lcm(out, *i);
    }
    out
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn get_diff(v: &Vec<u64>) -> Vec<u64> {
    let mut difference = Vec::new();
    for i in 0..v.len() - 1 {
        let diff = v[i + 1] - v[i];
        difference.push(diff)
    }
    difference
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
    is_start: bool,
    is_end: bool,
}

impl Node {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" = ").collect();
        let name = parts[0].to_string();
        let x: &[_] = &['(', ')'];
        let parts: Vec<&str> = parts[1].trim_matches(x).split(", ").collect();
        let is_start = name.ends_with('A');
        let is_end = name.ends_with('Z');
        Self {
            name,
            left: parts[0].to_string(),
            right: parts[1].to_string(),
            is_start,
            is_end,
        }
    }

    fn next(&self, c: char) -> Result<&String> {
        let next = match c {
            'R' => &self.right,
            'L' => &self.left,
            _ => return Err(anyhow!("{} direction not handled", c)),
        };
        Ok(next)
    }
}

struct Network {
    actions: String,
    nodes: HashMap<String, Node>,
}

impl Network {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let actions = parts[0].to_string();
        let nodes = Network::create_nodes(parts[1]);
        Self { actions, nodes }
    }

    fn create_nodes(input: &str) -> HashMap<String, Node> {
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let node = Node::new(line);
            nodes.insert(node.name.to_string(), node);
        }
        nodes
    }

    fn get_cycles_in_ghost_paths(&self) -> Result<Vec<u64>> {
        self.nodes
            .values()
            .filter(|n| n.is_start)
            .map(|v| self.get_ghost_path(v))
            .collect::<Result<Vec<u64>>>()
    }

    fn get_ghost_path(&self, start: &Node) -> Result<u64> {
        Ok(self.get_ghost_paths(start, 1)?[0])
    }

    fn get_ghost_paths(&self, start: &Node, count: usize) -> Result<Vec<u64>> {
        let mut f = start.name.as_str();
        let mut steps = 0;
        let mut ends = vec![];
        loop {
            for c in self.actions.chars() {
                steps += 1;
                let s = self.get_node(f)?;
                let n = s.next(c)?;
                let next = self.get_node(n)?;
                if next.is_end {
                    ends.push(steps);
                }
                if ends.len() == count {
                    return Ok(ends);
                }
                f = next.name.as_str();
            }
        }
    }

    fn get_steps(&self, from: &str, to: &str) -> Result<u64> {
        let mut f = from;
        let mut steps = 0;
        loop {
            for c in self.actions.chars() {
                steps += 1;
                let s = self.get_node(f)?;
                let next = s.next(c)?;
                if next == to {
                    return Ok(steps);
                }
                f = next.as_str();
            }
        }
    }

    fn get_node(&self, key: &str) -> Result<&Node> {
        self.nodes
            .get(key)
            .ok_or_else(|| anyhow!("{} should be present", key))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        for (input, actual) in [("day8_data_test1", 2), ("day8_data_test2", 6)] {
            // Arrange
            let input = fs::read_to_string(format!("../../data/{}.txt", input))?;

            // Act
            let network = Network::new(&input);
            let steps = network.get_steps("AAA", "ZZZ")?;

            // Assert
            assert_eq!(actual, steps);
        }
        Ok(())
    }

    #[test]
    fn test_part_2_using_lcm() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day8_data_test3.txt")?;

        // Act
        let network = Network::new(&input);
        let diffs = network.get_cycles_in_ghost_paths()?;
        let ghost_steps = find_common_lcm(diffs);

        // Assert
        assert_eq!(6, ghost_steps);
        Ok(())
    }
}
