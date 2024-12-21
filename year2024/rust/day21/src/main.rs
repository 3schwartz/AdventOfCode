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
    use std::collections::{HashMap, VecDeque};

    use super::*;

    trait Keypad: Sized {
        const N: [(i32, i32, char); 4] = [(0, 1, 'v'), (0, -1, '^'), (1, 0, '>'), (-1, 0, '<')];

        fn get_grid() -> Vec<(i32, i32, char)>;
        fn new(distances: HashMap<(char, char), String>) -> Self;

        fn generate() -> Self {
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
                    if *c == c_next {
                        continue;
                    }
                    if distances.contains_key(&(*c, c_next)) {
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
    }

    #[test]
    fn test_generate_keypad() {
        // Act
        let n = NumericKeypad::generate();

        // Assert
        assert!(n.distances.get(&('A', '9')).is_some());
        assert!(n.distances.get(&('9', 'A')).is_some());
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
