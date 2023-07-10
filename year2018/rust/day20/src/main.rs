use std::{collections::HashMap, fs::{self, File}, cmp, io::{Write}};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day20_data.txt")?;

    let mut map = HashMap::from([((0,0), Elem::You)]);
    create_map(
        &mut input.chars().enumerate().skip(1),
        0,
        (0,0), &mut map)?;

    print_map(&map)?;

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
                let door = move_direction(position, direction.clone());
                let room = move_direction(door, direction);
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
                // let mut idx = next.0;
                while let Some(n) = clone.next() {
                    if previous_debt < debt {
                        return Err(anyhow!("debt should not be lower"));
                    }
                    if n.1 == ')' && debt == previous_debt {
                        // idx = n.0;
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
                // Validate if it is optional
                if previous.is_some() && previous.unwrap() == '|' {
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

fn move_direction(coord: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::W => (coord.0 - 1, coord.1),
        Direction::N => (coord.0, coord.1 - 1),
        Direction::E => (coord.0 + 1, coord.1),
        Direction::S => (coord.0, coord.1 + 1),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day20_test_data.txt")?;

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