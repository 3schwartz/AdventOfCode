use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day15_data.txt")?;

    let mut part_1 = 0;
    for seq in input.trim_end().split(',') {
        part_1 += hash(seq);
    }
    println!("Part 1: {}", part_1);
    Ok(())
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
