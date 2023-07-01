use std::{collections::{HashMap, BTreeMap}, fs};

use anyhow::{anyhow, Result};

fn sum_of_divisors(n: u32) -> u32 {
    let mut sum = 0;
    let mut j = 1;
    loop {
        if j * j > n {
            break;
        }
        if (n % j) == 0 {
            if n / j == j {
                sum += j;
            }
            else {
                sum += j + n / j;
            }
        }
        j+=1;
    }
    
    return sum;
}

fn main() -> Result<()> {
    let div = sum_of_divisors(10551326);

    println!("{}", div);
    let input = fs::read_to_string("../data/day19_data.txt")?;

    let possible_bound = input.lines().next();
    let bound = Instruction::get_bound(possible_bound)?;
    let mut instructions = HashMap::new();
    for (idx, line) in input.lines().skip(1).enumerate() {
        let instruction = Instruction::from(line)?;
        instructions.insert(idx as u32, instruction);
    }


    // let mut register = HashMap::new();
    let debug = true;
    let mut register = BTreeMap::from([(0, 1)]);
    let mut pointer = 0;
    loop {
        let Some(instruction) = instructions.get(&pointer) else {
            break;
        };
        if debug {
            
            println!("{}", pointer);
            println!("{:?}", instruction);
            println!("{:?}", register);   
        }
        if pointer == 3 {
            let reg_2 = *register.get(&2).ok_or_else(|| anyhow!("should have 2"))?;
            let reg_4 = *register.get(&4).ok_or_else(|| anyhow!("should have 4"))?;
            if reg_4 % reg_2 == 0 {
                let reg_0 = register.get(&0).ok_or_else(|| anyhow!("should have 0"))?;
                register.insert(0, reg_0 + reg_2);
            };
            register.insert(1, 1);
            register.insert(5, reg_4);
            pointer = 12;
            continue;
        }

        register.insert(bound, pointer);

        instruction.opcode.invoke(
            instruction.input_a,
            instruction.input_b,
            instruction.output,
            &mut register,
        )?;

        pointer = *register
            .get(&bound)
            .ok_or_else(|| anyhow!("should exists: {:?}", register))?;

        pointer += 1;
    }
    let part_1 = register
        .get(&0)
        .ok_or_else(|| anyhow!("registry missing 0: {:?}", register))?;

    println!("Part 1: {}", part_1);

    Ok(())
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcodes,
    input_a: u32,
    input_b: u32,
    output: u32,
}

impl Instruction {
    fn from(input: &str) -> Result<Self> {
        let parts = input.split(" ").collect::<Vec<&str>>();
        if parts.len() != 4 {
            return Err(anyhow!("not able to map line: {}", input));
        }
        let opcode = Opcodes::from(parts[0])?;
        let instruction = Instruction {
            opcode,
            input_a: parts[1].parse()?,
            input_b: parts[2].parse()?,
            output: parts[3].parse()?,
        };
        Ok(instruction)
    }

    fn get_bound(possible: Option<&str>) -> Result<u32> {
        let Some(line) = possible else {
            return Err(anyhow!("possible is none"));
        };
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() != 2 {
            return Err(anyhow!("not able to find bound: {}", line));
        }
        Ok(parts[1].parse()?)
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
    fn from(input: &str) -> Result<Self> {
        let code = match input {
            "addr" => Opcodes::Addr,
            "addi" => Opcodes::Addi,
            "mulr" => Opcodes::Mulr,
            "muli" => Opcodes::Muli,
            "banr" => Opcodes::Banr,
            "bani" => Opcodes::Bani,
            "borr" => Opcodes::Borr,
            "bori" => Opcodes::Bori,
            "setr" => Opcodes::Setr,
            "seti" => Opcodes::Seti,
            "gtir" => Opcodes::Gtir,
            "gtri" => Opcodes::Gtri,
            "gtrr" => Opcodes::Gtrr,
            "eqir" => Opcodes::Eqir,
            "eqri" => Opcodes::Eqri,
            "eqrr" => Opcodes::Eqrr,
            _ => return Err(anyhow!("not able to map opcode: {}", input)),
        };
        Ok(code)
    }

    fn invoke(
        &self,
        input_a: u32,
        input_b: u32,
        output: u32,
        register: &mut BTreeMap<u32, u32>,
    ) -> Result<()> {
        let value = match self {
            Opcodes::Addr => {
                let a = self.get_or_insert(input_a, register);
                let b = self.get_or_insert(input_b, register);
                a + b
            }
            Opcodes::Addi => {
                let a = self.get_or_insert(input_a, register);
                a + input_b
            }
            Opcodes::Mulr => {
                let a = self.get_or_insert(input_a, register);
                let b = self.get_or_insert(input_b, register);
                a * b
            }
            Opcodes::Muli => {
                let a = self.get_or_insert(input_a, register);
                a * input_b
            }
            Opcodes::Banr => {
                let a = self.get_or_insert(input_a, register);
                let b = self.get_or_insert(input_b, register);
                a & b
            }
            Opcodes::Bani => {
                let a = self.get_or_insert(input_a, register);
                a & input_b
            }
            Opcodes::Borr => {
                let a = self.get_or_insert(input_a, register);
                let b = self.get_or_insert(input_b, register);
                a | b
            }
            Opcodes::Bori => {
                let a = self.get_or_insert(input_a, register);
                a | input_b
            }
            Opcodes::Setr => {
                let a = self.get_or_insert(input_a, register);
                a
            }
            Opcodes::Seti => input_a,
            Opcodes::Gtir => {
                let b = self.get_or_insert(input_b, register);
                if input_a > b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Gtri => {
                let a = self.get_or_insert(input_b, register);
                if a > input_b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Gtrr => {
                let a = self.get_or_insert(input_a, register);
                let b = self.get_or_insert(input_b, register);
                if a > b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Eqir => {
                let b = self.get_or_insert(input_b, register);
                if input_a == b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Eqri => {
                let a = self.get_or_insert(input_a, register);
                if a == input_b {
                    1
                } else {
                    0
                }
            }
            Opcodes::Eqrr => {
                let a = self.get_or_insert(input_a, register);
                let b = self.get_or_insert(input_b, register);
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

    fn get_or_insert(&self, input: u32, register: &BTreeMap<u32, u32>) -> u32 {
        let value = *register.get(&input).unwrap_or(&0);
        value
    }
}
