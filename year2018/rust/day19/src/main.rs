use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day19_data.txt")?;
    let program = Program::from(&input)?;

    let part_1 = program.run(BTreeMap::new(), false)?;
    println!("Part 1: {}", part_1);

    let part_2 = program.run(BTreeMap::from([(0, 1)]), false)?;
    println!("Part 2: {}", part_2);

    let div = Program::sum_of_divisors(10551326);
    println!("Part 2 divisors: {}", div);

    Ok(())
}

struct Program {
    bound: u32,
    instructions: HashMap<u32, Instruction>,
}

impl Program {
    fn from(input: &str) -> Result<Self> {
        let possible_bound = input.lines().next();
        let bound = Instruction::get_bound(possible_bound)?;
        let mut instructions = HashMap::new();
        for (idx, line) in input.lines().skip(1).enumerate() {
            let instruction = Instruction::from(line)?;
            instructions.insert(idx as u32, instruction);
        }
        Ok(Self {
            bound,
            instructions,
        })
    }

    /// Part 2 is sum of divisors for a numbers. The high numbers
    /// is in registry 4 when the loops start.
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
                } else {
                    sum += j + n / j;
                }
            }
            j += 1;
        }

        return sum;
    }

    fn run(&self, mut register: BTreeMap<u32, u32>, debug: bool) -> Result<u32> {
        let mut pointer = 0;
        // Used to differentiate between part 1 and part 2.
        let initial_0 = *register.get(&0).unwrap_or(&0);
        loop {
            let Some(instruction) = self.instructions.get(&pointer) else {
                break;
            };
            if debug {
                println!("{}", pointer);
                println!("{:?}", instruction);
                println!("{:?}", register);
            }
            // Relevant for part 2 which is problem of getting all divisors of a high number.
            // The operations are a outer and a inner loop.
            // The outer loop starts with reg 2 = 1 and increases reg 2 all the way up to reg 4 (the high number).
            // The outer loop goes from (3) -> inner loop -> 12, 13, 14, 15.
            // The inner loop validates if the current value of reg 2 is a divisor of reg 4 and adds reg 2 (the divisor)
            // to reg 0 if it is a dvisior.
            if pointer == 3 && initial_0 != 0 {
                let reg_2 = *register.get(&2).ok_or_else(|| anyhow!("should have 2"))?;
                let reg_4 = *register.get(&4).ok_or_else(|| anyhow!("should have 4"))?;
                // First, instructions 3, 4, 5, 6, 8, 9, 10, 11 is run in a loop. This is the inner loop.
                // This one essentially increases registry 5 from 1 to registry 4 (the high number) by one for each iteration
                // and in each iteration checks if reg 2 * reg 5 = reg 4.
                // This can be validated by checking if reg 2 is a divisor of reg 4 => reg 4 % reg 2 == 0.
                if reg_4 % reg_2 == 0 {
                    // If it is a divisor reg 2 (the divisor) is added to reg 0 - hence the sum of divisors.
                    let reg_0 = register.get(&0).ok_or_else(|| anyhow!("should have 0"))?;
                    register.insert(0, reg_0 + reg_2);
                };
                // Is overwritten in outer loop.
                register.insert(1, 1);
                // The inner loop goes from 1 to reg 4 in reg 5. Hence last step of inner loop is having these equal.
                register.insert(5, reg_4);
                // When reg 4 == reg 5 then one leaves the inner loop.
                pointer = 12;
                // The outer loop now increases reg 2, and the outer loop is broken when reg 2 > reg 4.
                continue;
            }

            register.insert(self.bound, pointer);

            instruction.opcode.invoke(
                instruction.input_a,
                instruction.input_b,
                instruction.output,
                &mut register,
            )?;

            pointer = *register
                .get(&self.bound)
                .ok_or_else(|| anyhow!("should exists: {:?}", register))?;

            pointer += 1;
        }
        let reg_0 = register
            .get(&0)
            .ok_or_else(|| anyhow!("registry missing 0: {:?}", register))?;
        Ok(*reg_0)
    }
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
