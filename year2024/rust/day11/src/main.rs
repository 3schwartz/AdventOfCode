use anyhow::{anyhow, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day11_data.txt")?;

    let stones = stones(&input)?;

    let count = stone_count(&stones, 25);

    println!("Part 1: {}", count);

    let count = stone_count(&stones, 75);

    println!("Part 2: {}", count);

    Ok(())
}

fn stones(input: &str) -> Result<Vec<u64>> {
    input
        .split_whitespace()
        .map(|c| c.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|e| anyhow!(e))
}

fn stone_count(stones: &[u64], count: u64) -> usize {
    let mut stones = stones.to_vec();

    for _ in 0..count {
        let mut s = 0;
        loop {
            if s > stones.len() - 1 {
                break;
            }
            let stone = stones[s];
            if stone == 0 {
                stones[s] = 1;
            } else if has_even(stone) {
                let new_stones = split_into_parts(stone);
                stones[s] = new_stones[0];
                stones.insert(s + 1, new_stones[1]);
                s += 1;
            } else {
                stones[s] = stone * 2024
            }
            s += 1;
        }
    }
    stones.len()
}

fn has_even(mut input: u64) -> bool {
    let mut n = 0;

    loop {
        n += 1;
        if input < 10 {
            break;
        }
        input /= 10;
    }
    n % 2 == 0
}

fn split_into_parts(input: u64) -> [u64; 2] {
    let parts = format!("{}", input);
    let len = parts.len() / 2;
    [parts[..len].parse().unwrap(), parts[len..].parse().unwrap()]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_into_parts() {
        // Arrange
        let input = vec![(12, 1, 2), (257524, 257, 524)];

        // Act & Assert
        for i in input {
            let parts = split_into_parts(i.0);
            assert_eq!(parts[0], i.1);
            assert_eq!(parts[1], i.2);
        }
    }

    #[test]
    fn test_has_even() {
        // Arrange
        let input = vec![
            (123, false),
            (32, true),
            (0, false),
            (1, false),
            (3123, true),
        ];

        // Act & Assert
        for i in input {
            let actual = has_even(i.0);
            assert_eq!(actual, i.1);
        }
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day11_test_data.txt")?;

        // Act
        let stones = stones(&input)?;
        let stone_count = stone_count(&stones, 25);

        // Assert
        assert_eq!(stone_count, 55312);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }
}
