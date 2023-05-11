use std::{fs, collections::HashSet};

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

    let mut sue = HashSet::new();
    sue.insert(("children", 3));
    sue.insert(("cats", 7));
    sue.insert(("samoyeds", 2));
    sue.insert(("pomeranians", 3));
    sue.insert(("akitas", 0));
    sue.insert(("vizslas", 0));
    sue.insert(("goldfish", 5));
    sue.insert(("trees", 3));
    sue.insert(("cars", 2));
    sue.insert(("perfumes", 1));

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

        let is_sue = parts.iter()
            .filter_map(|c| get_type_count(c).ok())
            .all(|c| sue.contains(&c));

        if is_sue {
            println!("{}", line);
        }
    }


    Ok(())
}
