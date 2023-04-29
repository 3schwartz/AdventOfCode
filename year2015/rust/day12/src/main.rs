use std::fs;
use anyhow::{Result, anyhow};
use serde_json::Value;

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

fn get_sum(v: &Value) -> Result<i64> {
    
    let value = match v {
        Value::Number(n) => n.as_i64()
            .ok_or(anyhow!("issue with number: {}", n))?,
        Value::Array(vv) => vv
            .iter()
            .filter_map(|vvv| get_sum(&vvv).ok())
            .sum(),
        Value::Object(o) => {
            let contains_red = o
            .values()
            .any(|ov| {
                if let Value::String(os) = ov {
                    if os == "red" {
                        return true;
                    }
                }
                return false;
            });
            match contains_red {
                true => 0,
                false => o
                    .values()
                    .filter_map(|ov| get_sum(ov).ok())
                    .sum()
            }
        },
        _ => 0
    };

    Ok(value)
}
