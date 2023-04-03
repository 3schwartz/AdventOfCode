use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("../data/day5_data.txt").unwrap();

    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(ThreeVowels::new().unwrap()),
        Box::new(TwiceInRow::new()),
        Box::new(NotContains::new().unwrap()),
    ];

    let mut nice_strings = 0;
    for line in file.lines() {
        let mut is_ok = true;
        for rule in &rules {
            let rule_ok = rule.is_ok(line);
            if !rule_ok {
                is_ok = false;
                break;
            };
        }
        nice_strings += if is_ok {1} else {0};
    }

    println!("Part 1: {nice_strings}");
}

trait Rule {
    fn is_ok(&self, rule: &str) -> bool;
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
