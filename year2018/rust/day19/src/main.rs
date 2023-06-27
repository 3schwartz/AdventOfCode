use std::{fs, collections::HashMap};

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day19_data.txt")?;

    let mut bound;
    for (usize, line) in input.lines().enumerate() {
        if usize == 0 {
            bound = get_bound(line)?;
        }

    }
    println!("Hello, world!");

    Ok(())
}

fn get_bound(line: &str) -> Result<usize> {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() != 2 {
        return Err(anyhow!("not able to find bound: {}", line))
    }
    Ok(parts[1].parse()?)
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
    fn new_lookup() -> HashMap<String, Opcodes> {
        HashMap::from([
            ("addr".to_owned(), Opcodes::Addr),
            ("addi".to_owned(), Opcodes::Addi),
            ("mulr".to_owned(), Opcodes::Mulr),
            ("muli".to_owned(), Opcodes::Muli),
            ("banr".to_owned(), Opcodes::Banr),
            ("bani".to_owned(), Opcodes::Bani),
            ("borr".to_owned(), Opcodes::Borr),
            ("bori".to_owned(), Opcodes::Bori),
            ("setr".to_owned(), Opcodes::Setr),
            ("seti".to_owned(), Opcodes::Seti),
            ("gtir".to_owned(), Opcodes::Gtir),
            ("gtri".to_owned(), Opcodes::Gtri),
            ("gtrr".to_owned(), Opcodes::Gtrr),
            ("eqir".to_owned(), Opcodes::Eqir),
            ("eqri".to_owned(), Opcodes::Eqri),
            ("eqrr".to_owned(), Opcodes::Eqrr),
        ])
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
