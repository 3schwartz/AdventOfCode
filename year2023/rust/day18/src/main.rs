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

/// Pick's theorem says that the area A of a polygon is
/// A = i + b / 2 - 1
/// where i is the number of integer points interier to the polygon
/// and b is the number of integer points on its boundary.
/// https://en.wikipedia.org/wiki/Pick%27s_theorem
/// 
/// We see the coordinate system as points in the center of 
/// squares. Since we want to calculate the area covered by squares
/// we want to find the sum of the squares on the boundary plus
/// the sum of interier integer points. Hence
/// i + b
/// 
/// By rearranging Picks theorem I have
/// i = A - b / 2 + 1
/// and then by adding b to both sides I have
/// i + b = A + b / 2 + 1
/// 
/// Hence I need to find A and b.
/// b is just the sum of the squares on the boundary.
/// A can be found by using [shoelace_formula].
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
