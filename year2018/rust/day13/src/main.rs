fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::fs::File;
    use std::path::Path;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn test_some() {
        let foo: (i8,i8)= (4,0);
        let bar = &foo;
        let eggs = -bar.0;

        println!("{eggs}");
    }

    #[test]
    fn test_part1() {
        // Arrange
        let lines = read_lines("../data/day13_data.txt")
            .unwrap();

        let mut track: HashMap<(usize, usize), char> = HashMap::new();
        let mut carts_locations: HashSet<(usize,usize)> = HashSet::new();
        let mut carts: VecDeque<Cart> = VecDeque::new();

        for (y, line) in lines.enumerate() {
            for (x, char) in line.unwrap().chars().enumerate() {
                match char {
                    ' ' | '-' | '|' => (),
                    '>' | '^' | 'v' | '<' => {
                        let cart = Cart::new((x,y),char);
                        carts.push_back(cart);
                        carts_locations.insert((x,y));
                    },
                    _ => {
                        track.insert((x,y), char);
                    }
                }
            }
        }

        // Act
        let found_crash: Option<(usize, usize)> = loop {
            let bar: Option<(usize, usize)> = match carts.pop_front() {
                Some(cart) => {
                        let movement = track.get(&cart.position)
                            .map(|c| sign_to_direction(c, &cart.direction, cart.turns))
                            .unwrap_or(Movement::new(cart.direction, cart.turns));
                        carts_locations.remove(&cart.position);
            
                        let new_card = cart.new_from_movement(movement);
                        let is_not_crashed = carts_locations.insert(new_card.position);

                        if !is_not_crashed {
                            Some(new_card.position)
                        } else {
                            carts.push_back(new_card);
                            None
                        }
                },
                None => break None,
            };
            if let Some(crash) = bar {
                break Some(crash);
            }
            continue;
        };       


        // Assert
        println!("Hej {:?}", found_crash)

    }

    struct Movement {
        direction: (i8, i8),
        turn: u32
    }

    impl Movement {
        fn new(direction: (i8,i8), turn: u32) -> Self {
            Self{direction, turn}
        }
    }

    fn sign_to_direction(c: &char, direction: &(i8, i8), turn: u32) -> Movement {        
        match (c, direction, turn % 3) {
            ('+', _, 0) => Movement::new((direction.1, -direction.0), turn+1),
            ('+', _, 1) => Movement::new((direction.0, direction.1), turn+1),
            ('+', _, 2) => Movement::new((-direction.1, direction.0), turn+1),
            ('/', (0,-1), _) | 
            ('\\', (1,0), _) |
            ('/', (0,1), _) |
            ('\\', (-1,0), _) => Movement::new((-direction.1, direction.0), turn),
            ('/', (-1,0), _) | 
            ('\\', (0,1), _) |
            ('/', (1,0), _) |
            ('\\', (0,-1), _) => Movement::new((direction.1, -direction.0), turn),
            _ => todo!()
        }
    }

    fn cart_to_direction(c: char) -> (i8, i8) {
        match c {
            '>' => (1,0),
            'v' => (0,1),
            '^' => (0,-1),
            '<' => (-1,0),
            _ => todo!()
        }
    }

    struct Cart {
        position: (usize, usize),
        direction: (i8, i8),
        turns: u32
    }

    impl Cart {
        fn new(position: (usize, usize), c: char) -> Self {
            let direction = cart_to_direction(c);
            return Self { position, direction, turns: 0 }
        }

        fn new_from_movement(self, movement: Movement) -> Cart {
            let foo = movement.direction.1 as i32;
            let bar = movement.direction.0 as i32;
            let position = (self.position.0 as i32 + movement.direction.0 as i32, self.position.1 as i32 + movement.direction.1 as i32);
            return Cart{
                direction: movement.direction,
                position: (position.0 as usize, position.1 as usize),
                turns: movement.turn
            }
        }
    }

    fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path> {
            let file = File::open(path)?;
            return Ok(io::BufReader::new(file).lines())
        }

}