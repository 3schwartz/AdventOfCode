use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day5_data.txt")?;

    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules = BeforeAfter::rules(parts[0])?;

    let rules_result = RulesResult::apply_rules(&rules, parts[1])?;
    let total_center = rules_result.center_correct();

    println!("Part 1: {}", total_center);

    let total_center = rules_result.center_wrong(&rules);
    println!("Part 2: {}", total_center);

    Ok(())
}

struct RulesResult {
    correct: Vec<Vec<i32>>,
    wrong: Vec<Vec<i32>>,
}

impl RulesResult {
    fn center_correct(&self) -> i32 {
        let mut total_center = 0;
        for pages in &self.correct {
            total_center += pages[pages.len() / 2];
        }
        total_center
    }

    fn center_wrong(self, rules: &HashMap<i32, BeforeAfter>) -> i32 {
        let mut total_center = 0;

        for wrong in self.wrong {
            let mut last: Vec<_> = wrong.to_vec();

            let mut is_match = false;
            while !is_match {
                let mut updated = vec![];

                for (i, page) in last.iter().enumerate() {
                    if i == last.len() - 1 {
                        updated.push(*page);
                        continue;
                    }

                    if let Some(before_after) = rules.get(page) {
                        let mut to_append = vec![];
                        for last_item in last.iter().skip(i + 1) {
                            if before_after.before.contains(last_item) {
                                updated.push(*last_item);
                            } else {
                                to_append.push(*last_item);
                            }
                        }
                        if updated.len() != i {
                            updated.push(*page);
                            updated.append(&mut to_append);
                        }
                    }
                    if updated.len() == last.len() {
                        break;
                    } else {
                        updated.push(*page);
                    }
                }
                if updated.eq(&last) {
                    is_match = true
                }
                last = updated;
            }
            total_center += last[last.len() / 2]
        }

        total_center
    }

    fn apply_rules(rules: &HashMap<i32, BeforeAfter>, updates: &str) -> Result<Self> {
        let mut correct = vec![];
        let mut wrong = vec![];

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
                    for page_next in pages.iter().skip(i + 1) {
                        if before_after.before.contains(page_next) {
                            is_valid = false;
                            break;
                        }
                    }
                }
                if !is_valid {
                    break;
                }
            }
            if is_valid {
                correct.push(pages);
            } else {
                wrong.push(pages);
            }
        }
        Ok(Self { correct, wrong })
    }
}

#[derive(Default)]
struct BeforeAfter {
    before: HashSet<i32>,
    after: HashSet<i32>,
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day5_test_data.txt")?;
        let parts: Vec<&str> = input.split("\n\n").collect();

        // Act
        let rules = BeforeAfter::rules(parts[0])?;
        let rules_result = RulesResult::apply_rules(&rules, parts[1])?;
        let total_center = rules_result.center_correct();

        // Assert
        assert_eq!(total_center, 143);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day5_test_data.txt")?;
        let parts: Vec<&str> = input.split("\n\n").collect();

        // Act
        let rules = BeforeAfter::rules(parts[0])?;
        let rules_result = RulesResult::apply_rules(&rules, parts[1])?;
        let total_center = rules_result.center_wrong(&rules);

        // Assert
        assert_eq!(total_center, 123);
        Ok(())
    }
}
