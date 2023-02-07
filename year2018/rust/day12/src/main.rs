use std::fs;

fn main() {
    let input = fs::read_to_string("../data/da12_test_data");
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[derive(PartialEq, Debug)]
    struct Rule {
        sequence: String,
        to: char
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
                .map(|l| Rule{sequence: l[..5].to_string(), to: l.chars().next_back().unwrap()})
                .collect::<Vec<Rule>>();

            return Self { plants, rules, length: lines[0][15..].len()}
        }

        fn plants_sum(&self, state: &HashMap<i32, char>) -> u32 {
            state.iter()
                .map(|(_,c)| {
                    if c == &'#' {
                        return 1;
                    }
                    return 0;
                }).sum()
        }

        fn get_plant(&self, state: &HashMap<i32, char>, idx: &i32) -> char {
            state
                .get(idx)
                .unwrap_or(&'.').to_owned()
        }

        fn run(&self, count: u32) -> u32{
            let mut state = self.plants.clone();
            let mut sum = self.plants_sum(&state);
            let mut from: i32 = 0;
            let mut to: i32 = self.length as i32;
            for foo in 0..count {
                from -= 4;
                to += 4;
                let mut new_state: HashMap<i32, char> = HashMap::new();
                for idx in from..=to {
                    // let s = self.get_plant(&state, &idx);
                    let mut replaced = false;
                    for r in &self.rules {
                        // TODO Store as chars
                        let mut fits = true;
                        for (i, c) in r.sequence.chars().enumerate() {
                            let pot = self.get_plant(&state, &(idx + i as i32 - 2));
                            if pot == c {
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
                sum += self.plants_sum(&new_state);
                state = new_state;
            };

            return sum;
        }
    }

    #[test]
    fn test_when_new_generator_then_correct_state() {
        // Arrange
        let input = fs::read_to_string("../data/day12_test_data.txt")
            .expect("file not found");

        // Act
        let generator = Generator::new(input);

        // Assert
        assert_eq!(generator.plants.get(&0), Some(&'#'));
        assert_eq!(generator.rules.first(), Some(&Rule{sequence: "...##".to_string(), to: '#'}));
    }

    #[test]
    fn test_when_get_plants_sum_then_correct() {
        // Arrange
        let input = fs::read_to_string("../data/day12_test_data.txt")
            .expect("file not found");

        // Act
        let generator = Generator::new(input);
        let sum = generator.plants_sum(&generator.plants);

        // Assert
        assert_eq!(sum, 11)
    }

    #[test]
    fn test_when_generate_one() {
        // Arrange
        let input = fs::read_to_string("../data/day12_test_data.txt")
            .expect("file not found");

        // Act
        let generator = Generator::new(input);
        let sum = generator.run(1);

        // Assert
        assert_eq!(sum, 18)
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
