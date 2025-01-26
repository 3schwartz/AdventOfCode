use anyhow::{anyhow, Result};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    str::FromStr,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day23_data.txt")?;

    let instructions = Instruction::make_instructions(&input)?;
    let mul_count = Instruction::run(instructions);

    println!("Part 1: {mul_count}");
    Ok(())
}

enum Action {
    Set,
    Sub,
    Mul,
    Jgz,
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let t = match s {
            "set" => Action::Set,
            "sub" => Action::Sub,
            "mul" => Action::Mul,
            "jnz" => Action::Jgz,
            _ => return Err(anyhow!("not able to match: {:?}", s)),
        };
        Ok(t)
    }
}

struct Instruction {
    x: String,
    y: String,
    action: Action,
}

impl Instruction {
    fn new(x: String, y: String, action: Action) -> Self {
        Self { x, y, action }
    }
    fn make_instructions(input: &str) -> Result<BTreeMap<i64, Instruction>> {
        let mut instructions = BTreeMap::new();
        for (i, line) in input.lines().enumerate() {
            instructions.insert(i as i64, Instruction::try_from(line)?);
        }
        Ok(instructions)
    }
    fn run(instructions: BTreeMap<i64, Instruction>) -> u32 {
        let mut cursor = 0;
        let mut mul_invoked = 0;
        let mut registry = HashMap::new();
        while let Some(instruction) = instructions.get(&cursor) {
            let j = instruction.react(&mut registry, &mut mul_invoked);
            cursor += j;
        }
        mul_invoked
    }

    fn react<'a>(&'a self, registry: &mut HashMap<&'a str, i64>, mul_invoked: &mut u32) -> i64 {
        let y = Self::value_of(&self.y, registry);
        let x = &self.x;
        match self.action {
            Action::Set => {
                registry.insert(x, y);
            }
            Action::Sub => {
                let x_entry = registry.entry(x).or_default();
                *x_entry -= y;
            }
            Action::Mul => {
                let x_entry = registry.entry(x).or_default();
                *x_entry *= y;
                *mul_invoked += 1;
            }
            Action::Jgz => {
                let x_entry = Self::value_of(x, registry);
                if x_entry != 0 {
                    return y;
                }
            }
        }
        1
    }

    fn value_of(c: &str, registry: &HashMap<&str, i64>) -> i64 {
        match c.parse::<i64>() {
            std::result::Result::Ok(i) => i,
            Err(_) => *registry.get(c).unwrap_or(&0),
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let parts = value.split(' ').collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(anyhow!("wrong length: {:?}", parts));
        }
        let action = Action::from_str(parts[0])?;
        Ok(Instruction::new(
            parts[1].to_string(),
            parts[2].to_string(),
            action,
        ))
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_part_1() {
        // Arrange
        // Act
        // Assert
        assert!(true)
    }
}
