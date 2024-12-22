use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day21_data.txt")?;

    for line in input.lines() {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use anyhow::anyhow;
    use std::{
        borrow::Cow,
        collections::{HashMap, HashSet, VecDeque},
    };

    trait Keypad: Sized {
        const N: [(i32, i32, char); 4] = [(0, 1, 'v'), (0, -1, '^'), (1, 0, '>'), (-1, 0, '<')];

        fn get_grid() -> Vec<(i32, i32, char)>;

        fn new(distances: HashMap<(char, char), String>) -> Self;

        fn distances(&self) -> &HashMap<(char, char), String>;

        fn generate(&self, input: &str) -> Result<String> {
            let mut steps = vec!['A'];
            for c in input.chars() {
                steps.push(c);
            }
            let mut code = vec![];
            for i in 1..steps.len() {
                let from = steps[i - 1];
                let to = steps[i];
                let path = self
                    .distances()
                    .get(&(from, to))
                    .ok_or_else(|| anyhow!("missing {:?}", (from, to)))?;
                code.push(path.as_str());
                code.push("A");
            }
            Ok(code.iter().map(|s| *s).collect())
        }

        fn initialize() -> Self {
            let mut distances: HashMap<(char, char), String> = HashMap::new();
            let grid: HashMap<(i32, i32), char> = Self::get_grid()
                .iter()
                .map(|&(x, y, c)| ((x, y), c))
                .collect();
            for (e, c) in &grid {
                let mut queue = VecDeque::new();
                for (x, y, d) in Self::N {
                    let neighbor = (e.0 + x, e.1 + y);
                    queue.push_back((neighbor, d, Vec::<char>::new()));
                }
                while let Some((next, d, mut path)) = queue.pop_front() {
                    if !&grid.contains_key(&next) {
                        continue;
                    }
                    let c_next = *grid.get(&next).unwrap();
                    if distances.contains_key(&(*c, c_next)) {
                        continue;
                    }
                    if *c == c_next {
                        distances.insert((*c, c_next), "".to_string());
                        continue;
                    }

                    path.push(d);
                    distances.insert((*c, c_next), path.to_vec().iter().collect());
                    for (x, y, d) in NumericKeypad::N {
                        let neighbor = (next.0 + x, next.1 + y);
                        queue.push_back((neighbor, d, path.to_vec()));
                    }
                }
            }
            Self::new(distances)
        }
    }

    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    ///     | 0 | A |
    ///     +---+---+
    struct NumericKeypad {
        distances: HashMap<(char, char), String>,
    }

    impl NumericKeypad {
        const G: [(i32, i32, char); 11] = [
            (1, 3, '0'),
            (2, 3, 'A'),
            (0, 2, '1'),
            (1, 2, '2'),
            (2, 2, '3'),
            (0, 1, '4'),
            (1, 1, '5'),
            (2, 1, '6'),
            (0, 0, '7'),
            (1, 0, '8'),
            (2, 0, '9'),
        ];
    }

    impl Keypad for NumericKeypad {
        fn get_grid() -> Vec<(i32, i32, char)> {
            Self::G.to_vec()
        }

        fn new(distances: HashMap<(char, char), String>) -> Self {
            Self { distances }
        }

        fn distances(&self) -> &HashMap<(char, char), String> {
            &self.distances
        }
    }

    /// +---+---+
    /// | ^ | A |
    /// +---+---+---+
    /// | < | v | > |
    /// +---+---+---+
    struct DirectionalKeypad {
        distances: HashMap<(char, char), String>,
    }

    impl DirectionalKeypad {
        const G: [(i32, i32, char); 5] = [
            (0, 1, '<'),
            (1, 1, 'v'),
            (2, 1, '>'),
            (1, 0, '^'),
            (2, 0, 'A'),
        ];
    }

    impl Keypad for DirectionalKeypad {
        fn get_grid() -> Vec<(i32, i32, char)> {
            Self::G.to_vec()
        }

        fn new(distances: HashMap<(char, char), String>) -> Self {
            Self { distances }
        }

        fn distances(&self) -> &HashMap<(char, char), String> {
            &self.distances
        }
    }

    #[test]
    fn test_generate_keypad() {
        // Act
        let n = NumericKeypad::initialize();

        // Assert
        assert!(n.distances.get(&('A', '9')).is_some());
        assert!(n.distances.get(&('9', 'A')).is_some());
    }

    #[test]
    fn test_numeric_input_generate() {
        // Arrange
        let input = "029A";
        let n = NumericKeypad::initialize();

        // Act
        let output = n.generate(input);

        // Assert
        assert!(output.is_ok());
        let result = output.unwrap();
        assert!(
            HashSet::from(["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"])
                .contains(result.as_str())
        );
    }

    #[test]
    fn test_directional_input_generate() -> Result<()> {
        // Arrange
        // "<       A ^  A  >  ^   ^ A  v   v v A";
        // "v<<A >>^A <A >A vA <^A A >A <vA A A >^A";
        let input = "<A^A^^>AvvvA";
        let one_output = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let expected_a_count = get_a_count(one_output);
        let n = DirectionalKeypad::initialize();

        // Act
        let output = n.generate(input)?;

        // Assert
        assert_eq!(output.len(), one_output.len());
        let actual_a_count = get_a_count(&output);
        assert_eq!(actual_a_count, expected_a_count);
        Ok(())
    }

    #[test]
    fn test_directional_input_generate_2() -> Result<()> {
        // Arrange
        let input = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let one_output = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
        let expected_a_count = get_a_count(one_output);
        let n = DirectionalKeypad::initialize();

        // Act
        let output = n.generate(input)?;

        // Assert
        assert_eq!(output.len(), one_output.len());
        let actual_a_count = get_a_count(&output);
        assert_eq!(actual_a_count, expected_a_count);
        Ok(())
    }

    fn get_a_count(s: &str) -> u32 {
        s.chars().fold(0, |mut acc, e| {
            if e == 'A' {
                acc += 1
            };
            acc
        })
    }

    struct KeypadLink<K: Keypad, C: Keypad> {
        keypad: K,
        child: Option<C>,
    }

    impl<K: Keypad, C: Keypad> KeypadLink<K, C> {
        fn new(keypad: K, child: Option<C>) -> Self {
            Self { keypad, child }
        }

        fn evaluate(&self, input: &str) -> Result<String> {
            let e = match &self.child {
                Some(c) => Cow::Owned(c.generate(input)?),
                None => Cow::Borrowed(input),
            };
            self.keypad.generate(&e)
        }
    }

    #[test]
    fn test_keypad_link_level_1() -> Result<()> {
        // Arrange
        let l0 = KeypadLink::new(
            DirectionalKeypad::initialize(),
            Some(NumericKeypad::initialize()),
        );
        let expected_output = "v<<A>^>A<A>A<AAv>A^Av<AAA^>A";
        let input = "029A";

        // Act
        let actual = l0.evaluate(input)?;

        // Assert
        assert_eq!(expected_output, actual);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }
}
