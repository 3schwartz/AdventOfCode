use anyhow::Result;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day17_data.txt")?;

    let mut computer = Computer::from_str(&input)?;
    let output = computer.run();

    println!("Part 1: {}", output.unwrap());

    let computer = Computer::from_str(&input)?;
    let a = computer.find_a();

    println!("Part 2: {}", a);

    Ok(())
}

use anyhow::anyhow;
use std::str::FromStr;

#[derive(Clone)]
struct Computer {
    a: i32,
    b: i32,
    c: i32,
    program: Vec<i32>,
}

impl Computer {
    fn find_a(&self) -> i32 {
        let mut a = 0;
        let expected = Self::output(&self.program);
        loop {
            let mut clone = self.clone();
            clone.a = a;
            let output = clone.run();
            if let Some(o) = output {
                if o == expected {
                    break;
                }
            }
            a += 1;
        }
        a
    }

    fn run(&mut self) -> Option<String> {
        let mut pointer = 0;
        let mut output: Vec<i32> = vec![];
        let mut seen = HashSet::new();

        loop {
            if pointer >= self.program.len() {
                break;
            }
            if !seen.insert((pointer, self.a, self.b, self.c, Self::output(&output))) {
                return None;
            }
            let opcode = self.program[pointer];
            let operand = self.program[pointer + 1];

            match opcode {
                0 => self.a = self.a / (2_i32.pow(self.combo_operand(operand) as u32)),
                1 => self.b ^= operand,
                2 => self.b = self.combo_operand(operand) % 8,
                3 => {
                    if self.a != 0 {
                        pointer = operand as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    output.push(self.combo_operand(operand) % 8);
                }
                6 => self.b = self.a / (2_i32.pow(self.combo_operand(operand) as u32)),
                7 => self.c = self.a / (2_i32.pow(self.combo_operand(operand) as u32)),
                _ => panic!("got opcode {}", operand),
            };

            pointer += 2;
        }

        Some(Self::output(&output))
    }

    fn output(output: &Vec<i32>) -> String {
        output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn combo_operand(&self, operand: i32) -> i32 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("got operand {}", operand),
        }
    }
}

impl FromStr for Computer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        let registers = parts[0].split("\n").collect::<Vec<&str>>();
        assert_eq!(registers.len(), 3);
        let a: i32 = registers[0].split(": ").collect::<Vec<&str>>()[1].parse()?;
        let b: i32 = registers[1].split(": ").collect::<Vec<&str>>()[1].parse()?;
        let c: i32 = registers[2].split(": ").collect::<Vec<&str>>()[1].parse()?;

        let program = parts[1]
            .trim()
            .strip_prefix("Program: ")
            .ok_or_else(|| anyhow!("not able to strip program"))?
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(Self { a, b, c, program })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day17_test_data.txt")?;

        // Act
        let mut computer = Computer::from_str(&input)?;
        let output = computer.run();

        // Assert
        assert!(output.is_some());
        assert_eq!(output.unwrap(), "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day17_test2_data.txt")?;

        // Act
        let computer = Computer::from_str(&input)?;
        let a = computer.find_a();

        // Assert
        assert_eq!(a, 117_440);
        Ok(())
    }
}
