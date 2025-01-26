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

    let part_2 = part_2();

    println!("Part 2: {part_2}");

    Ok(())
}

/// set b 81
/// #set c b
/// jnz a 2
/// #jnz 1 5
/// mul b 100           / b = 8_100
/// sub b -100000       / b = 108_100
/// set c b             / c = 108_100
/// sub c -17000        / c = 125_100
/// set f 1             / f = 1 (reset from last step)
/// set d 2             / d = 2
/// set e 2             / e = 2 (back from -13 - here d increased)
/// set g d             / g = 2                      / g = 2 (back grom jnz -8 - here e increased)
/// mul g e             / g = 4                      / g = 6                        / g = 8
/// sub g b             / g = 4 - 108_100 = -108_096 / g = 6 - 108_100 = -108_094   / g = -108_092
/// jnz g 2             
/// set f 0             / only when d % b == 0 since then 'e' exists such that e * d = b
/// sub e -1            / e = 2                       / e = 3
/// set g e             / g = 2                       / g = 3
/// sub g b             / g = 2 - 108_100 = 108_098   / g = 0 (when e = 108_100 (e = b))
/// jnz g -8            (when e = 108_100 = b continue. Hence compare up to <b)
/// sub d -1
/// set g d
/// sub g b             
/// jnz g -13           (when d = 108_100 = b continue. Hence compare up to <b)
/// jnz f 2
/// sub h -1
/// set g b             / g = 108_100
/// sub g c             / c = -108900 => 108_100 - 125_100 => -17_000
/// jnz g 2             / when b = c
/// jnz 1 3
/// sub b -17           / Done 1_000 + 1 iterations
/// jnz 1 -23
fn part_2() -> i32 {
    let b_start = 108_100;

    let mut h = 0;
    for i in 0..1_001 {
        let b = b_start + i * 17;
        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
    }
    h
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
