use anyhow::Result;
use anyhow::{anyhow, Ok};
use std::fs;
use std::{
    collections::{HashMap, VecDeque},
    num::ParseIntError,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day21_data.txt")?;

    let codes: Vec<&str> = input.lines().collect();
    let complexity = code_complexity(&codes, 2)?;

    println!("Part 1: {complexity}");

    let complexity = code_complexity(&codes, 25)?;

    println!("Part 2: {complexity}");

    Ok(())
}

trait Keypad: Sized {
    const N: [(i32, i32, char); 4] = [(0, 1, 'v'), (0, -1, '^'), (1, 0, '>'), (-1, 0, '<')];

    fn get_grid() -> Vec<(i32, i32, char)>;

    fn new(paths: HashMap<(char, char), Vec<Vec<char>>>) -> Self;

    fn paths(&self) -> &HashMap<(char, char), Vec<Vec<char>>>;

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

        Self::new(distances_all)
    }
}

fn code_complexity(input: &[&str], debt: u16) -> Result<usize> {
    let n = NumericKeypad::initialize();
    let mut complexity = 0;
    let mut cache = HashMap::new();
    for code in input {
        let final_count = n.shortest_segment(code, debt, &mut cache)?;
        let numeric_code = numeric_code(code)?;
        complexity += final_count * numeric_code;
    }
    Ok(complexity)
}

fn numeric_code(input: &str) -> Result<usize, ParseIntError> {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
}

///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
struct DirectionalKeypad {
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
        segment: &[char],
        debt: u16,
        best: usize,
        cache: &mut HashMap<(char, char, u16), usize>,
    ) -> Result<usize> {
        if debt == 0 {
            return Ok(segment.len() - 1);
        }
        let mut segment_size = 0;
        for i in 1..segment.len() {
            let from = segment[i - 1];
            let to = segment[i];
            let optimal = if let Some(cached) = cache.get(&(from, to, debt)) {
                *cached
            } else {
                let paths = self
                    .paths()
                    .get(&(from, to))
                    .ok_or_else(|| anyhow!("missing {:?}", (from, to)))?;
                let mut shortest_path = usize::MAX;
                for path in paths {
                    let s = self.shortest_segment(path, debt - 1, best, cache)?;
                    if s < shortest_path {
                        shortest_path = s;
                    }
                }
                cache.insert((from, to, debt), shortest_path);
                shortest_path
            };

            segment_size += optimal;
            if segment_size > best {
                return Ok(usize::MAX);
            }
        }

        Ok(segment_size)
    }
}

impl Keypad for DirectionalKeypad {
    fn get_grid() -> Vec<(i32, i32, char)> {
        Self::G.to_vec()
    }

    fn paths(&self) -> &HashMap<(char, char), Vec<Vec<char>>> {
        &self.paths
    }

    fn new(paths: HashMap<(char, char), Vec<Vec<char>>>) -> Self {
        Self { paths }
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

    fn shortest_segment(
        &self,
        input: &str,
        debt: u16,
        cache: &mut HashMap<(char, char, u16), usize>,
    ) -> Result<usize> {
        let formatted: Vec<char> = format!("A{input}").chars().collect();
        let d = DirectionalKeypad::initialize();

        let mut final_count = 0;
        for i in 1..formatted.len() {
            let from = formatted[i - 1];
            let to = formatted[i];
            let paths = self
                .paths()
                .get(&(from, to))
                .ok_or_else(|| anyhow!("missing {:?}", (from, to)))?;
            let mut shortest_path = usize::MAX;
            for path in paths {
                let s = d.shortest_segment(path, debt, shortest_path, cache)?;
                if s < shortest_path {
                    shortest_path = s;
                }
            }
            final_count += shortest_path;
        }
        Ok(final_count)
    }
}

impl Keypad for NumericKeypad {
    fn get_grid() -> Vec<(i32, i32, char)> {
        Self::G.to_vec()
    }

    fn paths(&self) -> &HashMap<(char, char), Vec<Vec<char>>> {
        &self.paths
    }

    fn new(paths: HashMap<(char, char), Vec<Vec<char>>>) -> Self {
        Self { paths }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() -> Result<()> {
        // Arrange
        let n = NumericKeypad::initialize();
        let input = "029A";
        let parameters = [
            ("<A^A>^^AvvvA".len(), 0),
            ("v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len(), 1),
            (
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(),
                2,
            ),
        ];
        let mut cache = HashMap::new();
        for (expected_length, debt) in parameters {
            // Act
            let final_count = n.shortest_segment(input, debt, &mut cache)?;

            // Assert
            assert_eq!(expected_length, final_count);
        }

        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        // Arrange
        let parameters = [
            ("029A", 29),
            ("980A", 980),
            ("179A", 179),
            ("456A", 456),
            ("379A", 379),
        ];

        for (input, expected) in parameters {
            // Act
            let actual = numeric_code(input)?;

            // Assert
            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[test]
    fn test_find_code_complexity() -> Result<()> {
        // Arrange
        let codes = ["029A", "980A", "179A", "456A", "379A"];

        // Act
        let complexity = code_complexity(&codes, 2)?;

        // Assert
        assert_eq!(complexity, 126_384);
        Ok(())
    }

    #[test]
    fn test_keypad_link_parametrized_shortest() -> Result<()> {
        // Arrange
        let n = NumericKeypad::initialize();
        let inputs = [
            (
                "029A",
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(),
            ),
            (
                "980A",
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len(),
            ),
            (
                "179A",
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
            ),
            (
                "456A",
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len(),
            ),
            (
                "379A",
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
            ),
        ];
        let mut cache = HashMap::new();
        for (input, expected) in inputs {
            // Act
            let final_count = n.shortest_segment(input, 2, &mut cache)?;

            // Assert
            assert_eq!(final_count, expected);
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
