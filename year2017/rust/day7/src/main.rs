use std::{fs, collections::{HashMap, HashSet}};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day7_data.txt")?;
    let mut programs: HashMap<&str, Program> = HashMap::new();
    let mut leafs_seen: HashSet<&str> = HashSet::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let first_parts: Vec<&str> = parts[0].split_whitespace().collect();
        let weight: u32 = first_parts[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .parse()?;

        // let program = programs.entry(first_parts[0])
        //     .or_insert_with(|| Program { weight: 0, leafs: vec![] });
        // program.weight = weight;

        let mut leafs = vec![];
        if parts.len() > 1 {
            for leaf in parts[1].split(", ") {
                leafs.push(leaf);
                leafs_seen.insert(leaf);
            }
        }
        programs.insert(first_parts[0], Program { weight, leafs });
    }
    let unique: HashSet<_> = programs.keys().filter(|&key| !leafs_seen.contains(key))
        .collect();

    println!("Part 1: {:#?}", unique);

    Ok(())
}

struct Program<'a> {
    weight: u32,
    leafs: Vec<&'a str>
}

impl<'a> Program<'a> {
    fn new(weight: u32) -> Self {
        Self {weight, leafs: vec![]}
    }
}
