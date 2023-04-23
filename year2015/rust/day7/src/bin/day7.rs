use std::{fs, collections::HashMap};
use day7::{self, AoCResult, find_signal, AoCError};

fn main() -> AoCResult<()> {
    let input = fs::read_to_string("../data/day7_data.txt").unwrap();
    let lines = input.lines();
    let mut signals: HashMap<&str, u16> = HashMap::new();

    find_signal(&mut signals, &lines, true)?;

    let a = signals.get("a").ok_or(AoCError::NoResult)?;
    println!("Part 1: {}", a);

    let mut signals_part2: HashMap<&str, u16> = HashMap::from([("b", *a)]);
    
    find_signal(&mut signals_part2, &lines, true)?;
    let a_part2 = signals_part2.get("a").ok_or(AoCError::NoResult)?;
    println!("Part 2: {}", a_part2);

    let mut foos: HashMap<&str, u16> = HashMap::new();
    let foo = input.trim().split('\n').collect::<Vec<&str>>();
    find_signal(&mut foos, &foo, true)?;

    for a in foo {
        println!("{a}");
    }

    Ok(())
}
