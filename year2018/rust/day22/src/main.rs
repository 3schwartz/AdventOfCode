use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let (x_target, y_target) = (7, 782);
    let depth = 11820;

    let risk_level = get_risk_level(x_target, y_target, depth)?;

    println!("Part 1: {}", risk_level);

    Ok(())
}

fn get_risk_level(x_target: usize, y_target: usize, depth: usize) -> Result<usize> {
    let mut geologic_index = vec![vec![0;x_target + 1]; y_target + 1];
    let mut erosion_levels = vec![vec![0;x_target + 1]; y_target + 1];

    let mut risk_level = 0;

    for y in 0..=y_target {
        for x in 0..=x_target {
            geologic_index[y][x] = get_geologic_index(x, y, x_target, y_target, &erosion_levels);
            erosion_levels[y][x] = get_erosion_level(x, y, depth, &geologic_index);

            let region_type = Type::from(erosion_levels[y][x])?;

            risk_level += region_type.get_risk_level();
        }
    }

    Ok(risk_level)
}

fn get_erosion_level(x: usize, y: usize, depth: usize, geologic_index: &Vec<Vec<usize>>) -> usize {
    (geologic_index[y][x] + depth) % 20183
}

fn get_geologic_index(x: usize, y: usize, x_target: usize, y_target: usize, erosion_levels: &Vec<Vec<usize>>) -> usize {
    if x == 0 && y == 0 {
        0
    } else if x == x_target && y == y_target {
        0
    } else if y == 0 {
        x * 16807
    } else if x == 0 {
        y * 48271
    } else {
        erosion_levels[y-1][x] * erosion_levels[y][x-1]
    }
}

enum Type {
    Rocky,
    Wet,
    Narrow
}

impl Type {
    fn from(erosion_level: usize) -> Result<Type> {
        let result = match erosion_level % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => return Err(anyhow!("not able to map erosion level to type: {}", erosion_level)),
        };
        Ok(result)
    }

    fn get_risk_level(&self) -> usize {
        match self {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow => 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let (x_target, y_target) = (10, 10);
        let depth = 510;
        let expected_risk_level = 114;

        // Act
        let risk_level = get_risk_level(x_target, y_target, depth)?;

        // Assert
        assert_eq!(risk_level, expected_risk_level);

        Ok(())
    }
}