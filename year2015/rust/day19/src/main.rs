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
        let lookup_length = lookup.len();

        for i in 0..(molecule.len() - lookup_length+1) {
            let part = &molecule[i..i+lookup_length];
            if part != lookup {
                continue;
            }
            let start = &molecule[0..i];
            let end = &molecule[i+lookup_length..];
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
    dfs(final_molecule, &mut seen, &replacements, 0, u128::MAX)
        .ok_or_else(|| anyhow!("not able to find min steps"))
}


fn dfs(molecule: &str, seen: &mut HashSet<String>, replacements: &Vec<(&str, &str)>, steps: u128, mut min_steps: u128) -> Option<u128> {
    let current_debt = steps + 1;
    if current_debt >= min_steps {
        return None
    }
    // println!("Debt: {}", current_debt);
    // println!("Seen: {}", seen.len());
    for (insert, lookup) in replacements {
        if lookup.len() > molecule.len() {
            continue;
        }
        let lookup_length = lookup.len();

        for i in 0..(molecule.len() - lookup_length+1) {
            let part = &molecule[i..i+lookup_length];
            if part != *lookup {
                continue;
            }
            let start = &molecule[0..i];
            let end = &molecule[i+lookup_length..];
            let new = [start, insert, end].concat();

            if new == "e" {
                return Some(current_debt);
            }

            if !seen.insert(new.clone()) {
                continue;
            }

            let Some(inner_steps) = dfs(&new, seen, replacements, current_debt, min_steps) else {
                continue;
            };
            if inner_steps < min_steps {
                println!("Steps: {}, min: {}", current_debt, inner_steps);
                min_steps = inner_steps;
            }
        }
    }
    Some(min_steps)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../data/day19_test2_data.txt")?;

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