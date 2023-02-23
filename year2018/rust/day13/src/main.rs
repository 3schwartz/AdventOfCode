use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let lines = read_lines("../data/day13_data.txt").unwrap();
    let (track, carts_locations, carts) = initialize(lines);
    let crash = find_first_crash(&track, carts_locations.clone(), carts.clone());

    match crash {
        Some(c) => println!("Part 1: {:?}", c),
        None => panic!("not able to find solution for Part 1."),
    }

    let location = find_last_location(&track, carts_locations, carts);

    match location {
        Some(l) => println!("Part 2: {:?}", l),
        None => panic!("not able to find solution for Part 2."),
    }
}

fn initialize(
    lines: io::Lines<io::BufReader<File>>,
) -> (
    HashMap<(usize, usize), char>,
    HashSet<(usize, usize)>,
    Vec<Cart>,
) {
    let mut track: HashMap<(usize, usize), char> = HashMap::new();
    let mut carts_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut carts: Vec<Cart> = Vec::new();

    for (y, line) in lines.enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            match char {
                ' ' | '-' | '|' => (),
                '>' | '^' | 'v' | '<' => {
                    let cart = Cart::new((x, y), char);
                    carts.push(cart);
                    carts_locations.insert((x, y));
                }
                _ => {
                    track.insert((x, y), char);
                }
            }
        }
    }
    return (track, carts_locations, carts);
}

fn find_last_location(
    track: &HashMap<(usize, usize), char>,
    mut carts_locations: HashSet<(usize, usize)>,
    mut carts: Vec<Cart>,
) -> Option<(usize, usize)> {
    let mut next_carts: Vec<Cart> = Vec::new();
    carts.sort();

    loop {
        let popped = carts.pop();
        match (popped.is_none(), next_carts.len() == 1) {
            (true, true) => return next_carts.iter().map(|c| c.position).next(),
            (true, false) => {
                next_carts.sort();
                carts = next_carts;
                next_carts = Vec::new();
                continue;
            }
            (false, _) => (),
        }
        let cart = popped.unwrap();

        if !carts_locations.remove(&cart.position) {
            continue;
        }

        let new_card = track
            .get(&cart.position)
            .map(|c| {
                let movement = cart.sign_to_direction(c);
                cart.create(movement.0, movement.1)
            })
            .unwrap_or(cart.create(cart.direction, 0));

        let unique = carts_locations.insert(new_card.position);

        if unique {
            next_carts.push(new_card);
            continue;
        }
        carts_locations.remove(&new_card.position);
        next_carts.retain(|c| c.position != new_card.position);
    }
}

fn find_first_crash(
    track: &HashMap<(usize, usize), char>,
    mut carts_locations: HashSet<(usize, usize)>,
    carts_init: Vec<Cart>,
) -> Option<(usize, usize)> {
    let mut carts = VecDeque::from(carts_init);

    loop {
        let popped = carts.pop_front();
        if popped.is_none() {
            break None;
        }
        let cart = popped.unwrap();
        carts_locations.remove(&cart.position);

        let new_card = track
            .get(&cart.position)
            .map(|c| {
                let movement = cart.sign_to_direction(c);
                cart.create(movement.0, movement.1)
            })
            .unwrap_or(cart.create(cart.direction, 0));

        let unique = carts_locations.insert(new_card.position);

        if !unique {
            return Some(new_card.position);
        }
        carts.push_back(new_card);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cart {
    position: (usize, usize),
    direction: (i8, i8),
    turns: u32,
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.position.1 == other.position.1 {
            return other.position.0.cmp(&self.position.0);
        }
        return other.position.1.cmp(&self.position.1);
    }
}

impl Cart {
    fn new(position: (usize, usize), c: char) -> Self {
        let direction = Cart::cart_to_direction(c);
        return Self {
            position,
            direction,
            turns: 0,
        };
    }

    fn create(&self, direction: (i8, i8), turn: u32) -> Cart {
        let position = (
            self.position.0 as i32 + direction.0 as i32,
            self.position.1 as i32 + direction.1 as i32,
        );
        return Cart {
            direction,
            position: (position.0 as usize, position.1 as usize),
            turns: self.turns + turn,
        };
    }

    fn cart_to_direction(c: char) -> (i8, i8) {
        match c {
            '>' => (1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            '<' => (-1, 0),
            _ => panic!("direction not known: {}", c),
        }
    }

    fn direction_to_index(&self) -> usize {
        match self.direction {
            (0, -1) => 0,
            (1, 0) => 1,
            (0, 1) => 2,
            (-1, 0) => 3,
            _ => panic!("direction to index failed: {:?}", self.direction),
        }
    }

    fn index_to_direction(&self, idx: usize) -> (i8, i8) {
        match idx {
            0 => (0, -1),
            1 => (1, 0),
            2 => (0, 1),
            3 => (-1, 0),
            _ => panic!("index to direction failed: {}", idx),
        }
    }

    fn sign_to_direction(&self, c: &char) -> ((i8, i8), u32) {
        let idx = self.direction_to_index();
        match c {
            '+' => match self.turns % 3 {
                0 => (self.index_to_direction((idx + 3) % 4), 1),
                1 => (self.direction, 1),
                _ => (self.index_to_direction((idx + 1) % 4), 1),
            },
            '/' => (self.index_to_direction([1, 0, 3, 2][idx]), 0),
            '\\' => (self.index_to_direction([3, 2, 1, 0][idx]), 0),
            _ => panic!("sign not known: {}", c),
        }
    }
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    return Ok(io::BufReader::new(file).lines());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cart_order() {
        // Arrange
        let mut carts = vec![
            Cart {
                position: (1, 6),
                direction: (0, 0),
                turns: 0,
            },
            Cart {
                position: (7, 2),
                direction: (0, 0),
                turns: 0,
            },
            Cart {
                position: (3, 4),
                direction: (0, 0),
                turns: 0,
            },
            Cart {
                position: (7, 4),
                direction: (0, 0),
                turns: 0,
            },
        ];
        let expected = vec![(1, 6), (7, 4), (3, 4), (7, 2)];

        // Act
        carts.sort();

        // Assert
        assert_eq!(
            carts
                .iter()
                .map(|c| c.position)
                .collect::<Vec<(usize, usize)>>(),
            expected
        );
    }

    #[test]
    fn test_part1() {
        // Arrange
        let lines = read_lines("../data/day13_test_data.txt").unwrap();
        let (track, carts_locations, carts) = initialize(lines);

        // Act
        let crash = find_first_crash(&track, carts_locations, carts);

        // Assert
        assert_eq!(crash, Some((7, 3)));
    }

    #[test]
    fn test_part2() {
        // Arrange
        let lines = read_lines("../data/day13_test2_data.txt").unwrap();
        let (track, carts_locations, carts) = initialize(lines);

        // Act
        let location = find_last_location(&track, carts_locations, carts);

        // Assert
        assert_eq!(location, Some((6, 4)));
    }
}
