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
            let mut distances_all: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
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
                    if let Some(prior) = distances_all.get(&(*c, c_next)) {
                        if prior[0].len() < path.len() + 1 {
                            continue;
                        }
                    }
                    if *c == c_next {
                        distances_all.insert((*c, c_next), Vec::from(["".chars().collect()]));
                        continue;
                    }

                    path.push(d);
                    distances_all
                        .entry((*c, c_next))
                        .and_modify(|v| v.push(path.to_vec()))
                        .or_insert_with(|| Vec::from([path.to_vec()]));
                    for (x, y, d) in NumericKeypad::N {
                        let neighbor = (next.0 + x, next.1 + y);
                        queue.push_back((neighbor, d, path.to_vec()));
                    }
                }
            }
            let mut distances: HashMap<(char, char), String> = HashMap::new();
            for (k, v) in distances_all {
                let mut scored: Vec<(i32, &Vec<char>)> =
                    v.iter().map(|n| (Self::path_score(n), n)).collect();
                scored.sort_by_key(|&(score, _)| score);
                distances.insert(k, scored[0].1.iter().collect());
            }
            Self::new(distances)
        }

        fn path_score(chars: &Vec<char>) -> i32 {
            let mut score = 0;
            for i in 1..chars.len() {
                if chars[i - 1] != chars[i] {
                    score += 1;
                }
            }
            score
        }

        // fn reorder_string(input: &Vec<char>) -> String {
        //     let order = |c: char| match c {
        //         '>' => 0,
        //         '<' => 1,
        //         '^' => 2,
        //         'v' => 3,
        //         _ => panic!("{c}"),
        //     };

        //     let mut chars: Vec<char> = input.to_vec();
        //     chars.sort_by_key(|&c| order(c));
        //     chars.into_iter().collect()
        // }
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

    ///     +---+---+
    ///     | ^ | A |
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
    fn test_numeric_input_generate() -> Result<()> {
        // Arrange
        let input = "029A";
        let n = NumericKeypad::initialize();

        // Act
        let output = n.generate(input)?;

        // Assert
        assert!(
            HashSet::from(["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"])
                .contains(output.as_str())
        );
        assert_eq!(output, "<A^A>^^AvvvA");
        Ok(())
    }

    #[test]
    fn test_directional_input_generate() -> Result<()> {
        // Arrange
        let input = "<A^A>^^AvvvA";
        let one_output = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let expected_a_count = get_a_count(one_output);
        let n = DirectionalKeypad::initialize();

        // Act
        let output = n.generate(input)?;

        // Assert
        assert_eq!(output.len(), one_output.len());
        let actual_a_count = get_a_count(&output);
        assert_eq!(actual_a_count, expected_a_count);
        assert_eq!(output, "<<vA>>^A<A>AvA<^AA>A<vAAA>^A");
        Ok(())
    }

    #[test]
    fn test_directional_input_generate_2_2() -> Result<()> {
        // Arrange
        let input = "<<vA>>^A<A>AvA<^AA>A<vAAA>^A";
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

    trait KeypadChoice {
        fn evaluate(&self, input: &str) -> Result<String>;
    }

    struct KeypadLink<K: Keypad, KC: KeypadChoice> {
        keypad: K,
        child: Option<KC>,
    }

    impl<K: Keypad, KC: KeypadChoice> KeypadLink<K, KC> {
        fn new(keypad: K, child: Option<KC>) -> Self {
            Self { keypad, child }
        }
    }

    impl<K: Keypad, KC: KeypadChoice> KeypadChoice for KeypadLink<K, KC> {
        fn evaluate(&self, input: &str) -> Result<String> {
            let e = match &self.child {
                Some(c) => Cow::Owned(c.evaluate(input)?),
                None => Cow::Borrowed(input),
            };
            self.keypad.generate(&e)
        }
    }

    impl<T: Keypad> KeypadChoice for T {
        fn evaluate(&self, input: &str) -> Result<String> {
            self.generate(input)
        }
    }

    #[test]
    fn test_keypad_link_level_0() -> Result<()> {
        // Arrange
        let l0 = KeypadLink::<NumericKeypad, NumericKeypad>::new(NumericKeypad::initialize(), None);
        let expected_output = "<A^A>^^AvvvA";
        let input = "029A";

        // Act
        let actual = l0.evaluate(input)?;

        // Assert
        assert_eq!(expected_output, actual);
        Ok(())
    }

    #[test]
    fn test_keypad_link_level_1_v2() -> Result<()> {
        // Arrange
        let l0 = KeypadLink::<NumericKeypad, NumericKeypad>::new(NumericKeypad::initialize(), None);
        let l1 = KeypadLink::new(DirectionalKeypad::initialize(), Some(l0));
        let expected_output = "<<vA>>^A<A>AvA<^AA>A<vAAA>^A";
        let input = "029A";

        // Act
        let actual = l1.evaluate(input)?;

        // Assert
        assert_eq!(expected_output, actual);
        Ok(())
    }

    #[test]
    fn test_keypad_link_level_2_v2() -> Result<()> {
        // Arrange
        let l0 = KeypadLink::<NumericKeypad, NumericKeypad>::new(NumericKeypad::initialize(), None);
        let l1 = KeypadLink::new(DirectionalKeypad::initialize(), Some(l0));
        let l2 = KeypadLink::new(DirectionalKeypad::initialize(), Some(l1));
        let expected_output =
            "<<vAA>A>^AvAA<^A>A<<vA>>^AvA^A<vA>^A<<vA>^A>AAvA^A<<vA>A>^AAAvA<^A>A";
        let input = "029A";

        // Act
        let actual = l2.evaluate(input)?;

        // Assert
        assert_eq!(expected_output, actual);
        Ok(())
    }

    #[test]
    fn test_keypad_link_parametrized() -> Result<()> {
        // Arrange
        let l0 = KeypadLink::<NumericKeypad, NumericKeypad>::new(NumericKeypad::initialize(), None);
        let l1 = KeypadLink::new(DirectionalKeypad::initialize(), Some(l0));
        let l2 = KeypadLink::new(DirectionalKeypad::initialize(), Some(l1));
        let inputs = [
            // (
            //     "029A",
            //     "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            // ),
            // (
            //     "980A",
            //     "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
            // ),
            // (
            //     "179A",
            //     "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            // ),
            // (
            //     "456A",
            //     "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
            // ),
            (
                "379A",
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            ),
        ];
        for (input, expected) in inputs {
            // Act
            let actual = l2.evaluate(input)?;

            // Assert
            assert_eq!(actual.len(), expected.len());
            let actual_a_count = get_a_count(&actual);
            assert_eq!(actual_a_count, get_a_count(&expected));
        }
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
