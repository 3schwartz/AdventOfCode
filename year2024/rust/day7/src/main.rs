use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day7_data.txt")?;

    let mut total_calibration: u64 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(2, parts.len());
        let test_value = parts[0].parse::<u64>()?;
        let numbers = parts[1]
            .split(' ')
            .map(|c| c.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?;

        for combination in generate_operator_combinations(numbers.len()) {
            assert_eq!(combination.len() + 1, numbers.len());

            let mut total: u64 = numbers[0];

            for (i, c) in combination.chars().enumerate() {
                if c == '+' {
                    total += numbers[i + 1]
                } else if c == '*' {
                    total *= numbers[i + 1]
                }
                if total > test_value {
                    break;
                }
            }

            if total == test_value {
                total_calibration += test_value;
                break;
            }
        }
    }

    println!("Part 1: {}", total_calibration);

    Ok(())
}

fn generate_operator_combinations(n: usize) -> Vec<String> {
    let n = n - 1;
    let mut combinations = Vec::new();

    // Generate all 2^n combinations of operators
    for i in 0..(1 << n) {
        let mut operators = String::new();

        for j in 0..n {
            if (i & (1 << j)) == 0 {
                operators.push('+');
            } else {
                operators.push('*');
            }
        }

        combinations.push(operators);
    }

    combinations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        // Act
        // Assert
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
