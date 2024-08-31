use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day1_data.txt")?;
    let numbers: Vec<u32> = input.chars().map(|c| (c as u32 - '0' as u32)).collect();

    let part_1 = next_sum(&numbers);
    println!("Part 1: {}", part_1);

    let part_2 = next_modulo(&numbers);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn next_modulo(numbers: &[u32]) -> u32 {
    let len = numbers.len();
    let step = len / 2;
    let mut sum = 0;
    for (idx, number) in numbers.iter().enumerate() {
        let other = numbers[(idx + step) % len];
        if *number == other {
            sum += other;
        }
    }
    sum
}

fn next_sum(numbers: &Vec<u32>) -> u32 {
    let mut before = numbers.last().expect("should not be empty");
    let mut sum = 0;
    for number in numbers {
        if number == before {
            sum += before;
        }
        before = number;
    }
    sum
}
