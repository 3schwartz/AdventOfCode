use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day16_data.txt")?;
    let parts: Vec<&str> = input.split("\n\n\n\n").collect();
    let instructions = Instruction::from_several(parts[0])?;
    let runner = OpcodeRunner::new();

    let part_1 = runner.find_behave_above_threshold(&instructions, 2)?;

    println!("Part 1: {}", part_1);

    let lookup = runner.find_opcode_possibilities(&instructions)?;
    let part_2 = OpcodeRunner::run_program(parts[1], lookup)?;

    println!("Part 2: {}", part_2);

    Ok(())
}

struct OpcodeRunner {
    opcodes: HashSet<Opcodes>,
}

struct PossibilitySets {
    possible: BTreeSet<u32>,
    not_possible: HashSet<u32>,
}

impl PossibilitySets {
    fn new() -> Self {
        Self {
            possible: BTreeSet::new(),
            not_possible: HashSet::new(),
        }
    }
}

struct Possibilities {
    possibile: HashMap<Opcodes, PossibilitySets>,
}

impl Possibilities {
    fn new() -> Self {
        Self {
            possibile: HashMap::new(),
        }
    }

    fn remove(&mut self, opcode: Opcodes, value: u32) -> () {
        let possible = self
            .possibile
            .entry(opcode)
            .or_insert_with(|| PossibilitySets::new());
        possible.not_possible.insert(value);
        possible.possible.remove(&value);
    }

    fn insert(&mut self, opcode: Opcodes, value: u32) -> () {
        let possible = self
            .possibile
            .entry(opcode)
            .or_insert_with(|| PossibilitySets::new());
        if possible.not_possible.contains(&value) {
            return;
        }
        possible.possible.insert(value);
    }
}

impl OpcodeRunner {
    fn new() -> Self {
        Self {
            opcodes: Opcodes::new_set(),
        }
    }

    fn run_program(program: &str, opcode_lookup: HashMap<u32, Opcodes>) -> Result<u32> {
        let mut register = HashMap::from([(0, 0), (1, 0), (2, 0), (3, 0)]);
        for line in program.lines() {
            let order = Instruction::map_order(line)?;
            let opcode = opcode_lookup.get(&order[0]).ok_or_else(|| {
                anyhow!(
                    "order: {:?} has key not in map: {:?} not present",
                    order,
                    opcode_lookup
                )
            })?;
            opcode.invoke(order[1], order[2], order[3], &mut register)?;
        }

        let result = register
            .get(&0)
            .ok_or_else(|| anyhow!("key 0 not present in map: {:?}", register))?;

        Ok(*result)
    }

    fn find_opcode_possibilities(
        &self,
        instructions: &Vec<Instruction>,
    ) -> Result<HashMap<u32, Opcodes>> {
        let mut opcode_possibilities = Possibilities::new();

        for instruction in instructions {
            for opcode in &self.opcodes {
                let mut before = instruction.create_register(instruction.before);

                match opcode.invoke(
                    instruction.order[1],
                    instruction.order[2],
                    instruction.order[3],
                    &mut before,
                ) {
                    Ok(_) => (),
                    Err(_) => opcode_possibilities.remove(*opcode, instruction.order[0]),
                }

                let after = instruction.create_register(instruction.after);
                if before == after {
                    opcode_possibilities.insert(*opcode, instruction.order[0]);
                }
            }
        }

        let mut opcode_lookup: HashMap<u32, Opcodes> = HashMap::new();
        while &opcode_lookup.len() != &opcode_possibilities.possibile.len() {
            for (opcode, possibilities) in &mut opcode_possibilities.possibile {
                if possibilities.possible.len() == 0 {
                    continue;
                }

                possibilities
                    .possible
                    .retain(|&v| !opcode_lookup.contains_key(&v));

                if possibilities.possible.len() > 1 {
                    continue;
                }
                let next = possibilities.possible.pop_first().ok_or_else(|| {
                    anyhow!("there should be one in set: {:?}", possibilities.possible)
                })?;
                opcode_lookup.insert(next, *opcode);
            }
        }

        Ok(opcode_lookup)
    }

    fn find_behave_above_threshold(
        &self,
        instructions: &Vec<Instruction>,
        threshold: u32,
    ) -> Result<u32> {
        let mut above_threshold = 0;
        for instruction in instructions {
            let mut instruction_count = 0;
            for opcode in &self.opcodes {
                let mut before = instruction.create_register(instruction.before);

                match opcode.invoke(
                    instruction.order[1],
                    instruction.order[2],
                    instruction.order[3],
                    &mut before,
                ) {
                    Ok(_) => (),
                    Err(_) => continue,
                }

                let after = instruction.create_register(instruction.after);
                if before == after {
                    instruction_count += 1;
                }
            }
            if instruction_count > threshold {
                above_threshold += 1;
            }
        }
        Ok(above_threshold)
    }
}

#[derive(Debug)]
struct Instruction {
    before: [u32; 4],
    order: [u32; 4],
    after: [u32; 4],
}

impl Instruction {
    fn create_register(&self, array: [u32; 4]) -> HashMap<u32, u32> {
        let mut register = HashMap::with_capacity(4);
        for (idx, element) in array.iter().enumerate() {
            register.insert(idx as u32, *element);
        }
        register
    }

    fn from_several(input: &str) -> Result<Vec<Self>> {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let mut instructions: Vec<Instruction> = Vec::with_capacity(parts.len());
        for part in parts {
            instructions.push(Instruction::from(part)?);
        }
        Ok(instructions)
    }

    fn from(input: &str) -> Result<Self> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.len() != 3 {
            return Err(anyhow!("not able to parse instruction: {}", input));
        }
        let before = Instruction::map_register(lines[0])?;
        let after = Instruction::map_register(lines[2])?;
        let order = Instruction::map_order(lines[1])?;

        Ok(Self {
            before,
            order,
            after,
        })
    }

    fn map_order(line: &str) -> Result<[u32; 4]> {
        let order: Vec<u32> = line.split(" ").flat_map(|c| c.parse()).collect();

        if order.len() != 4 {
            return Err(anyhow!("not able to map order: {}", line));
        }

        Ok([order[0], order[1], order[2], order[3]])
    }

    fn map_register(line: &str) -> Result<[u32; 4]> {
        let register = &line[9..line.len() - 1];
        let register: Vec<u32> = register
            .split(", ")
            .flat_map(|c| c.parse::<u32>())
            .collect();
        if register.len() != 4 {
            return Err(anyhow!("not able to parse register: {}", line));
        }
        Ok([register[0], register[1], register[2], register[3]])
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Opcodes {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcodes {
    fn new_set() -> HashSet<Opcodes> {
        let set = HashSet::from([
            Opcodes::Addr,
            Opcodes::Addi,
            Opcodes::Mulr,
            Opcodes::Muli,
            Opcodes::Banr,
            Opcodes::Bani,
            Opcodes::Borr,
            Opcodes::Bori,
            Opcodes::Setr,
            Opcodes::Seti,
            Opcodes::Gtir,
            Opcodes::Gtri,
            Opcodes::Gtrr,
            Opcodes::Eqir,
            Opcodes::Eqri,
            Opcodes::Eqrr,
        ]);
        set
    }

    fn invoke(
        &self,
        input_a: u32,
        input_b: u32,
        output: u32,
        register: &mut HashMap<u32, u32>,
    ) -> Result<()> {
        let value = match self {
            Opcodes::Addr => {
                let a = self.get_or_error(input_a, register)?;
                let b = self.get_or_error(input_b, register)?;
                a + b
            }
            Opcodes::Addi => {
                let a = self.get_or_error(input_a, register)?;
                a + input_b
            }
            Opcodes::Mulr => {
                let a = self.get_or_error(input_a, register)?;
                let b = self.get_or_error(input_b, register)?;
                a * b
            }
            Opcodes::Muli => {
                let a = self.get_or_error(input_a, register)?;
                a * input_b
            }
            Opcodes::Banr => {
                let a = self.get_or_error(input_a, register)?;
                let b = self.get_or_error(input_b, register)?;
                a & b
            }
            Opcodes::Bani => {
                let a = self.get_or_error(input_a, register)?;
                a & input_b
            }
            Opcodes::Borr => {
                let a = self.get_or_error(input_a, register)?;
                let b = self.get_or_error(input_b, register)?;
                a | b
            }
            Opcodes::Bori => {
                let a = self.get_or_error(input_a, register)?;
                a | input_b
            }
            Opcodes::Setr => {
                let a = self.get_or_error(input_a, register)?;
                a
            }
            Opcodes::Seti => input_a,
            Opcodes::Gtir => {
                let b = self.get_or_error(input_b, register)?;
                if input_a > b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Gtri => {
                let a = self.get_or_error(input_b, register)?;
                if a > input_b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Gtrr => {
                let a = self.get_or_error(input_a, register)?;
                let b = self.get_or_error(input_b, register)?;
                if a > b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Eqir => {
                let b = self.get_or_error(input_b, register)?;
                if input_a == b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Eqri => {
                let a = self.get_or_error(input_a, register)?;
                if a == input_b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Eqrr => {
                let a = self.get_or_error(input_a, register)?;
                let b = self.get_or_error(input_b, register)?;
                if a == b {
                    1
                } else {
                    0
                }
            }
        };
        register.insert(output, value);
        Ok(())
    }

    fn get_or_error(&self, input: u32, register: &HashMap<u32, u32>) -> Result<u32> {
        let value = *register
            .get(&input)
            .ok_or_else(|| anyhow!("not able to get input: {} in {:?}", input, register))?;
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_new() -> Result<()> {
        // Arrange
        let input = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";

        // Act
        let instructon = Instruction::from(input)?;

        // Assert
        assert_eq!(instructon.after[0], 3);
        Ok(())
    }
}
