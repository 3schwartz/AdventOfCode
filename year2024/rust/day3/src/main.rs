use anyhow::Result;
use regex::Regex;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day3_data.txt")?;

    let mut whole_string = vec![];
    for line in input.lines() {
        whole_string.push(line);
    }
    let whole = whole_string.concat();

    println!("Part 1: {}", get_count(&whole)?);

    let do_dont_re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let do_dont_vec = do_dont_re
        .find_iter(&whole)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    let parts = whole
        .split("do()")
        .flat_map(|c| c.split("don't()"))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    assert_eq!(parts.len(), do_dont_vec.len() + 1);

    let mut total = get_count(&parts[0])?;
    for i in 1..parts.len() {
        if do_dont_vec[i - 1] != "do()" {
            continue;
        }
        total += get_count(&parts[i])?;
    }

    println!("Part 2: {}", total);

    Ok(())
}

fn get_count(whole: &str) -> Result<i32> {
    let mut total = 0;
    let re = Regex::new(r"mul\((\d{1,100}),(\d{1,100})\)")?;
    for cap in re.captures_iter(whole) {
        println!("Found match: {}", &cap[0]);
        let first = cap[1].parse::<i32>()?;
        let second = cap[2].parse::<i32>()?;
        total += first * second;
    }
    Ok(total)
}
