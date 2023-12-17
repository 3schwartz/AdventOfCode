use std::fs;

use anyhow::{anyhow, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day8_data.txt")?;

    let network = Network::new(&input);
    let steps = network.get_steps("AAA", "ZZZ")?;

    println!("Part 1: {}", steps);
    Ok(())
}

struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(input: &str) -> Self {
        let x: &[_] = &['(', ')'];
        let parts: Vec<&str> = input.trim_matches(x).split(", ").collect();
        Self {
            left: parts[0].to_string(),
            right: parts[1].to_string(),
        }
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
            let parts: Vec<&str> = line.split(" = ").collect();
            let name = parts[0].to_string();
            let node = Node::new(parts[1]);
            nodes.insert(name, node);
        }
        nodes
    }

    fn get_steps(&self, from: &str, to: &str) -> Result<u64> {
        let mut f = from;
        let mut steps = 0;
        loop {
            for c in self.actions.chars() {
                steps += 1;
                let s = self
                    .nodes
                    .get(f)
                    .ok_or_else(|| anyhow!("{} should be present", f))?;
                let next = match c {
                    'R' => &s.right,
                    'L' => &s.left,
                    _ => return Err(anyhow!("{} direction not handled", c)),
                };
                if next == to {
                    return Ok(steps);
                }
                f = next.as_str();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_1() -> Result<()> {
        // Arrange
        for (input, actual) in [("day8_data_test1", 2), ("day8_data_test2", 6)] {
            let input = fs::read_to_string(format!("../../data/{}.txt", input))?;

            // Act
            let network = Network::new(&input);
            let steps = network.get_steps("AAA", "ZZZ")?;

            // Assert
            assert_eq!(actual, steps);
        }
        Ok(())
    }
}
