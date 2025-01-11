use anyhow::{anyhow, Ok, Result};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    str::FromStr,
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[tokio::main]
async fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day18_data.txt")?;

    let instructions = Instruct::make_instructions(&input)?;
    Instruct::run(instructions);

    Ok(())
}

enum InstructionTwo {
    Set,
    Add,
    Mul,
    Mod,
    Jgz,
}

impl FromStr for InstructionTwo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let t = match s {
            "set" => InstructionTwo::Set,
            "add" => InstructionTwo::Add,
            "mul" => InstructionTwo::Mul,
            "mod" => InstructionTwo::Mod,
            "jgz" => InstructionTwo::Jgz,
            _ => return Err(anyhow!("not able to match: {:?}", s)),
        };
        Ok(t)
    }
}

enum InstructionSingle {
    Snd,
    Rcv,
}

impl FromStr for InstructionSingle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let t = match s {
            "snd" => InstructionSingle::Snd,
            "rcv" => InstructionSingle::Rcv,
            _ => return Err(anyhow!("not able to match: {:?}", s)),
        };
        Ok(t)
    }
}

enum Instruct<'a> {
    Two(&'a str, &'a str, InstructionTwo),
    Single(&'a str, InstructionSingle),
}

struct StateAsync {
    last_sound: i64,
    done: bool,
    sender: UnboundedSender<i32>,
    receiver: UnboundedReceiver<i32>,
}

impl<'a> Instruct<'a> {
    fn make_instructions(input: &str) -> Result<BTreeMap<i64, Instruct>> {
        let mut instructions = BTreeMap::new();
        for (i, line) in input.lines().enumerate() {
            instructions.insert(i as i64, Instruct::try_from(line)?);
        }
        Ok(instructions)
    }

    fn run_async(instruction: BTreeMap<i64, Instruct>) {
        let (first_tx, mut first_rx) = mpsc::unbounded_channel::<i32>();
    }

    fn run(instructions: BTreeMap<i64, Instruct>) -> Option<i64> {
        let mut cursor = 0;
        let mut state = State::new();
        let mut registry = HashMap::new();
        while let Some(instruction) = instructions.get(&cursor) {
            let j = instruction.react(&mut registry, &mut state);
            if state.done {
                return Some(state.last_sound);
            }
            cursor += j;
        }
        None
    }
    async fn react_async<'b>(
        &self,
        registry: &'b mut HashMap<&'a str, i64>,
        state: &mut State,
    ) -> i64 {
        match self {
            Instruct::Two(r, s, instruction_two) => {
                let v = Self::value_of(s, registry);
                match instruction_two {
                    InstructionTwo::Set => {
                        registry.insert(*r, v);
                    }
                    InstructionTwo::Add => {
                        let entry = registry.entry(*r).or_default();
                        *entry += v;
                    }
                    InstructionTwo::Mul => {
                        let x = registry.entry(*r).or_default();
                        *x *= v;
                    }
                    InstructionTwo::Mod => {
                        let x = registry.entry(*r).or_default();
                        *x %= v;
                    }
                    InstructionTwo::Jgz => {
                        let sound = Self::value_of(r, registry);
                        if sound != 0 {
                            return v;
                        }
                    }
                }
            }
            Instruct::Single(r, instruction_single) => {
                let next = Self::value_of(r, registry);
                match instruction_single {
                    InstructionSingle::Snd => state.last_sound = next,
                    InstructionSingle::Rcv => {
                        if next != 0 {
                            println!("{}", state.last_sound);
                            state.done = true;
                        }
                    }
                }
            }
        }
        1
    }

    fn react<'b>(&self, registry: &'b mut HashMap<&'a str, i64>, state: &mut State) -> i64 {
        match self {
            Instruct::Two(r, s, instruction_two) => {
                let v = Self::value_of(s, registry);
                match instruction_two {
                    InstructionTwo::Set => {
                        registry.insert(*r, v);
                    }
                    InstructionTwo::Add => {
                        let entry = registry.entry(*r).or_default();
                        *entry += v;
                    }
                    InstructionTwo::Mul => {
                        let x = registry.entry(*r).or_default();
                        *x *= v;
                    }
                    InstructionTwo::Mod => {
                        let x = registry.entry(*r).or_default();
                        *x %= v;
                    }
                    InstructionTwo::Jgz => {
                        let sound = Self::value_of(r, registry);
                        if sound != 0 {
                            return v;
                        }
                    }
                }
            }
            Instruct::Single(r, instruction_single) => {
                let next = Self::value_of(r, registry);
                match instruction_single {
                    InstructionSingle::Snd => state.last_sound = next,
                    InstructionSingle::Rcv => {
                        if next != 0 {
                            println!("{}", state.last_sound);
                            state.done = true;
                        }
                    }
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

impl<'a> TryFrom<&'a str> for Instruct<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        let parts = value.split(' ').collect::<Vec<&str>>();
        if parts.len() != 2 && parts.len() != 3 {
            return Err(anyhow!("wrong length: {:?}", parts));
        }
        let i = if parts.len() == 3 {
            let instruct = InstructionTwo::from_str(parts[0])?;
            Instruct::Two(parts[1], parts[2], instruct)
        } else {
            let instruct = InstructionSingle::from_str(parts[0])?;
            Instruct::Single(parts[1], instruct)
        };
        Ok(i)
    }
}

struct State {
    last_sound: i64,
    done: bool,
}

impl State {
    fn new() -> Self {
        Self {
            last_sound: 0,
            done: false,
        }
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::RwLock;

    use super::*;

    #[tokio::test]
    async fn test_mutext() {
        let mutext = RwLock::new(5);
        {
            let mut i = mutext.write().await;
            *i += 1;
        }

        let i_ = mutext.read().await;
        assert_eq!(*i_, 6);
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day18_test_data.txt")?;

        // Act
        let instructions = Instruct::make_instructions(&input)?;
        let last = Instruct::run(instructions);

        // Assert
        assert!(matches!(last, Some(4)));
        Ok(())
    }
}
