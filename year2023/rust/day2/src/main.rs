use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day2_data.txt")?;

    let part_1 = part_1(&input)?;

    println!("Part 1: {}", part_1);

    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let mut part_1 = 0;
    for (idx, line) in input.lines().enumerate() {
        if evaluate_game(line)? {
            part_1 += idx + 1;
        }
    }
    Ok(part_1)
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
        let part_1 = part_1(&input)?;

        // Assert
        assert_eq!(8, part_1);
        Ok(())
    }
}
