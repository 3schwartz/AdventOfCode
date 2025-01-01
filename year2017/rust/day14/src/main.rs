use anyhow::{anyhow, Result};
use day10::make_knot_hash;
use std::fmt::Write;

fn main() -> Result<()> {
    let input = "jzgqcdpd";

    let used = count_used(input)?;

    println!("Part 1: {used}");
    Ok(())
}

fn binary_used_count(binary_string: &str) -> usize {
    binary_string.chars().filter(|c| *c == '1').count()
}

fn hex_to_binary(hex_string: &str) -> Result<String> {
    let mut binary_string = String::new();
    for c in hex_string.chars() {
        let digit = c
            .to_digit(16)
            .ok_or_else(|| anyhow!("not able to make {c} to base 16 digit."))?;
        let bit_format = format!("{:04b}", digit);
        write!(&mut binary_string, "{bit_format}")?;
    }
    Ok(binary_string)
}

fn count_used(input: &str) -> Result<usize> {
    let mut used = 0;
    for r in 0..128 {
        let i = format!("{input}-{r}");
        let hash = make_knot_hash(&i);
        let binary = hex_to_binary(&hash)?;
        used += binary_used_count(&binary);
    }
    Ok(used)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_to_binary() -> Result<()> {
        // Arrange
        let hex = "a0c2017";

        // Act
        let binary = hex_to_binary(&hex)?;

        // Assert
        assert_eq!(binary, "1010000011000010000000010111");
        Ok(())
    }

    #[test]
    fn test_knot_hashes() {
        // Arrange
        let input = vec!["flqrgnkx-0"];

        // Act
        for i in input {
            let hash = make_knot_hash(i);

            // Assert
            assert_eq!(hash.len(), 32);
        }
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = "flqrgnkx";

        // Act
        let used = count_used(input)?;

        // Assert
        assert_eq!(8_108, used);
        Ok(())
    }
}
