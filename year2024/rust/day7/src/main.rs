use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day7_data.txt")?;

    let mut total_calibration_1: u128 = 0;
    let mut total_calibration_2: u128 = 0;
    let mut total_calibration_2_v2: u128 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(2, parts.len());
        let test_value = parts[0].parse::<u128>()?;
        let numbers = parts[1]
            .split(' ')
            .map(|c| c.parse::<u128>())
            .collect::<Result<Vec<u128>, _>>()?;

        if correct(test_value, &numbers, false) {
            total_calibration_1 += test_value;
        }
        if check(test_value, &numbers, true) {
            total_calibration_2 += test_value;
        }
        if correct(test_value, &numbers, true) {
            total_calibration_2_v2 += test_value;
        }
    }

    println!("Part 1: {}", total_calibration_1);
    println!("Part 2: {}", total_calibration_2);
    println!("Part 2: {}", total_calibration_2_v2);

    Ok(())
}

fn correct(test_value: u128, numbers: &[u128], part_2: bool) -> bool {
    if numbers.len() == 1 {
        return test_value == numbers[0];
    };
    if numbers[0] > test_value {
        return false;
    }

    let mut sum = Vec::from([numbers[0] + numbers[1]]);
    sum.extend_from_slice(&numbers[2..]);
    if correct(test_value, &sum, part_2) {
        return true;
    };

    let mut prod = Vec::from([numbers[0] * numbers[1]]);
    prod.extend_from_slice(&numbers[2..]);
    if correct(test_value, &prod, part_2) {
        return true;
    };

    if part_2 {
        let mut concat = Vec::from([format!("{}{}", numbers[0], numbers[1]).parse().unwrap()]);
        concat.extend_from_slice(&numbers[2..]);
        if correct(test_value, &concat, part_2) {
            return true;
        };
    }

    false
}

fn check(test_value: u128, numbers: &[u128], part_2: bool) -> bool {
    let combinations = if part_2 {
        generate_operator_combinations_e(numbers.len())
    } else {
        generate_operator_combinations(numbers.len())
    };
    for combination in combinations {
        assert_eq!(combination.len() + 1, numbers.len());

        let mut total: u128 = numbers[0];
        for (i, c) in combination.chars().enumerate() {
            if c == '+' {
                total += numbers[i + 1]
            } else if c == '*' {
                total *= numbers[i + 1]
            } else if c == '|' {
                let n = numbers[i + 1];
                let f = format!("{}{}", total, n).parse().unwrap();
                total = f;
            }
            if total > test_value {
                break;
            }
        }

        if total == test_value {
            return true;
        }
    }
    false
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
    use anyhow::Ok;

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

        let mut total_calibration: u128 = 0;

        for line in input.lines() {
            let parts: Vec<&str> = line.split(": ").collect();
            assert_eq!(2, parts.len());
            let test_value = parts[0].parse::<u128>()?;
            let numbers = parts[1]
                .split(' ')
                .map(|c| c.parse::<u128>())
                .collect::<Result<Vec<u128>, _>>()?;

            if check(test_value, &numbers, true) {
                total_calibration += test_value;
            }
        }
        assert_eq!(total_calibration, 11387);
        Ok(())
    }
}
