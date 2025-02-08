use std::fs;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day9_data.txt")?;

    let decompressed = decompress(input.trim())?;

    println!("Part 1: {}", decompressed);

    let decompressed_recursive = decompress_recursive(input.trim())?;

    println!("Part 2: {}", decompressed_recursive);

    Ok(())
}

fn decompress(input: &str) -> Result<usize> {
    let mut cursor = 0;
    let mut total = 0;
    while cursor < input.len() {
        let s = &input[cursor..cursor + 1];
        if s != "(" {
            cursor += 1;
            total += 1;
            continue;
        }
        let mut end = cursor;
        loop {
            end += 1;
            let e = &input[end..end + 1];
            if e != ")" {
                continue;
            }
            break;
        }
        let parts = input[cursor + 1..end]
            .split('x')
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()?;
        if parts.len() != 2 {
            return Err(anyhow!("wrong length: {:?}", parts));
        }
        total += parts[0] * parts[1];
        cursor = end + 1 + parts[0];
    }
    Ok(total)
}

fn decompress_recursive(input: &str) -> Result<usize> {
    let mut cursor = 0;
    let mut total = 0;
    while cursor < input.len() {
        let s = &input[cursor..cursor + 1];
        if s != "(" {
            cursor += 1;
            total += 1;
            continue;
        }
        let mut end = cursor;
        loop {
            end += 1;
            let e = &input[end..end + 1];
            if e != ")" {
                continue;
            }
            break;
        }
        let parts = input[cursor + 1..end]
            .split('x')
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()?;
        if parts.len() != 2 {
            return Err(anyhow!("wrong length: {:?}", parts));
        }
        let nested_code = &input[end + 1..end + 1 + parts[0]];
        let nested_length = decompress_recursive(nested_code)?;
        total += nested_length * parts[1];
        cursor = end + 1 + parts[0];
    }
    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = vec![
            ("ADVENT", 6),
            (
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                445,
            ),
            ("(3x3)XYZ", 9),
            ("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241_920),
            ("X(8x2)(3x3)ABCY", "XABCABCABCABCABCABCY".len()),
        ];

        for (code, expected) in input {
            // Act
            let actual = decompress_recursive(code)?;

            // Assert
            assert_eq!(actual, expected);
        }

        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = vec![
            ("A(2x2)BCD(2x2)EFG", 11),
            ("ADVENT", 6),
            ("A(1x5)BC", 7),
            ("(3x3)XYZ", 9),
            ("(6x1)(1x3)A", 6),
            ("X(8x2)(3x3)ABCY", 18),
        ];

        for (code, expected) in input {
            // Act
            let actual = decompress(code)?;

            // Assert
            assert_eq!(actual, expected);
        }

        Ok(())
    }
}
