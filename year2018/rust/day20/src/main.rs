use std::{collections::{HashMap, HashSet, VecDeque}, fs::{self, File}, cmp, io::{Write}};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day20_data.txt")?;

    let mut map = HashMap::from([((0,0), Elem::You)]);
    create_map(
        &mut input.chars().enumerate().skip(1),
        0,
        (0,0), &mut map)?;

    print_map(&map)?;

    let paths = find_shortest_paths(&map);
    let max = find_max(&paths);

    println!("Part 1: {}", max);

    Ok(())
}

fn print_map(map: &HashMap<(i32,i32), Elem>) -> Result<()> {
    let mut x_max = i32::MIN;
    let mut y_max = i32::MIN;
    let mut x_min = i32::MAX;
    let mut y_min = i32::MAX;
    for ((x,y), _) in map {
        x_min = cmp::min(x_min, *x);
        x_max = cmp::max(x_max, *x);
        y_min = cmp::min(y_min,*y);
        y_max = cmp::max(y_max, *y);
    }

    let mut buffer = Vec::<u8>::new();
    for y in y_min-1..=y_max+1 {
        for x in x_min-1..=x_max+1 {
            match map.get(&(x,y)) {
                Some(elm) => match elm {
                    Elem::You => {
                        print!("X");
                        buffer.push(b'X');
                    }
                    Elem::Room => {
                        print!(".");
                        buffer.push(b'.');
                    },
                    Elem::Door => {
                        print!("|");
                        buffer.push(b'|');
                    },
                },
                None => {
                    print!("#");
                    buffer.push(b'#');
                }
            }
        }
        buffer.push(b'\n');
        println!()
    }
    let mut file = File::create("map.txt")?;
    file.write_all(&buffer)?;
    Ok(())
}

enum Elem {
    Room,
    Door,
    You
}

#[derive(Clone)]
enum Direction {
    W, N, E, S
}

impl Direction {
    fn from(c: char) -> Result<Direction> {
        let direction = match c {
            'W' => Direction::W,
            'N' => Direction::N,
            'E' => Direction::E,
            'S' => Direction::S,
            _ => Err(anyhow!("not able to map direction from: {}", c))?
        };
        Ok(direction)
    }

    fn move_direction(&self, coord: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::W => (coord.0 - 1, coord.1),
            Direction::N => (coord.0, coord.1 - 1),
            Direction::E => (coord.0 + 1, coord.1),
            Direction::S => (coord.0, coord.1 + 1),
        }
    }

    fn get_directions() -> [Direction; 4] {
        [
            Direction::W,
            Direction::N,
            Direction::E,
            Direction::S,
        ]
    }
}

enum Continuation {
    Break,
    Stop
}

fn create_map(
    iter: &mut (impl Iterator<Item = (usize, char)> + Clone),
    debt: u32,
    initial_position: (i32, i32),
    map: &mut HashMap<(i32, i32), Elem>,
) -> Result<Continuation> {
    let mut position = initial_position;


    while let Some(next) = iter.next() {
        match next.1 {
            '^' | '$' => continue,
            'W' | 'N' | 'E' | 'S' => {
                let direction = Direction::from(next.1)?;
                let door = direction.move_direction(position);
                let room = direction.move_direction(door);
                map.insert(door, Elem::Door);
                map.insert(room, Elem::Room);
                position = room;
            }
            ')' => return Ok(Continuation::Stop),
            '|' => return Ok(Continuation::Break),
            '(' => {
                let mut clone = iter.clone();
                let mut previous: Option<char> = None;
                let mut previous_debt = debt;
                let mut idx = next.0;
                while let Some(n) = clone.next() {
                    if previous_debt < debt {
                        return Err(anyhow!("debt should not be lower"));
                    }
                    if n.1 == ')' && debt == previous_debt {
                        idx = n.0;
                        break;
                    }
                    if n.1 == '(' {
                        previous_debt += 1;
                    }
                    if n.1 == ')' {
                        previous_debt -= 1;
                    }
                    previous = Some(n.1);
                }
                // Validate if it is optional and move iterator if
                if previous.is_some() && previous.unwrap() == '|' {
                    while let Some(continuing) = iter.next() {
                        if continuing.0 == idx {
                            break;
                        }
                    }
                    continue;
                }

                let position_at_branch = position;
                loop {
                    match create_map(iter, debt, position_at_branch, map)? {
                        Continuation::Break => (),
                        Continuation::Stop => break,
                    }
                }

            }
            _ => return Err(anyhow!("not able to map: {:?}", next)),
        }
    }
    Ok(Continuation::Stop)
}

fn find_shortest_paths(map: &HashMap<(i32, i32), Elem>) -> HashMap<(i32, i32), u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(0,(0,0))]);
    let mut paths = HashMap::new();

    while let Some(next) = queue.pop_front() {
        let steps = next.0;
        let position = next.1;
        if !visited.insert(position) {
            continue;
        }
        paths.insert(position, steps);

        for direction in Direction::get_directions() {
            let neighbor = direction.move_direction(position);
            match map.get(&neighbor) {
                Some(elem) => match elem {
                    Elem::Room | Elem::You => (),
                    Elem::Door => {
                        let room = direction.move_direction(neighbor);
                        queue.push_back((steps + 1, room));
                    },
                },
                None => (),
            }
        }
    }
    paths
}

fn find_max(paths: &HashMap<(i32, i32), u32> ) -> u32 {
    *paths.values().max().unwrap_or(&0)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_with_empty() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../data/day20_test_data2.txt")?;

        // Act
        let mut map = HashMap::from([((0,0), Elem::You)]);
        create_map(
            &mut input.chars().enumerate().skip(1),
            0,
            (0,0), &mut map)?;

        let paths = find_shortest_paths(&map);
        let max = find_max(&paths);
    
        // Assert
        assert_eq!(max, 18);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../data/day20_test_data.txt")?;

        // Act
        let mut map = HashMap::from([((0,0), Elem::You)]);
        create_map(
            &mut input.chars().enumerate().skip(1),
            0,
            (0,0), &mut map)?;

        let paths = find_shortest_paths(&map);
        let max = find_max(&paths);
    
        // Assert
        assert_eq!(max, 31);
        Ok(())
    }

    #[test]
    #[ignore = "Printing map"]
    fn test_part_1_map() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../data/day20_test_data2.txt")?;

        // Act
        let mut map = HashMap::from([((0,0), Elem::You)]);
        create_map(
            &mut input.chars().enumerate().skip(1),
            0,
            (0,0), &mut map)?;
    
        // Assert
        print_map(&map)?;
        Ok(())
    }
}