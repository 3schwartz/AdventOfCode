use anyhow::{anyhow, Ok, Result};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    str::FromStr,
    sync::Arc,
};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    RwLock,
};

#[tokio::main]
async fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day18_data.txt")?;

    let instructions = Instruct::make_instructions(&input)?;
    Instruct::run(instructions);

    let instructions = Instruct::make_instructions(&input)?;
    let s = Instruct::start_async(instructions).await?;

    println!("Part 2: {s}");

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

enum Instruct {
    Two(String, String, InstructionTwo),
    Single(String, InstructionSingle),
}

struct StateAsync {
    sender: UnboundedSender<i64>,
    receiver: UnboundedReceiver<i64>,
    queue_size: Arc<RwLock<i64>>,
    receiver_awaiting: Arc<RwLock<i16>>,
}

impl Instruct {
    fn make_instructions(input: &str) -> Result<BTreeMap<i64, Instruct>> {
        let mut instructions = BTreeMap::new();
        for (i, line) in input.lines().enumerate() {
            instructions.insert(i as i64, Instruct::try_from(line)?);
        }
        Ok(instructions)
    }

    async fn start_async(instructions: BTreeMap<i64, Instruct>) -> Result<u128> {
        let (first_snd, first_rcv) = mpsc::unbounded_channel::<i64>();
        let (second_snd, second_rcv) = mpsc::unbounded_channel::<i64>();

        let queue_size = Arc::new(RwLock::new(0));
        let rcv_awaits = Arc::new(RwLock::new(0));

        let first_state = StateAsync {
            sender: second_snd,
            receiver: first_rcv,
            queue_size: queue_size.clone(),
            receiver_awaiting: rcv_awaits.clone(),
        };
        let second_state = StateAsync {
            sender: first_snd,
            receiver: second_rcv,
            queue_size,
            receiver_awaiting: rcv_awaits,
        };
        let registry_first = HashMap::from([("p", 0)]);
        let registry_second = HashMap::from([("p", 1)]);

        let instrs = Arc::new(instructions);
        let instrs_clone = instrs.clone();

        let first_result = tokio::spawn(async move {
            Self::run_async(0, registry_first, first_state, instrs_clone).await
        });
        let second_result =
            tokio::spawn(
                async move { Self::run_async(1, registry_second, second_state, instrs).await },
            );

        let _ = first_result.await??;
        let s = second_result.await??;
        Ok(s)
    }

    async fn run_async(
        id: u8,
        mut registry: HashMap<&str, i64>,
        mut state: StateAsync,
        instructions: Arc<BTreeMap<i64, Instruct>>,
    ) -> Result<u128> {
        let mut send_count = 0;
        let mut cursor = 0;
        while let Some(instruction) = instructions.get(&cursor) {
            let j = instruction
                .react_async(id, &mut registry, &mut state, &mut send_count)
                .await?;
            if let Some(n) = j {
                cursor += n;
            } else {
                break;
            }
        }
        Ok(send_count)
    }

    async fn react_async<'a>(
        &'a self,
        id: u8,
        registry: &mut HashMap<&'a str, i64>,
        state: &mut StateAsync,
        send_count: &mut u128,
    ) -> Result<Option<i64>> {
        match self {
            Instruct::Two(r, s, instruction_two) => {
                let v = Self::value_of(s, registry);
                match instruction_two {
                    InstructionTwo::Set => {
                        registry.insert(r, v);
                    }
                    InstructionTwo::Add => {
                        let entry = registry.entry(r).or_default();
                        *entry += v;
                    }
                    InstructionTwo::Mul => {
                        let x = registry.entry(r).or_default();
                        *x *= v;
                    }
                    InstructionTwo::Mod => {
                        let x = registry.entry(r).or_default();
                        *x %= v;
                    }
                    InstructionTwo::Jgz => {
                        let sound = Self::value_of(r, registry);
                        if sound != 0 {
                            return Ok(Some(v));
                        }
                    }
                }
            }
            Instruct::Single(r, instruction_single) => match instruction_single {
                InstructionSingle::Snd => {
                    let next = Self::value_of(r, registry);
                    state.sender.send(next)?;
                    let mut queue_size = state.queue_size.write().await;
                    *queue_size += 1;
                    *send_count += 1;
                    if *send_count % 1_000_000 == 0 {
                        println!("{id}: {send_count}");
                    }
                }
                InstructionSingle::Rcv => {
                    let queue_size = { *state.queue_size.read().await };
                    let rcv_awaits = {
                        let mut rcv_awaits = state.receiver_awaiting.write().await;
                        *rcv_awaits += 1;
                        *rcv_awaits
                    };
                    if queue_size == 0 && rcv_awaits == 2 {
                        return Ok(None);
                    }
                    if let Some(next) = state.receiver.recv().await {
                        registry.insert(r, next);
                        let mut rcv_awaits = state.receiver_awaiting.write().await;
                        *rcv_awaits -= 1;
                        let mut queue_size = state.queue_size.write().await;
                        *queue_size -= 1;
                    } else {
                        return Ok(None);
                    }
                }
            },
        }
        Ok(Some(1))
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

    fn react<'a>(&'a self, registry: &mut HashMap<&'a str, i64>, state: &mut State) -> i64 {
        match self {
            Instruct::Two(r, s, instruction_two) => {
                let v = Self::value_of(s, registry);
                match instruction_two {
                    InstructionTwo::Set => {
                        registry.insert(r, v);
                    }
                    InstructionTwo::Add => {
                        let entry = registry.entry(r).or_default();
                        *entry += v;
                    }
                    InstructionTwo::Mul => {
                        let x = registry.entry(r).or_default();
                        *x *= v;
                    }
                    InstructionTwo::Mod => {
                        let x = registry.entry(r).or_default();
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

impl TryFrom<&str> for Instruct {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let parts = value.split(' ').collect::<Vec<&str>>();
        if parts.len() != 2 && parts.len() != 3 {
            return Err(anyhow!("wrong length: {:?}", parts));
        }
        let i = if parts.len() == 3 {
            let instruct = InstructionTwo::from_str(parts[0])?;
            Instruct::Two(parts[1].to_string(), parts[2].to_string(), instruct)
        } else {
            let instruct = InstructionSingle::from_str(parts[0])?;
            Instruct::Single(parts[1].to_string(), instruct)
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
    use super::*;

    fn some_reference(i: &mut i64) {
        *i += 1;
    }

    #[test]
    fn test_reference() {
        let mut i = 1;
        some_reference(&mut i);
        assert_eq!(2, i);
    }

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

    #[tokio::test]
    async fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day18_test2_data.txt")?;

        // Act
        let instructions = Instruct::make_instructions(&input)?;
        let last = Instruct::start_async(instructions).await?;

        // Assert
        assert!(matches!(last, 3));
        Ok(())
    }
}
