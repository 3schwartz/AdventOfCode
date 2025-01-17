use std::{collections::HashMap, fs};

use anyhow::{anyhow, Result};

const NEIGHBORS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day19_data.txt")?;

    let grid = parse(&input);
    let (start, direction) = find_start(&grid)?;

    let (word, count) = go_through(start, direction, &grid)?;

    println!("Part 1: {word}");
    println!("Part 2: {count}");
    Ok(())
}

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            grid.insert((x as i32, y as i32), c);
        }
    }
    grid
}

fn go_through(
    start: (i32, i32),
    direction: (i32, i32),
    grid: &HashMap<(i32, i32), char>,
) -> Result<(String, u32)> {
    let mut queue = Vec::from([(start, direction, 1)]);
    let mut word = Vec::new();
    let mut total_count = 0;

    while let Some((position, direction, mut count)) = queue.pop() {
        let position_char = if let Some(next_char) = grid.get(&position) {
            *next_char
        } else {
            continue;
        };
        if position_char == '+' {
            for n in NEIGHBORS {
                if n == (-direction.0, -direction.1) {
                    continue;
                }
                let next = (position.0 + n.0, position.1 + n.1);
                queue.push((next, n, count + 1));
            }
            continue;
        }
        if position_char.is_alphabetic() {
            total_count += count;
            count = 0;
            word.push(position_char);
        }
        let next = (position.0 + direction.0, position.1 + direction.1);
        queue.push((next, direction, count + 1));
    }

    Ok((word.iter().collect(), total_count))
}

fn find_start(grid: &HashMap<(i32, i32), char>) -> Result<((i32, i32), (i32, i32))> {
    let mut start = None;
    for (coord, c) in grid {
        if c.is_alphabetic() {
            continue;
        }
        let mut n_count = 0;
        for n in NEIGHBORS {
            let next = (coord.0 + n.0, coord.1 + n.1);
            if grid.contains_key(&next) {
                n_count += 1;
            }
        }
        if n_count == 1 {
            start = Some(*coord);
        }
    }
    let s = if let Some(s) = start {
        s
    } else {
        return Err(anyhow!("couldn't find start"));
    };

    let d = grid.get(&s).ok_or_else(|| anyhow!("just found start"))?;

    let direction = match (s, *d) {
        ((0, _), '-') => (1, 0),
        ((_, 0), '|') => (0, 1),
        _ => {
            return Err(anyhow!(
                "not able to find direction from start {:?} and direction {}",
                s,
                d
            ))
        }
    };
    Ok((s, direction))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_and_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day19_test_data.txt")?;

        // Act
        let grid = parse(&input);
        let (start, direction) = find_start(&grid)?;

        let (word, count) = go_through(start, direction, &grid)?;

        // Assert
        assert_eq!("ABCDEF", word);
        assert_eq!(38, count);
        Ok(())
    }
}
