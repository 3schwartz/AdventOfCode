use std::fs;

use anyhow::Result;

const A_G: u64 = 16_807;
const B_G: u64 = 48_271;

const A_M: u64 = 4;
const B_M: u64 = 8;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day15_data.txt")?;

    let (a, b) = parse(&input);

    let count = find_pair_count(a, b, 40_000_000, 1, 1, false);

    println!("Part 1: {count}");

    let count = find_pair_count(a, b, 5_000_000, A_M, B_M, false);

    println!("Part 2: {count}");
    Ok(())
}

fn parse(input: &str) -> (u64, u64) {
    let parts = input.trim().split("\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let first = &parts[0][24..];
    let second = &parts[1][24..];

    (first.parse().unwrap(), second.parse().unwrap())
}

fn next(previous: u64, generator: u64) -> u64 {
    (previous * generator) % 214_748_3647
}

fn last_16_bits(number: u64) -> u64 {
    number & 0xFFFF
}

fn find_pair_count(
    mut a: u64,
    mut b: u64,
    iterations: usize,
    a_mut: u64,
    b_mut: u64,
    debug: bool,
) -> u64 {
    let mut count = 0;
    for i in 0..iterations {
        if i % 100_000 == 0 && debug {
            println!("{i}");
        }
        loop {
            a = next(a, A_G);
            if a % a_mut == 0 {
                break;
            }
        }
        loop {
            b = next(b, B_G);
            if b % b_mut == 0 {
                break;
            }
        }

        if last_16_bits(a) == last_16_bits(b) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        let (a, b) = (65, 8921);

        // Act
        let count = find_pair_count(a, b, 40_000_000, 1, 1, true);

        // Assert
        assert_eq!(count, 588)
    }

    #[test]
    fn test_part_2() {
        // Arrange
        let (a, b) = (65, 8921);

        // Act
        let count = find_pair_count(a, b, 5_000_000, A_M, B_M, true);

        // Assert
        assert_eq!(count, 309)
    }
}
