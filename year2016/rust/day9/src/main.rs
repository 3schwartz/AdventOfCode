use std::fs;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day9_data.txt")?;

    let decompressed = decompress(&input.trim())?;

    println!("Part 1: {}", decompressed);

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

#[cfg(test)]
mod test {
    use super::*;

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

// Wandering around a secure area, you come across a datalink port to a new part of the network.
// After briefly scanning it for interesting files, you find one file in particular that catches your attention.
// It's compressed with an experimental format, but fortunately, the documentation for the format is nearby.

// The format compresses a sequence of characters. Whitespace is ignored.
// To indicate that some sequence should be repeated, a marker is added to the file, like (10x2).
// To decompress this marker, take the subsequent 10 characters and repeat them 2 times.
//  Then, continue reading the file after the repeated data. The marker itself is not included in the decompressed output.

// If parentheses or other characters appear within the data referenced by a marker, that's okay - treat it like normal data,
//  not a marker, and then resume looking for markers after the decompressed section.

// For example:

// ADVENT contains no markers and decompresses to itself with no changes, resulting in a decompressed length of 6.
// A(1x5)BC repeats only the B a total of 5 times, becoming ABBBBBC for a decompressed length of 7.
// (3x3)XYZ becomes XYZXYZXYZ for a decompressed length of 9.
// A(2x2)BCD(2x2)EFG doubles the BC and EF, becoming ABCBCDEFEFG for a decompressed length of 11.
// (6x1)(1x3)A simply becomes (1x3)A - the (1x3) looks like a marker, but because it's within a data section of another marker, it is not treated any differently from the A that comes after it. It has a decompressed length of 6.
// X(8x2)(3x3)ABCY becomes X(3x3)ABC(3x3)ABCY (for a decompressed length of 18), because the decompressed data from the (8x2) marker (the (3x3)ABC) is skipped and not processed further.
// What is the decompressed length of the file (your puzzle input)? Don't count whitespace.
