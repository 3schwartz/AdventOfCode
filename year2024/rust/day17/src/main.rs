use anyhow::Result;
use std::{collections::HashSet, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day17_data.txt")?;

    let mut computer = Computer::from_str(&input)?;
    let output = computer.run();

    println!("Part 1: {}", output.unwrap());

    let computer = Computer::from_str(&input)?;
    let part_2 = computer.solve();

    println!("Part 2: {}", part_2);

    Ok(())
}

use anyhow::anyhow;
use std::str::FromStr;

#[derive(Clone)]
struct Computer {
    a: u128,
    b: u128,
    c: u128,
    program: Vec<u128>,
}

impl Computer {
    fn solve(&self) -> u128 {
        let mut queue = vec![(0, self.program.len())];
        let mut solutions = vec![];
        while let Some((prior_a, prior_index)) = queue.pop() {
            if prior_index == 0 {
                solutions.push(prior_a);
                continue;
            }
            let index = prior_index - 1;
            let expected_output = self.program[index];
            for i in 0..8 {
                // only look at last 3 bits
                let a_in = (prior_a << 3) + i; // shift left since only i (and hence last 3 bits) should be evaluated
                let (a_out, output) = Computer::decompile(a_in);
                if output == expected_output && a_out == prior_a {
                    queue.push((a_in, index));
                }
            }
        }
        *solutions.iter().min().unwrap()
    }

    /// The program execution only depends on the last three
    /// bits of `a` since it starts with:
    /// b = a % 8
    ///
    /// Later, we have:
    /// c = a >> b
    /// This introduces a dependency on higher bits of `a`.
    /// Likewise:
    /// b ^= c
    /// may temporarily depend on higher bits. However, the final
    /// modulo operation ensures only the last three bits of `b`
    /// are used:
    /// output = b % 8
    ///
    /// The value of `a` is then shifted for the next operation,
    /// which can be seen as examining the next three bits:
    /// a = a >> 3;
    ///
    /// Program: 2,4, 1,1, 7,5, 1,5, 0,3, 4,4, 5,5, 3,0
    fn decompile(mut a: u128) -> (u128, u128) {
        // 2,4: combo(4) % 8 -> B, B = A % 8
        let mut b = a % 8; // or b = a and 7

        // 1,1: B = B XOR 1
        b ^= 1;

        // 7,5: A / 2**combo(5) -> C, C = A / 2**B
        let c = a / 2_u128.pow(b as u32); // or c = a >> b;

        // 1,5: B = B XOR 5
        b ^= 5;

        // 0,3: A / 2**combo(3) -> A, A = A / 2**3
        a /= 2_u128.pow(3); // or a = a >> 3;

        // 4,4: B = B XOR C
        b ^= c;

        // 5,5: out -> B % 8
        // 3,0: A == 0 -> noothing else pointer = 0
        (a, b % 8)
    }

    #[cfg(test)]
    fn find_a(&self) -> u128 {
        let mut a = 0;
        let expected = Self::output(&self.program);
        loop {
            if a % 1_000_000 == 0 {
                println!("{a}");
            }
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
        let mut output: Vec<u128> = vec![];
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
                0 => self.a /= 2_u128.pow(self.combo_operand(operand) as u32),
                1 => self.b ^= operand,
                2 => self.b = self.combo_operand(operand) % 8,
                3 => {
                    if self.a != 0 {
                        pointer = operand as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => output.push(self.combo_operand(operand) % 8),
                6 => self.b = self.a / (2_u128.pow(self.combo_operand(operand) as u32)),
                7 => self.c = self.a / (2_u128.pow(self.combo_operand(operand) as u32)),
                _ => panic!("got opcode {}", operand),
            };

            pointer += 2;
        }

        Some(Self::output(&output))
    }

    fn output(output: &[u128]) -> String {
        output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn combo_operand(&self, operand: u128) -> u128 {
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
        let a: u128 = registers[0].split(": ").collect::<Vec<&str>>()[1].parse()?;
        let b: u128 = registers[1].split(": ").collect::<Vec<&str>>()[1].parse()?;
        let c: u128 = registers[2].split(": ").collect::<Vec<&str>>()[1].parse()?;

        let program = parts[1]
            .trim()
            .strip_prefix("Program: ")
            .ok_or_else(|| anyhow!("not able to strip program"))?
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<u128>, _>>()?;

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
