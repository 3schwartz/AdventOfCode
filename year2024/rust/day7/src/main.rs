use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day7_data.txt")?;

    let mut total_calibration_1: u64 = 0;
    let mut total_calibration_2: u64 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(2, parts.len());
        let test_value = parts[0].parse::<u64>()?;
        let numbers = parts[1]
            .split(' ')
            .map(|c| c.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?;

        if check(test_value, &numbers, false) {
            total_calibration_1 += test_value;
        }
        if check(test_value, &numbers, true) {
            total_calibration_2 += test_value;
        }
    }

    println!("Part 1: {}", total_calibration_1);
    println!("Part 2: {}", total_calibration_2);

    Ok(())
}

fn check(test_value: u64, numbers: &Vec<u64>, part_2: bool) -> bool {
    let combinations = if part_2 {
        generate_operator_combinations_e(numbers.len())
    } else {
        generate_operator_combinations(numbers.len())
    };
    for combination in combinations {
        assert_eq!(combination.len() + 1, numbers.len());

        let mut total: u64 = numbers[0];

        for (i, c) in combination.chars().enumerate() {
            if c == '+' {
                total += numbers[i + 1]
            } else if c == '*' {
                total *= numbers[i + 1]
            } else if c == '|' {
                let n = numbers[i + 1];
                let smallest = smallest_power_of_ten(n);
                if let Some(next) = total.checked_mul(smallest) {
                    total = next + n;
                } else {
                    break;
                }
            }
            if total >= test_value {
                break;
            }
        }

        if total == test_value {
            return true;
        }
    }
    false
}

fn smallest_power_of_ten(n: u64) -> u64 {
    let mut power = 1;

    while power < n {
        power *= 10;
    }

    power
}

fn generate_operator_combinations_e(n: usize) -> Vec<String> {
    let n = n - 1; // Number of operator positions (n digits have n-1 operator slots)
    let mut combinations = Vec::new();

    // There are 3^n combinations since we now have three operators
    let total_combinations = 3_usize.pow(n as u32);

    for i in 0..total_combinations {
        let mut operators = String::new();
        let mut value = i;

        for _ in 0..n {
            match value % 3 {
                0 => operators.push('+'),
                1 => operators.push('*'),
                2 => operators.push('|'),
                _ => unreachable!(),
            }
            value /= 3;
        }

        combinations.push(operators);
    }

    combinations
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
        let input = fs::read_to_string("../../data/day7_test_data.txt")?;

        let mut total_calibration: u64 = 0;

        for line in input.lines() {
            let parts: Vec<&str> = line.split(": ").collect();
            assert_eq!(2, parts.len());
            let test_value = parts[0].parse::<u64>()?;
            let numbers = parts[1]
                .split(' ')
                .map(|c| c.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()?;

            if check(test_value, &numbers, true) {
                total_calibration += test_value;
            }
        }
        assert_eq!(total_calibration, 11387);
        Ok(())
    }
}
