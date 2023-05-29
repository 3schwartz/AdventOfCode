use std::{collections::HashMap, fs};

use anyhow::{anyhow, Result};

struct Program<'a> {
    registries: HashMap<&'a str, usize>,
}

impl<'a> Program<'a> {
    fn from(registries: Vec<(&'a str, usize)>) -> Self {
        Self {
            registries: registries.into_iter().collect(),
        }
    }

    fn create_instructions(&self, input: &'a str) -> Vec<Vec<&'a str>> {
        let lines: Vec<Vec<&str>> = input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|s| s.trim_end_matches(','))
                    .collect()
            })
            .collect();

        return lines;
    }

    fn run(&mut self, input: String, out: &str) -> Result<usize> {
        let mut idx: usize = 0;
        let lines = self.create_instructions(&input);

        while let Some(line) = lines.get(idx) {
            if line.len() < 2 {
                return Err(anyhow!("invalid line: {:?}", line));
            }
            match line[0] {
                "hlf" => {
                    let registry = self
                        .registries
                        .get_mut(line[1])
                        .ok_or_else(|| anyhow!("unknown registry from line: {:?}", line))?;
                    *registry = *registry / 2;
                    idx += 1;
                }
                "tpl" => {
                    let registry = self
                        .registries
                        .get_mut(line[1])
                        .ok_or_else(|| anyhow!("unknown registry from line: {:?}", line))?;
                    *registry = *registry * 3;
                    idx += 1;
                }
                "inc" => {
                    let registry = self
                        .registries
                        .get_mut(line[1])
                        .ok_or_else(|| anyhow!("unknown registry from line: {:?}", line))?;
                    *registry += 1;
                    idx += 1;
                }
                "jmp" => {
                    idx = (idx as i32 + line[1].parse::<i32>()?) as usize;
                }
                "jie" => {
                    if line.len() != 3 {
                        return Err(anyhow!("not correct len: {:?}", line));
                    }
                    let registry = self
                        .registries
                        .get_mut(line[1])
                        .ok_or_else(|| anyhow!("unknown registry from line: {:?}", line))?;
                    if *registry % 2 != 0 {
                        idx += 1;
                        continue;
                    }
                    idx = (idx as i32 + line[2].parse::<i32>()?) as usize;
                }
                "jio" => {
                    if line.len() != 3 {
                        return Err(anyhow!("not correct len: {:?}", line));
                    }
                    let registry = self
                        .registries
                        .get_mut(line[1])
                        .ok_or_else(|| anyhow!("unknown registry from line: {:?}", line))?;
                    if *registry != 1 {
                        idx += 1;
                        continue;
                    }
                    idx = (idx as i32 + line[2].parse::<i32>()?) as usize;
                }
                _ => {
                    return Err(anyhow!(
                        "not able to read instruction from line: {:?}",
                        line
                    ))
                }
            };
        }
        let out = self
            .registries
            .get(out)
            .ok_or_else(|| anyhow!("missing b registry: {:?}", self.registries))?;

        Ok(*out)
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day23_data.txt")?;
    let mut computer = Program::from(vec![("a", 0), ("b", 0)]);
    let part_1 = computer.run(input.clone(), "b")?;
    println!("Part 1: {}", part_1);

    let mut computer = Program::from(vec![("a", 1), ("b", 0)]);

    let part_2 = computer.run(input, "b")?;
    println!("Part 2: {}", part_2);

    Ok(())
}
