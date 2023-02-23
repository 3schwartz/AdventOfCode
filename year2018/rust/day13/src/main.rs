use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{File};
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
    VecDeque<Cart>,
) {
    let mut track: HashMap<(usize, usize), char> = HashMap::new();
    let mut carts_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut carts: VecDeque<Cart> = VecDeque::new();

    for (y, line) in lines.enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            match char {
                ' ' | '-' | '|' => (),
                '>' | '^' | 'v' | '<' => {
                    let cart = Cart::new((x, y), char);
                    carts.push_back(cart);
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
    carts_init: VecDeque<Cart>,
) -> Option<(usize, usize)> {
    let mut next_carts: Vec<Cart> = Vec::new();
    let mut carts = Vec::from(carts_init);
    carts.sort();

    let location: Option<(usize, usize)> = loop {
        let popped = carts.pop();
        if popped.is_none() {
            if next_carts.len() == 1 {
                break next_carts.iter().map(|c| c.position).next();    
            }
            next_carts.sort();
            carts = next_carts;
            next_carts = Vec::new();
            continue;
        }
        let cart = popped.unwrap();

        if !carts_locations.remove(&cart.position) {
            continue;
        }

        let movement = track
            .get(&cart.position)
            .map(|c| cart.sign_to_direction(c))
            .unwrap_or(Movement::new(cart.direction, cart.turns));

        let new_card = cart.new_from_movement(movement);
        
        let unique = carts_locations.insert(new_card.position);

        if unique {
            next_carts.push(new_card);
            continue;
        }
        carts_locations.remove(&new_card.position);
        next_carts.retain(|c| c.position != new_card.position);
    };

    return location;
}

fn find_first_crash(
    track: &HashMap<(usize, usize), char>,
    mut carts_locations: HashSet<(usize, usize)>,
    mut carts: VecDeque<Cart>,
) -> Option<(usize, usize)> {
    let crash: Option<(usize, usize)> = loop {
        let popped = carts.pop_front();
        if popped.is_none() {
            break None;
        }
        let cart = popped.unwrap();

        let movement = track
            .get(&cart.position)
            .map(|c| cart.sign_to_direction(c))
            .unwrap_or(Movement::new(cart.direction, cart.turns));
        carts_locations.remove(&cart.position);

        let new_card = cart.new_from_movement(movement);
        let unique = carts_locations.insert(new_card.position);

        if !unique {
            break Some(new_card.position);
        }
        carts.push_back(new_card);
    };
    return crash;
}

struct Movement {
    direction: (i8, i8),
    turn: u32,
}

impl Movement {
    fn new(direction: (i8, i8), turn: u32) -> Self {
        Self { direction, turn }
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
            return other.position.0.cmp(&self.position.0)
        }
        return other.position.1.cmp(&self.position.1)
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

    fn cart_to_direction(c: char) -> (i8, i8) {
        match c {
            '>' => (1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            '<' => (-1, 0),
            _ => panic!("direction not known: {}", c),
        }
    }

    fn new_from_movement(self, movement: Movement) -> Cart {
        let position = (
            self.position.0 as i32 + movement.direction.0 as i32,
            self.position.1 as i32 + movement.direction.1 as i32,
        );
        return Cart {
            direction: movement.direction,
            position: (position.0 as usize, position.1 as usize),
            turns: movement.turn,
        };
    }

    fn sign_to_direction(&self, c: &char) -> Movement {
        match (c, self.direction, self.turns % 3) {
            ('+', _, 0) => Movement::new((self.direction.1, -self.direction.0), self.turns + 1),
            ('+', _, 1) => Movement::new((self.direction.0, self.direction.1), self.turns + 1),
            ('+', _, 2) => Movement::new((-self.direction.1, self.direction.0), self.turns + 1),
            ('/', (0, -1), _) | ('\\', (1, 0), _) | ('/', (0, 1), _) | ('\\', (-1, 0), _) => {
                Movement::new((-self.direction.1, self.direction.0), self.turns)
            }
            ('/', (-1, 0), _) | ('\\', (0, 1), _) | ('/', (1, 0), _) | ('\\', (0, -1), _) => {
                Movement::new((self.direction.1, -self.direction.0), self.turns)
            }
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
            Cart{position: (1,6), direction: (0,0), turns:0},
            Cart{position: (7,2), direction: (0,0), turns:0},
            Cart{position: (3,4), direction: (0,0), turns:0},
            Cart{position: (7,4), direction: (0,0), turns:0}
            ];
        let expected = vec![
            (1,6),
            (7,4),
            (3,4),
            (7,2)
            ];            
        
        // Act
        carts.sort();

        // Assert
        assert_eq!(
            carts.iter().map(|c| c.position).collect::<Vec<(usize,usize)>>(),
            expected);
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
