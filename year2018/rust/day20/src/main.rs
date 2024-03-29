use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    fs::{self, File},
    io::Write,
};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day20_data.txt")?;
    let debug = false;

    let area = Area::from(input.clone(), true)?;

    if debug {
        area.print_map()?;
    }

    let paths = area.find_shortest_paths();
    let max = Area::find_max(&paths);

    println!("Part 1: {}", max);

    let area = Area::from(input.clone(), false)?;
    let paths = area.find_shortest_paths();
    let above = Area::find_above(1_000, &paths);

    println!("Part 2: {}", above);

    // Simple
    let simple_paths = simple(input)?;

    let max_simple = *simple_paths.values().max().unwrap_or(&0);
    println!("Part 1 simple: {}", max_simple);

    let above_simple = simple_paths.values().filter(|&v| *v >= 1_000).count();
    println!("Part 2 simple: {}", above_simple);

    Ok(())
}

fn simple(input: String) -> Result<HashMap<(i32, i32), u32>> {
    let mut position = (0, 0);
    let mut previous = position;
    let mut debt = Vec::new();
    let mut steps: HashMap<(i32, i32), u32> = HashMap::from([((0, 0), 0)]);
    for c in input.chars() {
        match c {
            '^' | '$' => continue,
            '(' => debt.push(position),
            '|' => {
                position = *debt
                    .last()
                    .ok_or_else(|| anyhow!("should not be empty when getting last"))?;
            }
            ')' => {
                position = debt
                    .pop()
                    .ok_or_else(|| anyhow!("should not be empty when going up"))?;
            }
            'W' | 'N' | 'E' | 'S' => {
                position = shift(c, position)?;
                let previos_step = *steps.get(&previous).ok_or_else(|| {
                    anyhow!("previous: {:?}, should be in map: {:?}", previous, steps)
                })?;
                let steps_from_previous = previos_step + 1;

                let next = if let Some(earlier) = steps.get(&position) {
                    std::cmp::min(*earlier, steps_from_previous)
                } else {
                    steps_from_previous
                };
                steps.insert(position, next);
            }

            _ => return Err(anyhow!("unknown: {c}")),
        }
        previous = position;
    }

    Ok(steps)
}

fn shift(c: char, position: (i32, i32)) -> Result<(i32, i32)> {
    let shift = match c {
        'W' => (position.0 - 1, position.1),
        'N' => (position.0, position.1 + 1),
        'E' => (position.0 + 1, position.1),
        'S' => (position.0, position.1 - 1),
        _ => return Err(anyhow!("unknown: {c} for position: {:?}", position)),
    };
    Ok(shift)
}

enum Elem {
    Room,
    Door,
    You,
}

#[derive(Clone)]
enum Direction {
    W,
    N,
    E,
    S,
}

impl Direction {
    fn from(c: char) -> Result<Direction> {
        let direction = match c {
            'W' => Direction::W,
            'N' => Direction::N,
            'E' => Direction::E,
            'S' => Direction::S,
            _ => Err(anyhow!("not able to map direction from: {}", c))?,
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
        [Direction::W, Direction::N, Direction::E, Direction::S]
    }
}

enum Continuation {
    Break,
    Stop,
}

struct Area {
    map: HashMap<(i32, i32), Elem>,
}

impl Area {
    /// Creates a area.
    ///
    /// # Arguments
    /// * `without_detours` - Main route has detours and these can be found as empty options`|)`.
    ///    Hence when one only want to find the room with shortest path which passes through most rooms
    ///    one can avoid these.
    fn from(input: String, without_detours: bool) -> Result<Self> {
        let mut area = Self {
            map: HashMap::from([((0, 0), Elem::You)]),
        };
        area.create_map(
            &mut input.chars().enumerate().skip(1),
            0,
            (0, 0),
            without_detours,
        )?;
        Ok(area)
    }

    fn create_map(
        &mut self,
        iter: &mut (impl Iterator<Item = (usize, char)> + Clone),
        debt: u32,
        initial_position: (i32, i32),
        without_detours: bool,
    ) -> Result<Continuation> {
        let mut position = initial_position;

        while let Some(next) = iter.next() {
            match next.1 {
                '^' | '$' => continue,
                'W' | 'N' | 'E' | 'S' => {
                    let direction = Direction::from(next.1)?;
                    let door = direction.move_direction(position);
                    let room = direction.move_direction(door);
                    self.map.insert(door, Elem::Door);
                    self.map.insert(room, Elem::Room);
                    position = room;
                }
                ')' => return Ok(Continuation::Stop),
                '|' => return Ok(Continuation::Break),
                '(' => {
                    // When only looking at main route avoid options.
                    if without_detours {
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
                    }

                    let position_at_branch = position;
                    loop {
                        match self.create_map(iter, debt, position_at_branch, without_detours)? {
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

    fn find_shortest_paths(&self) -> HashMap<(i32, i32), u32> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([(0, (0, 0))]);
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
                match self.map.get(&neighbor) {
                    Some(elem) => match elem {
                        Elem::Room | Elem::You => (),
                        Elem::Door => {
                            let room = direction.move_direction(neighbor);
                            queue.push_back((steps + 1, room));
                        }
                    },
                    None => (),
                }
            }
        }
        paths
    }

    fn find_max(paths: &HashMap<(i32, i32), u32>) -> u32 {
        *paths.values().max().unwrap_or(&0)
    }

    fn find_above(threshold: u32, paths: &HashMap<(i32, i32), u32>) -> usize {
        paths.values().filter(|&v| v >= &threshold).count()
    }

    fn print_map(&self) -> Result<()> {
        let mut x_max = i32::MIN;
        let mut y_max = i32::MIN;
        let mut x_min = i32::MAX;
        let mut y_min = i32::MAX;
        for ((x, y), _) in &self.map {
            x_min = cmp::min(x_min, *x);
            x_max = cmp::max(x_max, *x);
            y_min = cmp::min(y_min, *y);
            y_max = cmp::max(y_max, *y);
        }

        let mut buffer = Vec::<u8>::new();
        for y in y_min - 1..=y_max + 1 {
            for x in x_min - 1..=x_max + 1 {
                match self.map.get(&(x, y)) {
                    Some(elm) => match elm {
                        Elem::You => {
                            print!("X");
                            buffer.push(b'X');
                        }
                        Elem::Room => {
                            print!(".");
                            buffer.push(b'.');
                        }
                        Elem::Door => {
                            print!("|");
                            buffer.push(b'|');
                        }
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_with_empty() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day20_test_data2.txt")?;

        // Act
        let area = Area::from(input, true)?;

        let paths = area.find_shortest_paths();
        let max = Area::find_max(&paths);

        // Assert
        assert_eq!(max, 18);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day20_test_data.txt")?;

        // Act
        let area = Area::from(input, true)?;

        let paths = area.find_shortest_paths();
        let max = Area::find_max(&paths);

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
        let area = Area::from(input, true)?;

        // Assert
        area.print_map()?;
        Ok(())
    }
}
