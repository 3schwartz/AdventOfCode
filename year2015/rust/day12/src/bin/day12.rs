use std::fs;
use anyhow::Result;
use serde_json::Value;
use day12::get_sum;

fn main() -> Result<()>{
    let input = fs::read_to_string("../data/day12_data.txt")?;
    let delimiters = [':','{','}', '"', ']', '[',','];
    let part_1 : i32 = input.split(|c| delimiters.contains(&c))
        .filter_map(|c| c.parse::<i32>().ok())
        .sum();

    println!("Part 1: {}", part_1);


    let v: Value = serde_json::from_str(&input)?;
    let part_2 = get_sum(&v)?;

    println!("Part 2: {}", part_2);
    
    Ok(())
}