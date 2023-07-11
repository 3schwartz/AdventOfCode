use std::{fs, collections::{BTreeMap, HashSet}};

use anyhow::{Result, anyhow};
use program::Program;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day21_data.txt")?;
    let program = Program::from(&input)?;

    run(&program, &mut BTreeMap::new())?;

    Ok(())
}

/// Looking at the input program only instruction 28, eqrr 5 0 2, uses registry 0
/// which is the only one which can be changed.
/// When registry 0 is equal to 5 then the program halts.
/// 
/// The shortest program, part 1, hence needs to look at the value of registry 5 the first time this instruction is run.
/// 
/// The longest program, part 2, needs to look at when a value for registry 5 is seen twice. Then
/// use the value of registry 5 the last time the instruction was run.
fn run(program: &Program, register: &mut BTreeMap<u128, u128>) -> Result<()> {
    let mut pointer = 0;

    let mut seen_first = false;

    let mut last = 0;
    let mut seen = HashSet::new();
    loop {
        let Some(instruction) = program.instructions.get(&pointer) else {
            break;
        };
    

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
            if !seen_first {
                println!("Part 1: {}", now);
                seen_first = true;
            }
            if !seen.insert(now) {
                println!("Part 2: {}", last);
                break;
            }
            last = now;
        }
        
        pointer += 1;
    }
    Ok(())
}
