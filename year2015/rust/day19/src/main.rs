use std::{fs, collections::HashSet};

use anyhow::{Result, anyhow};

fn split_by<'a, 'b>(input: &'a str, pattern: &'b str) -> Result<(&'a str, &'a str)> {
    let split: Vec<&str> = input.split(pattern).collect();
    let first = *split.get(0).ok_or_else(|| anyhow!("{:?}", split))?;
    let second = *split.get(1).ok_or_else(|| anyhow!("{:?}", split))?;
    Ok((first, second))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day19_data.txt")?;

    let distrinct_molecyles = distrinct_molecyles(&input, false)?;
    
    println!("Part 1: {}", distrinct_molecyles);

    let steps = create_molecyle_in_steps(&input)?;

    println!("Part 2: {}", steps);

    Ok(())
}

fn distrinct_molecyles(input: &str, debug: bool) -> Result<usize> {
    let (replacement_string, molecule) = split_by(input, "\n\n")?;
    let mut molecyles = HashSet::new();

    for line in replacement_string.lines() {
        let (lookup, insert) = split_by(line, " => ")?;
        let insert_length = lookup.len();

        for i in 0..(molecule.len() - insert_length+1) {
            let part = &molecule[i..i+insert_length];
            if part != lookup {
                continue;
            }
            let start = &molecule[0..i];
            let end = &molecule[i+insert_length..];
            let new = [start, insert, end].concat();
            if debug {
                println!("{}", new);
            }
            molecyles.insert(new);
        }
        if debug {
            println!("{}", line);
        }
    }
    Ok(molecyles.len())
}

fn create_molecyle_in_steps(input: &str) -> Result<u128> {
    let (replacement_string, final_molecule) = split_by(input, "\n\n")?;
    let mut replacements = vec![];
    for line in replacement_string.lines() {
        let (lookup, insert) = split_by(line, " => ")?;
        replacements.push((lookup, insert));
    }

    let mut seen = HashSet::from([final_molecule.to_string()]);
    let mut idx: u128 = 0;
    loop {
        println!("Idx: {}", idx);
        if seen.is_empty() {
            break;
        }
        idx+=1;
        println!("Size: {}", seen.len());
        let mut nexts = HashSet::new();

        for molecule in seen {
            for (insert, lookup) in &replacements {
                if lookup.len() > molecule.len() {
                    continue;
                }
                let insert_length = lookup.len();

                for i in 0..(molecule.len() - insert_length+1) {
                    let part = &molecule[i..i+insert_length];
                    if part != *lookup {
                        continue;
                    }
                    let start = &molecule[0..i];
                    let end = &molecule[i+insert_length..];
                    let new = [start, insert, end].concat();

                    if new == "e" {
                        return Ok(idx);
                    }

                    nexts.insert(new);
                }
            }
        }
        seen = nexts;
    }
    Err(anyhow!("not able to find steps"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day19_test2_data.txt")?;

        // Act
        let steps = create_molecyle_in_steps(&input)?;

        // Assert
        assert_eq!(3, steps);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day19_test_data.txt")?;

        // Act
        let distrinct_molecyles = distrinct_molecyles(&input, false)?;

        // Assert
        assert_eq!(4, distrinct_molecyles);
        Ok(())
    }

}