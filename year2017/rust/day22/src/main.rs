use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day22_data.txt")?;
    let (infected, position) = parse_grid(&input);

    let actual = iterate(&infected, position, 10_000);
    println!("Part 1: {actual}");

    let actual = iterate_complex(&infected, position, 10_000_000);
    println!("Part 2: {actual}");
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
        let infection = burst(&mut infected, &mut position, &mut direction);
        if infection {
            total_infected += 1
        }
    }
    total_infected
}

fn burst(
    infected: &mut HashSet<(i32, i32)>,
    position: &mut (i32, i32),
    direction: &mut (i32, i32),
) -> bool {
    let next_direction = if infected.contains(position) {
        // right
        (-direction.1, direction.0)
    } else {
        // left
        (direction.1, -direction.0)
    };
    let prior_position = position.to_owned();
    *direction = next_direction;
    *position = (position.0 + direction.0, position.1 + direction.1);

    if !infected.remove(&prior_position) {
        infected.insert(prior_position);
        true
    } else {
        false
    }
}

fn iterate_complex(
    initial_infected: &HashSet<(i32, i32)>,
    initial_position: (i32, i32),
    count: usize,
) -> usize {
    let mut infected = Node::parse_grid(initial_infected);
    let mut position = initial_position;
    let mut direction = (0, -1);
    let mut total_infected = 0;

    for _ in 0..count {
        let infection = burst_complex(&mut infected, &mut position, &mut direction);
        if infection {
            total_infected += 1
        }
    }
    total_infected
}

fn burst_complex(
    infected: &mut HashMap<(i32, i32), Node>,
    position: &mut (i32, i32),
    direction: &mut (i32, i32),
) -> bool {
    let mut infection = false;
    let next_direction = if let Some(node) = infected.get(position).copied() {
        match node {
            Node::Weakened => {
                infection = true;
                infected.insert(position.to_owned(), Node::Infected);
                // same
                direction.to_owned()
            }
            Node::Infected => {
                infected.insert(position.to_owned(), Node::Flagged);
                // right
                (-direction.1, direction.0)
            }
            Node::Flagged => {
                infected.remove(position);
                // reverse
                (-direction.0, -direction.1)
            }
        }
    } else {
        infected.insert(position.to_owned(), Node::Weakened);
        // left
        (direction.1, -direction.0)
    };
    *direction = next_direction;
    *position = (position.0 + direction.0, position.1 + direction.1);

    infection
}

#[derive(Clone, Copy)]
enum Node {
    Weakened,
    Infected,
    Flagged,
}

impl Node {
    fn parse_grid(grid: &HashSet<(i32, i32)>) -> HashMap<(i32, i32), Node> {
        let mut nodes = HashMap::new();
        for e in grid {
            nodes.insert(e.to_owned(), Node::Infected);
        }
        nodes
    }
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

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day22_test_data.txt")?;
        let (infected, position) = parse_grid(&input);

        // Act
        let actual = iterate_complex(&infected, position, 10_000_000);

        // Assert
        assert_eq!(actual, 2_511_944);
        Ok(())
    }
}
