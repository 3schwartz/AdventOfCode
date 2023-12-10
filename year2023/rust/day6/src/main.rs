use std::{fs, iter::zip};

use anyhow::Result;

/// Quadratic equation
fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day6_data.txt")?;

    let races = parse_part_1(&input);
    let part_1 = total_solution(&races);

    println!("Part 1: {}", part_1);

    let joined = parse_part_2(&input)?;
    let part_2 = total_solution(&joined);

    println!("Part 2: {}", part_2);
    Ok(())
}

fn total_solution(races: &[(f64, f64)]) -> f64 {
    let mut total = 1_f64;
    for equation in races {
        let time = equation.0;
        let distance = equation.1;
        let d = time.powf(2_f64) - 4_f64 * distance;
        if d < 0.0 {
            continue;
        }
        let mut top = (time + d.sqrt()) / 2_f64;
        let mut down = (time - d.sqrt()) / 2_f64;
        top = round(top, true);
        down = round(down, false);
        total *= top - down + 1_f64;
    }
    total
}

fn round(n: f64, down: bool) -> f64 {
    if down {
        if n % 1_f64 == 0_f64 {
            return n - 1_f64;
        }
        return n.floor();
    }
    if n % 1_f64 == 0_f64 {
        return n + 1_f64;
    }
    n.ceil()
}

fn parse_part_2(input: &str) -> Result<Vec<(f64, f64)>> {
    let mut rows = vec![];
    for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .filter_map(|c| c.parse::<u32>().ok())
            .map(|n| n.to_string())
            .collect::<String>();
        rows.push(numbers.parse::<f64>()?);
    }
    Ok(vec![(rows[0], rows[1])])
}

fn parse_part_1(input: &str) -> Vec<(f64, f64)> {
    let mut rows = vec![];
    for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .filter_map(|c| c.parse().ok())
            .collect::<Vec<f64>>();
        rows.push(numbers);
    }
    let time = &rows[0];
    let distance = &rows[1];
    let races: Vec<(f64, f64)> = zip(time, distance).map(|(x, y)| (*x, *y)).collect();
    races
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day6_data_test.txt")?;
        let joined = parse_part_2(&input)?;

        // Act
        let part_2 = total_solution(&joined);

        // Assert
        assert_eq!(part_2, 71503.0);
        Ok(())
    }
}
