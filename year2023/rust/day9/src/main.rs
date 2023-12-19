use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day9_data.txt")?;

    let part_1 = get_nexts(&input, false)?;
    println!("Part 1: {}", part_1);

    let part_2 = get_nexts(&input, true)?;
    println!("Part 2: {}", part_2);
    Ok(())
}

fn get_nexts(input: &str, reverse: bool) -> Result<i32> {
    let mut total = 0;
    for line in input.lines() {
        let mut sequence: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        if reverse {
            sequence.reverse();
        }
        let next = get_next(&sequence);
        total += next;
    }
    Ok(total)
}

fn get_next(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|n| *n == 0) {
        return 0;
    }
    let diffs = get_diffs(sequence);
    let next = get_next(&diffs);
    sequence.last().unwrap() + next
}

fn get_diffs(sequence: &Vec<i32>) -> Vec<i32> {
    let mut out = Vec::with_capacity(sequence.len() - 1);
    for i in 0..sequence.len() - 1 {
        out.push(sequence[i + 1] - sequence[i])
    }
    out
}

#[cfg(test)]
mod test {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day9_data_test.txt")?;

        // Act
        let actual = get_nexts(&input, true)?;

        // Assert
        assert_eq!(2, actual);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day9_data_test.txt")?;

        // Act
        let actual = get_nexts(&input, false)?;

        // Assert
        assert_eq!(114, actual);
        Ok(())
    }
}
