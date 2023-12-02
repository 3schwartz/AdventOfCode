use anyhow::{anyhow, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day1_data.txt")?;

    let part1 = part1(&input)?;

    println!("Part 1: {}", part1);

    let part2 = part2(&input)?;

    println!("Part 2: {}", part2);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let mut total = 0;
    for line in input.lines() {
        let first = get_number(line)?;
        let last = get_number(&line.chars().rev().collect::<String>())?;
        let number = format!("{}{}", first, last);
        total += number.parse::<u32>()?;
    }
    Ok(total)
}

fn get_number(input: &str) -> Result<char> {
    for c in input.chars() {
        if c.is_numeric() {
            return Ok(c);
        }
    }
    Err(anyhow!(input.to_string()))
}

fn part2(input: &str) -> Result<u32> {
    let mut total = 0;
    for line in input.lines() {
        let numbers = get_numbers(line)?;
        let first = numbers
            .first()
            .ok_or_else(|| anyhow!("no first: {}", input.to_string()))?;
        let second = numbers
            .last()
            .ok_or_else(|| anyhow!("no last: {}", input.to_string()))?;
        total += format!("{}{}", first, second).parse::<u32>()?;
    }
    Ok(total)
}

fn get_numbers(input: &str) -> Result<Vec<u32>> {
    let mut numbers = Vec::<u32>::new();
    for i in 0..input.len() {
        if let Ok(number) = input[i..i + 1].parse::<u32>() {
            numbers.push(number);
            continue;
        }
        if i + 3 <= input.len() && &input[i..i + 3] == "one" {
            numbers.push(1);
            continue;
        }
        if i + 3 <= input.len() && &input[i..i + 3] == "two" {
            numbers.push(2);
            continue;
        }
        if i + 5 <= input.len() && &input[i..i + 5] == "three" {
            numbers.push(3);
            continue;
        }
        if i + 4 <= input.len() && &input[i..i + 4] == "four" {
            numbers.push(4);
            continue;
        }
        if i + 4 <= input.len() && &input[i..i + 4] == "five" {
            numbers.push(5);
            continue;
        }
        if i + 3 <= input.len() && &input[i..i + 3] == "six" {
            numbers.push(6);
            continue;
        }
        if i + 5 <= input.len() && &input[i..i + 5] == "seven" {
            numbers.push(7);
            continue;
        }
        if i + 5 <= input.len() && &input[i..i + 5] == "eight" {
            numbers.push(8);
            continue;
        }
        if i + 4 <= input.len() && &input[i..i + 4] == "nine" {
            numbers.push(9);
            continue;
        }
    }
    Ok(numbers)
}
