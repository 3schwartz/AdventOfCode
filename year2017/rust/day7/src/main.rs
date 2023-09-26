use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day7_data.txt")?;
    let (programs, leafs_seen) = create_programs(&input)?;

    let unique = get_unique(&programs, leafs_seen);
    println!("Part 1: {:#?}", unique);

    let mut node = Node::from(unique, &programs)?;
    node.is_balanced();

    Ok(())
}

fn get_unique<'a>(programs: &'a HashMap<&str, Program>, leafs_seen: HashSet<&str>) -> &'a str {
    let unique: Vec<_> = programs
        .keys()
        .filter(|&key| !leafs_seen.contains(key))
        .collect();
    unique[0]
}

fn create_programs<'a>(
    input: &'a str,
) -> Result<(HashMap<&'a str, Program<'a>>, HashSet<&'a str>)> {
    let mut programs: HashMap<&str, Program> = HashMap::new();
    let mut leafs_seen: HashSet<&str> = HashSet::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let first_parts: Vec<&str> = parts[0].split_whitespace().collect();
        let weight: u32 = first_parts[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .parse()?;

        let mut leafs = vec![];
        if parts.len() > 1 {
            for leaf in parts[1].split(", ") {
                leafs.push(leaf);
                leafs_seen.insert(leaf);
            }
        }
        programs.insert(first_parts[0], Program { weight, leafs });
    }
    Ok((programs, leafs_seen))
}

struct Node {
    _name: String,
    weight: u32,
    leafs: Vec<Node>,
    total_weight: Option<u32>,
}

impl Node {
    fn from(node: &str, programs: &HashMap<&str, Program>) -> Result<Self> {
        let program = programs.get(node).unwrap();
        let leafs: Result<Vec<Node>> = program
            .leafs
            .iter()
            .map(|leaf| Node::from(&leaf, programs))
            .into_iter()
            .collect();
        Ok(Self {
            _name: node.to_string(),
            weight: program.weight,
            leafs: leafs?,
            total_weight: None,
        })
    }

    fn is_balanced(&mut self) {
        if self.leafs.len() < 2 {
            return;
        }
        for leaf in &mut self.leafs {
            leaf.is_balanced();
        }
        let mut weights: HashMap<u32, Vec<u32>> = HashMap::new();
        for leaf in &mut self.leafs {
            let leaf_weight = leaf.get_total_weight();
            let weights = weights.entry(leaf_weight).or_insert(vec![]);
            weights.push(leaf.weight);
        }
        if weights.len() == 1 {
            return;
        }
        let should_be = weights
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(k, _)| *k)
            .next()
            .unwrap();
        let (is_total_weight, node_weight): (u32, u32) = weights
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .map(|(k, v)| (*k, v[0]))
            .next()
            .unwrap();
        let node_weight_should_be =
            node_weight as i32 + (should_be as i32 - is_total_weight as i32);
        println!("Part 2: {} (only first output)", node_weight_should_be)
    }

    fn get_total_weight(&mut self) -> u32 {
        let Some(total_weight) = self.total_weight else {
            let mut total = self.weight;
            for leaf in &mut self.leafs {
                total += leaf.get_total_weight();
            }
            self.total_weight = Some(total);
            return total;
        };
        total_weight
    }
}

struct Program<'a> {
    weight: u32,
    leafs: Vec<&'a str>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day7_test_data.txt")?;
        let (programs, leafs_seen) = create_programs(&input)?;

        // Act
        let unique = get_unique(&programs, leafs_seen);

        // Assert
        assert_eq!(unique, "tknk");
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day7_test_data.txt")?;
        let (programs, leafs_seen) = create_programs(&input)?;
        let unique = get_unique(&programs, leafs_seen);

        // Act
        let mut node = Node::from(unique, &programs)?;
        node.is_balanced();
        Ok(())
    }
}
