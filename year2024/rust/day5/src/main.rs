use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day5_data.txt")?;

    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules = BeforeAfter::rules(parts[0])?;

    let total_center = BeforeAfter::apply_rules(rules, parts[1])?;

    println!("Part 1: {}", total_center);

    Ok(())
}

struct BeforeAfter {
    before: HashSet<i32>,
    after: HashSet<i32>,
}

impl Default for BeforeAfter {
    fn default() -> Self {
        Self {
            before: Default::default(),
            after: Default::default(),
        }
    }
}

impl BeforeAfter {
    fn rules(input: &str) -> Result<HashMap<i32, BeforeAfter>> {
        let mut rules = HashMap::new();
        for line in input.lines() {
            let parts = line
                .split('|')
                .map(|c| c.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?;
            assert_eq!(parts.len(), 2);

            let entry = rules.entry(parts[0]).or_insert(BeforeAfter::default());
            entry.after.insert(parts[1]);

            let entry = rules.entry(parts[1]).or_insert(BeforeAfter::default());
            entry.before.insert(parts[0]);
        }
        Ok(rules)
    }

    fn apply_rules(rules: HashMap<i32, BeforeAfter>, updates: &str) -> Result<i32> {
        let mut total_center = 0;
        for line in updates.lines() {
            let pages = line
                .split(',')
                .map(|c| c.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()?;
            let mut is_valid = true;
            for (i, page) in pages.iter().enumerate() {
                if i == pages.len() - 1 {
                    continue;
                }
                if let Some(before_after) = rules.get(page) {
                    for i_ in i + 1..pages.len() {
                        if before_after.before.contains(&pages[i_]) {
                            is_valid = false;
                            break;
                        }
                    }
                }
                if !is_valid {
                    break;
                }
            }
            if !is_valid {
                continue;
            }
            total_center += pages[pages.len() / 2];
        }
        Ok(total_center)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../data/day5_test_data.txt")?;
        let parts: Vec<&str> = input.split("\n\n").collect();

        // Act
        let rules = BeforeAfter::rules(parts[0])?;
        let total_center = BeforeAfter::apply_rules(rules, parts[1])?;

        // Assert
        assert_eq!(total_center, 143);
        Ok(())
    }
}
