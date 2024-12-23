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

    use anyhow::{anyhow, Ok};
    use std::{
        borrow::Cow,
        collections::{HashMap, HashSet, VecDeque},
        u32, usize,
    };

    trait Keypad: Sized {
        const N: [(i32, i32, char); 4] = [(0, 1, 'v'), (0, -1, '^'), (1, 0, '>'), (-1, 0, '<')];

        fn get_grid() -> Vec<(i32, i32, char)>;

        fn new(
            distances: HashMap<(char, char), String>,
            paths: HashMap<(char, char), Vec<Vec<char>>>,
        ) -> Self;

        fn distances(&self) -> &HashMap<(char, char), String>;

        fn paths(&self) -> &HashMap<(char, char), Vec<Vec<char>>>;

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
                // code.push("A");
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
                    queue.push_back((neighbor, d, Vec::<char>::from(['A'])));
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
                        distances_all.insert((*c, c_next), Vec::from([vec!['A', 'A']]));
                        continue;
                    }

                    path.push(d);
                    let mut path_to_insert = path.to_vec();
                    path_to_insert.push('A');
                    distances_all
                        .entry((*c, c_next))
                        .and_modify(|v| v.push(path_to_insert.to_vec()))
                        .or_insert_with(|| Vec::from([path_to_insert]));
                    for (x, y, d) in NumericKeypad::N {
                        let neighbor = (next.0 + x, next.1 + y);
                        queue.push_back((neighbor, d, path.to_vec()));
                    }
                }
            }
            let mut distances: HashMap<(char, char), String> = HashMap::new();
            for (k, v) in &distances_all {
                let mut scored: Vec<(i32, &Vec<char>)> =
                    v.iter().map(|n| (Self::path_score(n), n)).collect();
                scored.sort_by_key(|&(score, _)| score);
                distances.insert(*k, scored[0].1.iter().collect());
            }
            Self::new(distances, distances_all)
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
        paths: HashMap<(char, char), Vec<Vec<char>>>,
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

        fn distances(&self) -> &HashMap<(char, char), String> {
            &self.distances
        }

        fn paths(&self) -> &HashMap<(char, char), Vec<Vec<char>>> {
            &self.paths
        }

        fn new(
            distances: HashMap<(char, char), String>,
            paths: HashMap<(char, char), Vec<Vec<char>>>,
        ) -> Self {
            Self { distances, paths }
        }
    }

    #[test]
    fn test_shortest_debt_0() -> Result<()> {
        // Arrange
        let n = DirectionalKeypad::initialize();

        // Act
        let actual = n.shortest('<', 'A', 0, usize::MAX)?;

        // Assert
        assert_eq!(actual, 4);
        Ok(())
    }

    #[test]
    fn test_shortest_debt_1() -> Result<()> {
        // Arrange
        let n = DirectionalKeypad::initialize();

        // Act
        let actual = n.shortest('<', 'A', 1, usize::MAX)?;

        // Assert
        assert_eq!(actual, 8);
        Ok(())
    }

    #[test]
    fn test_1() -> Result<()> {
        // Arrange
        let d = DirectionalKeypad::initialize();
        let n = NumericKeypad::initialize();
        let input = "029A";
        let formatted: Vec<char> = format!("A{input}").chars().collect();
        let parameters = [
            ("<A^A>^^AvvvA".len(), 0),
            ("v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len(), 1),
            (
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(),
                2,
            ),
        ];

        for (expected_length, debt) in parameters {
            let mut final_count = 0;
            for i in 1..formatted.len() {
                let from = formatted[i - 1];
                let to = formatted[i];
                let paths = n
                    .paths()
                    .get(&(from, to))
                    .ok_or_else(|| anyhow!("missing {:?}", (from, to)))?;
                let mut shortest_path = usize::MAX;
                for path in paths {
                    let s = d.shortest_segment(path, debt, shortest_path)?;
                    if s < shortest_path {
                        shortest_path = s;
                    }
                }
                final_count += shortest_path;
            }
            assert_eq!(expected_length, final_count);
        }

        Ok(())
    }

    ///     +---+---+
    ///     | ^ | A |
    /// +---+---+---+
    /// | < | v | > |
    /// +---+---+---+
    struct DirectionalKeypad {
        distances: HashMap<(char, char), String>,
        paths: HashMap<(char, char), Vec<Vec<char>>>,
    }

    impl DirectionalKeypad {
        const G: [(i32, i32, char); 5] = [
            (0, 1, '<'),
            (1, 1, 'v'),
            (2, 1, '>'),
            (1, 0, '^'),
            (2, 0, 'A'),
        ];

        fn shortest_segment(
            &self,
            segment: &Vec<char>,
            debt: u16,
            mut best: usize,
        ) -> Result<usize> {
            if debt == 0 {
                return Ok(segment.len() - 1);
            }
            let mut segment_size = 0;
            for i in 1..segment.len() {
                let from = segment[i - 1];
                let to = segment[i];
                let paths = self
                    .paths()
                    .get(&(from, to))
                    .ok_or_else(|| anyhow!("missing {:?}", (from, to)))?;
                let mut shortest_path = usize::MAX;
                for path in paths {
                    let s = self.shortest_segment(path, debt - 1, best)?;
                    if s < shortest_path {
                        shortest_path = s;
                    }
                }
                segment_size += shortest_path;
                if segment_size > best {
                    return Ok(best);
                }
            }

            Ok(segment_size)
        }

        fn shortest(&self, from: char, to: char, debt: u16, best: usize) -> Result<usize> {
            let paths = self
                .paths()
                .get(&(from, to))
                .ok_or_else(|| anyhow!("missing {:?}", (from, to)))?;

            if debt == 0 {
                let p: String = paths[0].iter().collect();
                return Ok(p.len());
            }
            let mut this_best = best;
            for path in paths {
                let p: String = path.iter().collect();
                let mut steps = vec!['A'];
                for c in p.chars() {
                    steps.push(c);
                }
                let mut count = 0;
                for i in 1..steps.len() {
                    let from_n = steps[i - 1];
                    let to_n = steps[i];
                    count += self.shortest(from_n, to_n, debt - 1, this_best)?;
                    if count >= this_best {
                        break;
                    }
                }
                if count >= this_best {
                    continue;
                }
                this_best = count;
            }

            if this_best < best {
                Ok(this_best)
            } else {
                Ok(best)
            }
        }
    }

    impl Keypad for DirectionalKeypad {
        fn get_grid() -> Vec<(i32, i32, char)> {
            Self::G.to_vec()
        }

        fn paths(&self) -> &HashMap<(char, char), Vec<Vec<char>>> {
            &self.paths
        }

        fn new(
            distances: HashMap<(char, char), String>,
            paths: HashMap<(char, char), Vec<Vec<char>>>,
        ) -> Self {
            Self { distances, paths }
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
    fn test_directional_generate() -> Result<()> {
        // Arrange
        let input = "^>";
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
