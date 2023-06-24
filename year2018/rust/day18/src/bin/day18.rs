use day18;
use std::fs;
use anyhow::{Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day18_data.txt")?;

    let map = day18::LumberCollection::from(&input)?;

    let part_1 = map.find_resource_after_iterations(10, 0); // 50 to debug
    let part_1_simple = map.find_resource_after_iterations_using_sumple(10);

    println!("Part 1: {}", part_1);
    println!("Part 1 simple: {}", part_1_simple);

    let part_2 = map.find_resources_after_one_billion();

    println!("Part 2: {}", part_2);

    Ok(())
}