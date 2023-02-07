use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("../data/day12_data.txt")
        .expect("not able to find file");
    let generator = Generator::new(input);
    let mut sum = generator.run(20);

    println!("Part 1: {sum}");

    // sum = generator.run(50_000_000_000);
    
    // println!("Part 2: {sum}");
}

#[derive(PartialEq, Debug)]
struct Rule {
    sequence: Vec<(usize, char)>,
    to: char
}

impl Rule {
    fn create_sequence(input: &str) -> Vec<(usize, char)> {
        input.chars().enumerate().collect::<Vec<(usize, char)>>()
    }
}

struct Generator {
    plants: HashMap<i32,char>,
    length: usize,
    rules: Vec<Rule>
}

impl Generator {
    fn new(input: String) -> Self {
        let lines = input.split("\r\n").collect::<Vec<&str>>();

        let plants = lines[0][15..]
            .chars()
            .enumerate()
            .map(|(i,c)| (i as i32, c))
            .collect::<HashMap<i32, char>>();
        let rules = lines[2..]
            .iter()
            .map(|l| Rule{sequence: Rule::create_sequence(&l[..5]), to: l.chars().next_back().unwrap()})
            .collect::<Vec<Rule>>();

        return Self { plants, rules, length: lines[0][15..].len()}
    }

    fn plants_sum(&self, state: &HashMap<i32, char>) -> i32 {
        state.iter()
            .map(|(i,c)| {
                if c == &'#' {
                    return i;
                }
                return &0;
            }).sum()
    }

    fn get_plant(&self, state: &HashMap<i32, char>, idx: &i32) -> char {
        state
            .get(idx)
            .unwrap_or(&'.').to_owned()
    }

    fn run(&self, count: u128) -> i32{
        let mut state = self.plants.clone();
        let mut from: i32 = 0;
        let mut to: i32 = self.length as i32;
        for _ in 0..count {
            from -= 4;
            to += 4;
            let mut new_state: HashMap<i32, char> = HashMap::new();
            for idx in from..=to {
                let mut replaced = false;
                for r in &self.rules {
                    let mut fits = true;
                    for (i, c) in &r.sequence {
                        let pot = self.get_plant(&state, &(idx + *i as i32 - 2));
                        if pot == *c {
                            fits &= true;
                            continue;
                        }
                        fits = false;
                        break;
                    }
                    if fits {
                        replaced = true;
                        new_state.insert(idx as i32, r.to);
                        break;
                    }
                }
                if replaced {
                    continue;
                }
                new_state.insert(idx as i32, '.');
            }
            state = new_state;
        };

        return self.plants_sum(&state);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_when_new_generator_then_correct_state() {
        // Arrange
        let input = fs::read_to_string("../data/day12_test_data.txt")
            .expect("file not found");

        // Act
        let generator = Generator::new(input);

        // Assert
        assert_eq!(generator.plants.get(&0), Some(&'#'));
        assert_eq!(generator.rules.first(), Some(&Rule{sequence: Rule::create_sequence("...##"), to: '#'}));
    }

    #[test]
    fn test_part1() {
        // Arrange
        let input = fs::read_to_string("../data/day12_test_data.txt")
            .expect("file not found");

        // Act
        let generator = Generator::new(input);
        let sum = generator.run(20);

        // Assert
        assert_eq!(sum, 325)
    }
}
