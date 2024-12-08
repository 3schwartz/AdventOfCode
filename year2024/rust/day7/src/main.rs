use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day7_data.txt")?;

    let mut total_calibration_1: u128 = 0;
    let mut total_calibration_2: u128 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(2, parts.len());
        let test_value = parts[0].parse::<u128>()?;
        let numbers = parts[1]
            .split(' ')
            .map(|c| c.parse::<u128>())
            .collect::<Result<Vec<u128>, _>>()?;

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

fn check(test_value: u128, numbers: &[u128], part_2: bool) -> bool {
    if numbers.len() == 1 {
        return test_value == numbers[0];
    };
    if numbers[0] > test_value {
        return false;
    }

    let mut sum = Vec::from([numbers[0] + numbers[1]]);
    sum.extend_from_slice(&numbers[2..]);
    if check(test_value, &sum, part_2) {
        return true;
    };

    let mut prod = Vec::from([numbers[0] * numbers[1]]);
    prod.extend_from_slice(&numbers[2..]);
    if check(test_value, &prod, part_2) {
        return true;
    };

    if part_2 {
        let mut concat = Vec::from([format!("{}{}", numbers[0], numbers[1]).parse().unwrap()]);
        concat.extend_from_slice(&numbers[2..]);
        if check(test_value, &concat, part_2) {
            return true;
        };
    }

    false
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
