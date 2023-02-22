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
    carts_locations: HashSet<(usize, usize)>,
    carts: VecDeque<Cart>,
) -> Option<(usize, usize)> {

    let mut locations = carts_locations
        .into_iter()
        .map(|c| (0, c))
        .collect::<HashSet<(i32, (usize,usize))>>();
    let mut carts_with_ticks = carts
        .into_iter()
        .map(|c| (0, c))
        .collect::<VecDeque<(i32, Cart)>>();

    let mut tick_count = 0;
    let mut tick_length = carts_with_ticks.len();
    let mut ticks = 0;

    let location: Option<(usize, usize)> = loop {
        let popped = carts_with_ticks.pop_front();
        if popped.is_none() {
            break None;
        }
        let info = popped.unwrap();
        let cart = info.1;

        if carts_with_ticks.len() == 0 && tick_count % tick_length == 0 {
            break Some(cart.position);
        }

        if tick_count % tick_length == 0 {
            tick_count = 0;
            tick_length = carts_with_ticks.len() + 1;
            ticks += 1;
        }
        tick_count += 1;

        let movement = track
            .get(&cart.position)
            .map(|c| cart.sign_to_direction(c))
            .unwrap_or(Movement::new(cart.direction, cart.turns));
        locations.remove(&(ticks, cart.position));

        let new_card = cart.new_from_movement(movement);
        
        let unique = locations.insert((ticks+1, new_card.position));

        if unique {
            carts_with_ticks.push_back((ticks+1,new_card));
            continue;
        }
        locations.remove(&(ticks+1, new_card.position));
        carts_with_ticks.retain(|(t, c)| !(c.position == new_card.position && *t == ticks +1));
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

#[derive(Clone)]
struct Cart {
    position: (usize, usize),
    direction: (i8, i8),
    turns: u32,
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
