use anyhow::{anyhow, Result};
use day10::make_knot_hash;
use std::{collections::HashSet, fmt::Write};

fn main() -> Result<()> {
    let input = "jzgqcdpd";

    let used = count_used(input)?;

    println!("Part 1: {used}");

    let used = find_used(input)?;
    let regions = find_regions(&used);

    println!("Part 2: {regions}");
    Ok(())
}

fn binary_used_count(binary_string: &str) -> Vec<usize> {
    let mut cols = vec![];
    for (i, c) in binary_string.chars().enumerate() {
        if c == '1' {
            cols.push(i);
        }
    }
    cols
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

fn find_used(input: &str) -> Result<HashSet<(i32, i32)>> {
    let mut used = HashSet::new();
    for r in 0..128 {
        let i = format!("{input}-{r}");
        let hash = make_knot_hash(&i);
        let binary = hex_to_binary(&hash)?;
        for u in binary_used_count(&binary) {
            used.insert((u as i32, r));
        }
    }
    Ok(used)
}

fn find_regions(used: &HashSet<(i32, i32)>) -> u32 {
    let n = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut seen = HashSet::new();
    let mut regions = 0;
    for next in used {
        let mut queue = vec![*next];
        let mut region = HashSet::new();
        while let Some(q) = queue.pop() {
            if !seen.insert(q) {
                continue;
            }
            region.insert(q);
            for n in &n {
                let neighbor = (q.0 + n.0, q.1 + n.1);
                if used.contains(&neighbor) {
                    queue.push(neighbor);
                }
            }
        }
        if region.is_empty() {
            continue;
        }
        regions += 1;
    }
    regions
}

fn count_used(input: &str) -> Result<usize> {
    let mut used = 0;
    for r in 0..128 {
        let i = format!("{input}-{r}");
        let hash = make_knot_hash(&i);
        let binary = hex_to_binary(&hash)?;
        used += binary_used_count(&binary).len();
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
        let binary = hex_to_binary(hex)?;

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

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = "flqrgnkx";

        // Act
        let used = find_used(input)?;
        let regions = find_regions(&used);

        // Assert
        assert_eq!(1_242, regions);
        Ok(())
    }
}
