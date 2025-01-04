use std::fs;

use anyhow::Result;

const A_G: u64 = 16_807;
const B_G: u64 = 48_271;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day15_data.txt")?;

    let (a, b) = parse(&input);

    let count = find_pair_count(a, b, 40_000_000, false);

    println!("{count}");
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

fn find_pair_count(mut a: u64, mut b: u64, iterations: usize, debug: bool) -> u64 {
    let mut count = 0;
    for i in 0..iterations {
        if i % 100_000 == 0 && debug {
            println!("{i}");
        }
        a = next(a, A_G);
        b = next(b, B_G);
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
        let count = find_pair_count(a, b, 40_000_000, true);

        // Assert
        assert_eq!(count, 588)
    }
}
