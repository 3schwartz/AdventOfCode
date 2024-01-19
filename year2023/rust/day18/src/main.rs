use std::fs;

use anyhow::Result;

struct Step {
    direction: String,
    steps: i64,
}

impl Step {
    fn from(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        Ok(Self {
            direction: parts[0].to_string(),
            steps: parts[1].parse()?,
        })
    }

    fn from_hex(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let hexadecimal = parts[2].trim_start_matches('(').trim_end_matches(')');
        let hex_direction = &hexadecimal[hexadecimal.len() - 1..hexadecimal.len()];
        let direction = match hex_direction {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!("{}", hex_direction),
        };
        let steps = i64::from_str_radix(&hexadecimal[1..hexadecimal.len() - 1], 16)?;
        Ok(Self {
            direction: direction.to_string(),
            steps,
        })
    }
}

fn get_steps(input: &str, hexadecimal: bool) -> Result<Vec<Step>> {
    let mut steps = vec![];
    for line in input.lines() {
        let step = if hexadecimal {
            Step::from_hex(line)?
        } else {
            Step::from(line)?
        };
        steps.push(step);
    }
    Ok(steps)
}

/// The flow is counter wise and hence opposite signs
/// are used in the triangle formula.
/// https://en.wikipedia.org/wiki/Shoelace_formula
fn shoelace_formula(steps: &Vec<Step>) -> i64 {
    let mut position = (0, 0);
    let mut positions = vec![position];
    for step in steps {
        position = match step.direction.as_str() {
            "R" => (position.0 + step.steps, position.1),
            "U" => (position.0, position.1 + step.steps),
            "L" => (position.0 - step.steps, position.1),
            "D" => (position.0, position.1 - step.steps),
            _ => panic!("{}", step.direction),
        };
        positions.push(position);
    }
    let mut area = 0;
    for i in 0..positions.len() {
        area -= positions[i].0 * positions[(i + 1) % positions.len()].1;
        area += positions[(i + 1) % positions.len()].0 * positions[i].1;
    }
    area / 2
}

fn perimeter(steps: &[Step]) -> i64 {
    steps.iter().map(|s| s.steps).sum()
}

/// [shoelace_formula] gives interier but only half of the
/// area in the perimeter. Hence we need to add half of the
/// size of the perimeter.
/// 1 is added to account for the starting point.
fn get_area(steps: &Vec<Step>) -> i64 {
    shoelace_formula(steps) + perimeter(steps) / 2 + 1
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day18_data.txt")?;
    let steps = get_steps(&input, false)?;
    let area = get_area(&steps);

    println!("Part 1: {}", area);

    let steps = get_steps(&input, true)?;
    let area = get_area(&steps);

    println!("Part 2: {}", area);
    Ok(())
}
