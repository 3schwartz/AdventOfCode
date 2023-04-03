use regex::Regex;
use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    let file = fs::read_to_string("../data/day5_data.txt").unwrap();

    let rules_one: Vec<Box<dyn Rule>> = vec![
        Box::new(ThreeVowels::new().unwrap()),
        Box::new(TwiceInRow::new()),
        Box::new(NotContains::new().unwrap()),
    ];

    let machine_one = RuleMachine::new(rules_one);
    let part_one = machine_one.nice_strings(&file);

    println!("Part 1: {part_one}");

    let rules_two: Vec<Box<dyn Rule>> =
        vec![Box::new(OneLetterBetween::new()), Box::new(TimesTwo::new())];

    let machine_two = RuleMachine::new(rules_two);
    let part_two = machine_two.nice_strings(&file);

    println!("Part 2: {part_two}");
}

struct RuleMachine {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleMachine {
    fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    fn nice_strings(&self, file: &str) -> u64 {
        let mut nice_strings = 0;
        for line in file.lines() {
            let mut is_ok = true;
            for rule in &self.rules {
                let rule_ok = rule.is_ok(line);
                if !rule_ok {
                    is_ok = false;
                    break;
                };
            }
            nice_strings += if is_ok { 1 } else { 0 };
        }
        nice_strings
    }
}

trait Rule {
    fn is_ok(&self, rule: &str) -> bool;
}

struct TimesTwo {}

impl TimesTwo {
    fn new() -> Self {
        Self {}
    }
}

impl Rule for TimesTwo {
    fn is_ok(&self, rule: &str) -> bool {
        let mut pairs: HashMap<String, HashSet<i64>> = HashMap::new();
        let mut last = ' ';
        for (idx, c) in rule.chars().enumerate() {
            let pair = format!("{last}{c}");
            let index = idx as i64;
            let entry = pairs
                .entry(pair)
                .or_insert(HashSet::from([index - 1, index]));

            if !entry.contains(&index) && !entry.contains(&(index - 1)) {
                return true;
            }
            last = c;
        }
        return false;
    }
}

struct OneLetterBetween {}

impl OneLetterBetween {
    fn new() -> Self {
        Self {}
    }
}

impl Rule for OneLetterBetween {
    fn is_ok(&self, rule: &str) -> bool {
        let mut last = ' ';
        let mut before_last = ' ';
        for c in rule.chars() {
            if c == before_last {
                return true;
            }
            before_last = last;
            last = c;
        }
        return false;
    }
}

struct ThreeVowels {
    regex: Regex,
}

impl ThreeVowels {
    fn new() -> Result<Self, regex::Error> {
        let regex = Regex::new(r"(.*[aeiou]){3,}.*")?;
        Ok(Self { regex })
    }
}

impl Rule for ThreeVowels {
    fn is_ok(&self, rule: &str) -> bool {
        self.regex.is_match(rule)
    }
}

struct TwiceInRow {}

impl TwiceInRow {
    fn new() -> Self {
        Self {}
    }
}

impl Rule for TwiceInRow {
    fn is_ok(&self, rule: &str) -> bool {
        let mut last = ' ';
        for c in rule.chars() {
            if last == c {
                return true;
            }
            last = c;
        }
        return false;
    }
}

struct NotContains {
    regex: Regex,
}

impl NotContains {
    fn new() -> Result<Self, regex::Error> {
        let regex = Regex::new("ab|cd|pq|xy")?;
        Ok(Self { regex })
    }
}

impl Rule for NotContains {
    fn is_ok(&self, rule: &str) -> bool {
        !self.regex.is_match(rule)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_times_two() {
        // Arrange
        let ok_line = "xyxy";
        let false_line = "aaa";
        let rule = TimesTwo::new();

        // Act
        let ok_result = rule.is_ok(ok_line);
        let false_result = rule.is_ok(false_line);

        // Assert
        assert_eq!(ok_result, true);
        assert_eq!(false_result, false);
    }

    #[test]
    fn test_one_letter_between() {
        // Arrange
        let ok_line = "xyx";
        let false_line = "xyy";
        let rule = OneLetterBetween::new();

        // Act
        let ok_result = rule.is_ok(ok_line);
        let false_result = rule.is_ok(false_line);

        // Assert
        assert_eq!(ok_result, true);
        assert_eq!(false_result, false);
    }

    #[test]
    fn test_not_contains() {
        // Arrange
        let ok_line = "aieasdae";
        let false_line = "aaaabtt";
        let rule = NotContains::new().unwrap();

        // Act
        let ok_result = rule.is_ok(ok_line);
        let false_result = rule.is_ok(false_line);

        // Assert
        assert_eq!(ok_result, true);
        assert_eq!(false_result, false);
    }

    #[test]
    fn test_twice_in_row() {
        // Arrange
        let ok_line = "rrie";
        let false_line = "aie";
        let rule = TwiceInRow {};

        // Act
        let ok_result = rule.is_ok(ok_line);
        let false_result = rule.is_ok(false_line);

        // Assert
        assert_eq!(ok_result, true);
        assert_eq!(false_result, false);
    }

    #[test]
    fn test_three_vowels() {
        // Arrange
        let ok_line = "aie";
        let false_line = "rrie";
        let rule = ThreeVowels::new().unwrap();

        // Act
        let ok_result = rule.is_ok(ok_line);
        let false_result = rule.is_ok(false_line);

        // Assert
        assert_eq!(ok_result, true);
        assert_eq!(false_result, false);
    }
}
