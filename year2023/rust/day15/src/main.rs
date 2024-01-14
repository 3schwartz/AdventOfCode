use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day15_data.txt").map(|l| l.trim_end().to_string())?;

    let mut part_1 = 0;
    for seq in input.split(',') {
        part_1 += hash(seq);
    }
    println!("Part 1: {}", part_1);

    let boxes = hashmap(&input)?;
    let part_2 = hashmap_score(boxes);
    println!("Part 2: {}", part_2);
    Ok(())
}

fn hashmap_score(hash_map: Vec<Vec<(String, u32)>>) -> u32 {
    let mut score = 0;
    for (b, boxx) in hash_map.iter().enumerate() {
        for (s, sloth) in boxx.iter().enumerate() {
            score += ((b + 1) * (s + 1)) as u32 * sloth.1;
        }
    }
    score
}

fn hashmap(input: &str) -> Result<Vec<Vec<(String, u32)>>> {
    let commands: Vec<&str> = input.split(',').collect();
    let mut boxes: Vec<Vec<(String, u32)>> = vec![Default::default(); 256];

    for command in commands {
        let rev: String = command.chars().rev().collect();
        if &rev[0..1] == "-" {
            let label: String = rev[1..].chars().rev().collect();
            let h = hash(&label);
            boxes[h as usize].retain(|(lab, _)| *lab != label);
        } else if &rev[1..2] == "=" {
            let label: String = rev[2..].chars().rev().collect();
            let focal_length: u32 = rev[..1].parse()?;
            let h = hash(&label);
            let boxx = &mut boxes[h as usize];
            if boxx.iter().any(|(lab, _)| *lab == label) {
                boxes[h as usize] = boxx
                    .iter()
                    .map(|(lab, len)| {
                        if *lab == label {
                            (label.clone(), focal_length)
                        } else {
                            (lab.to_string(), *len)
                        }
                    })
                    .collect();
            } else {
                boxx.push((label, focal_length));
            }
        }
    }
    Ok(boxes)
}

fn hash(input: &str) -> u32 {
    let mut h = 0;
    for c in input.chars() {
        h = ((h + c as u32) * 17) % 256
    }
    h
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let input = fs::read_to_string("../../data/day15_data_test.txt")?;

        let mut part_1 = 0;
        for seq in input.trim_end().split(',') {
            let h = hash(seq);
            part_1 += h;
        }
        assert_eq!(1_320, part_1);
        Ok(())
    }
}
