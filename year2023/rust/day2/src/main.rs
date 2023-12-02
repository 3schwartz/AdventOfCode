use std::{collections::HashMap, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day2_data.txt")?;

    let (part_1, part_2) = solutions(&input)?;

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn solutions(input: &str) -> Result<(usize, u32)> {
    let mut part_1 = 0;
    let mut part_2 = 0;
    for (idx, line) in input.lines().enumerate() {
        if evaluate_game(line)? {
            part_1 += idx + 1;
        }
        part_2 += find_game_power(line)?;
    }
    Ok((part_1, part_2))
}

fn find_game_power(game: &str) -> Result<u32> {
    let game_parts: Vec<&str> = game.split(": ").collect();
    let sets: Vec<&str> = game_parts[1].split("; ").collect();

    let mut powers = HashMap::<&str, u32>::new();
    for set in sets {
        let cubes: Vec<&str> = set.split(", ").collect();
        for cube in cubes {
            let cube_parts: Vec<&str> = cube.split_whitespace().collect();
            let count = cube_parts[0].parse::<u32>()?;
            let color = cube_parts[1];

            let entry = powers.entry(color).or_insert(0);
            *entry = std::cmp::max(*entry, count);
        }
    }

    let power = powers.get("red").unwrap_or(&0)
        * powers.get("green").unwrap_or(&0)
        * powers.get("blue").unwrap_or(&0);
    Ok(power)
}

fn evaluate_game(game: &str) -> Result<bool> {
    let game_parts: Vec<&str> = game.split(": ").collect();
    let sets: Vec<&str> = game_parts[1].split("; ").collect();

    for set in sets {
        let cubes: Vec<&str> = set.split(", ").collect();
        for cube in cubes {
            let cube_parts: Vec<&str> = cube.split_whitespace().collect();
            let count = cube_parts[0].parse::<u32>()?;
            let color = cube_parts[1];
            if color == "red" && count > 12 {
                return Ok(false);
            }
            if color == "green" && count > 13 {
                return Ok(false);
            }
            if color == "blue" && count > 14 {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day2_data_test.txt")?;

        // Act
        let (part_1, _) = solutions(&input)?;

        // Assert
        assert_eq!(8, part_1);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day2_data_test.txt")?;

        // Act
        let (_, part_2) = solutions(&input)?;

        // Assert
        assert_eq!(2286, part_2);
        Ok(())
    }
}
