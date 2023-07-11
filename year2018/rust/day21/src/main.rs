use std::{fs, collections::{BTreeMap, HashSet}};

use anyhow::{Result, anyhow};
use program::Program;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day21_data.txt")?;
    let program = Program::from(&input)?;

    run(&program, &mut BTreeMap::new())?;

    Ok(())
}

fn run(program: &Program, register: &mut BTreeMap<u128, u128>) -> Result<u32> {
    let mut pointer = 0;
    let mut count = 0;
    let mut last = 0;
    let mut seen = HashSet::new();
    loop {
        let Some(instruction) = program.instructions.get(&pointer) else {
            break;
        };
        
        count += 1;

        register.insert(program.bound, pointer);

        instruction.opcode.invoke(
            instruction.input_a,
            instruction.input_b,
            instruction.output,
            register,
        )?;

        pointer = *register
            .get(&program.bound)
            .ok_or_else(|| anyhow!("should exists: {:?}", register))?;

        if pointer == 28 {
            let now = *register
            .get(&5)
            .unwrap_or(&0);
            if !seen.insert(now) {
                println!("{}", now);
                println!("{}", last);
                break;
            }
            last = now;
        }
        
        pointer += 1;
    }
    Ok(count)
}
