use std::{collections::HashSet, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day22_data.txt")?;
    let (infected, position) = parse_grid(&input);

    let actual = iterate(&infected, position, 10_000);
    println!("Part 1: {actual}");
    Ok(())
}

fn iterate(
    initial_infected: &HashSet<(i32, i32)>,
    initial_position: (i32, i32),
    count: usize,
) -> usize {
    let mut infected = initial_infected.clone();
    let mut position = initial_position;
    let mut direction = (0, -1);
    let mut total_infected = 0;

    for _ in 0..count {
        let mut infection = false;
        (position, direction, infection) = burst(&mut infected, position, direction);
        if infection {
            total_infected += 1
        }
    }
    total_infected
}
fn burst(
    infected: &mut HashSet<(i32, i32)>,
    position: (i32, i32),
    direction: (i32, i32),
) -> ((i32, i32), (i32, i32), bool) {
    let next_direction = if infected.contains(&position) {
        (-direction.1, direction.0)
    } else {
        (direction.1, -direction.0)
    };
    let infection = if !infected.remove(&position) {
        infected.insert(position);
        true
    } else {
        false
    };
    (
        (position.0 + next_direction.0, position.1 + next_direction.1),
        next_direction,
        infection,
    )
}

fn parse_grid(input: &str) -> (HashSet<(i32, i32)>, (i32, i32)) {
    let mut row_max = 0;
    let mut col_max = 0;
    let mut infected = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        row_max = row;
        for (col, c) in line.chars().enumerate() {
            col_max = col;
            if c == '#' {
                infected.insert((col as i32, row as i32));
            }
        }
    }
    let col = (col_max as i32) / 2;
    let row = (row_max as i32) / 2;
    (infected, (row, col))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day22_test_data.txt")?;
        let (infected, position) = parse_grid(&input);

        // Act
        let actual = iterate(&infected, position, 10_000);

        // Assert
        assert_eq!(actual, 5_587);
        Ok(())
    }
}
