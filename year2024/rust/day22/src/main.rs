use anyhow::{anyhow, Ok, Result};
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day22_data.txt")?;

    let secret_sum = find_secret_sum(&input)?;

    println!("Part 1: {secret_sum}");

    let bananas = find_bananas(&input)?;

    println!("Part 2: {bananas}");

    Ok(())
}

fn find_secret_sum(input: &str) -> Result<u64> {
    let mut secret_sum = 0;
    for line in input.lines() {
        let secrets = secrets(line)?;
        secret_sum += secrets[secrets.len() - 1];
    }
    Ok(secret_sum)
}

fn find_bananas(input: &str) -> Result<i64> {
    let mut sequence_prices = HashMap::new();
    for line in input.lines() {
        let secrets = secrets(line)?;
        let prices = prices(&secrets);
        let changes = changes(&prices);
        let sequences = sequences(&changes, &prices);
        for (sequence, price) in sequences {
            sequence_prices
                .entry(sequence)
                .and_modify(|p| *p += price)
                .or_insert(price);
        }
    }
    sequence_prices
        .values()
        .max()
        .copied()
        .ok_or_else(|| anyhow!("expected max to be found"))
}

fn sequences(changes: &[i64], prices: &[i64]) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut sequences = HashMap::new();
    for i in 3..changes.len() {
        let change = (changes[i - 3], changes[i - 2], changes[i - 1], changes[i]);
        if sequences.contains_key(&change) {
            continue;
        }
        let price = prices[i + 1];
        sequences.insert(change, price);
    }
    sequences
}

fn changes(prices: &[i64]) -> Vec<i64> {
    let mut changes = vec![];
    for i in 1..prices.len() {
        changes.push(prices[i] - prices[i - 1]);
    }
    changes
}

fn prices(secrets: &[u64]) -> Vec<i64> {
    secrets.iter().map(|s| s % 10).map(|p| p as i64).collect()
}

fn secrets(line: &str) -> Result<Vec<u64>> {
    let mut secrets = vec![];
    let mut secret = line.parse::<u64>()?;
    for _ in 0..2_000 {
        let result = secret * 64;
        secret = mix(secret, result);
        secret = prune(secret);

        let result = secret / 32;
        secret = mix(secret, result);
        secret = prune(secret);

        let result = secret * 2_048;
        secret = mix(secret, result);
        secret = prune(secret);
        secrets.push(secret);
    }
    Ok(secrets)
}

fn mix(secret: u64, result: u64) -> u64 {
    secret ^ result
}

fn prune(secret: u64) -> u64 {
    secret % 16_777_216
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day22_test_data.txt")?;

        // Act
        let secret_sum = find_secret_sum(&input)?;

        // Assert
        assert_eq!(secret_sum, 37_327_623);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day22_test2_data.txt")?;

        // Act
        let bananas = find_bananas(&input)?;

        // Assert
        assert_eq!(bananas, 23);
        Ok(())
    }
}
