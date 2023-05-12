use std::{fs, collections::HashMap};

use anyhow::{Result, anyhow};

fn get_type_count(part: &str) -> Result<(&str, i32)> {
    let parts: Vec<&str> = part.split(":")
        .collect();
    let p = *parts.get(0).ok_or(anyhow!("{}", part))?;
    let c: i32 = parts.get(1).ok_or(anyhow!("{}", part))?
        .parse()?;

    Ok((p,c))
}


fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day16_data.txt")?;

    let mut sue = HashMap::new();
    sue.insert("children", 3);
    sue.insert("cats", 7);
    sue.insert("samoyeds", 2);
    sue.insert("pomeranians", 3);
    sue.insert("akitas", 0);
    sue.insert("vizslas", 0);
    sue.insert("goldfish", 5);
    sue.insert("trees", 3);
    sue.insert("cars", 2);
    sue.insert("perfumes", 1);

    for line in input.lines() {
        let temp: String = line
            .split(' ')
            .skip(2)
            .collect::<Vec<&str>>()
            .concat();
        
        let parts: Vec<&str> = temp
            .split(",")
            .map(|s| s.trim_end_matches(","))
            .collect();

        let mut part_1 = true;
        let mut part_2 = true;
        for part in parts {
            let (sue_type, sue_count) = get_type_count(part)?;
            let count = *sue.get(sue_type).ok_or(anyhow!("missing"))?;

            part_1 &= count == sue_count;

            part_2 &= match sue_type {
                "cats" | "trees" => sue_count > count,
                "pomeranians" | "goldfish" => sue_count < count,
                _ => count == sue_count
            };
        }
        
        if part_1 {
            println!("Part 1: {}", line);
        }
        if part_2 {
            println!("Part 2: {}", line);
        }
    }


    Ok(())
}
