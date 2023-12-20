use anyhow::{anyhow, Ok, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day10_data.txt")?;
    let pipe_map = PipeMap::from(&input)?;

    let actual = pipe_map.get_max_distance()?;

    println!("Part 1: {}", actual);

    let part_2 = pipe_map.get_inside()?;

    println!("Part 2: {}", part_2);

    Ok(())
}

struct PipeMap {
    animal: Coord,
    map: HashMap<Coord, char>,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    fn west(&self) -> Self {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn east(&self) -> Self {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn south(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn north(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn next(&self, previous: Coord, pipe: &char) -> Result<Coord> {
        let n = match pipe {
            '|' => {
                if previous == self.north() {
                    self.south()
                } else {
                    self.north()
                }
            }
            '-' => {
                if previous == self.east() {
                    self.west()
                } else {
                    self.east()
                }
            }
            'L' => {
                if previous == self.north() {
                    self.east()
                } else {
                    self.north()
                }
            }
            'J' => {
                if previous == self.north() {
                    self.west()
                } else {
                    self.north()
                }
            }
            '7' => {
                if previous == self.south() {
                    self.west()
                } else {
                    self.south()
                }
            }
            'F' => {
                if previous == self.south() {
                    self.east()
                } else {
                    self.south()
                }
            }
            _ => return Err(anyhow!("{} not known for {:?}", pipe, self)),
        };
        Ok(n)
    }
}

impl PipeMap {
    fn from(input: &str) -> Result<PipeMap> {
        let mut animal_option = None;
        let mut map = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                let coord = Coord::new(x as i32, y as i32);
                if c == 'S' {
                    animal_option = Some(coord)
                }
                map.insert(coord, c);
            }
        }
        if let Some(animal) = animal_option {
            return Ok(Self { animal, map });
        }
        Err(anyhow!("not able to find animal in map"))
    }

    fn get_pipe(&self, coord: Coord) -> char {
        *self.map.get(&coord).unwrap_or(&'.')
    }

    fn get_next_from_animal(&self) -> Result<Coord> {
        let north = self.animal.north();
        let north_pipe = self.get_pipe(north);
        if ['|', '7', 'F'].contains(&north_pipe) {
            return Ok(north);
        }

        let south = self.animal.south();
        let south_pipe = self.get_pipe(south);
        if ['|', 'L', 'J'].contains(&south_pipe) {
            return Ok(south);
        }

        let west = self.animal.west();
        let west_pipe = self.get_pipe(west);
        if ['-', '7', 'J'].contains(&west_pipe) {
            return Ok(west);
        }

        let east = self.animal.east();
        let east_pipe = self.get_pipe(east);
        if ['-', 'L', 'F'].contains(&east_pipe) {
            return Ok(east);
        }

        Err(anyhow!("not match for {:?}", self.animal))
    }

    fn get_pipe_loop(&self) -> Result<HashSet<Coord>> {
        let mut previous = self.animal;
        let mut next = self.get_next_from_animal()?;
        let mut set = HashSet::from([next]);
        loop {
            let next_c = self.get_pipe(next);
            if next_c == 'S' {
                break;
            }
            let temp = next;
            next = next.next(previous, &next_c)?;
            previous = temp;
            set.insert(next);
        }
        Ok(set)
    }

    fn get_max_distance(&self) -> Result<u64> {
        let set = self.get_pipe_loop()?;
        Ok(set.len() as u64 / 2)
    }

    /// To validate if a point is within an enclosing shape the
    /// count of borders it crosses in any direction needs to be an odd number.
    ///
    /// Now since we also needs to handle corners following logic is used.
    /// If a '7', or 'L' is meet then a 'F', or 'J' needs to come (or '|', '-') for the ppint
    /// to be within the shape. This because the points can "followed" around the line of
    /// '7' and 'L' and a  'F', or 'J' will "stop" this.
    fn get_inside(&self) -> Result<u64> {
        let set = self.get_pipe_loop()?;
        let max_x = self
            .map
            .keys()
            .map(|c| c.x)
            .max()
            .ok_or_else(|| anyhow!("not able to find max x"))?;
        let max_y = self
            .map
            .keys()
            .map(|c| c.y)
            .max()
            .ok_or_else(|| anyhow!("not able to find max y"))?;
        let mut inside = 0;
        for y in 0..=max_y {
            for x in 0..=max_x {
                if set.contains(&Coord::new(x, y)) {
                    continue;
                }
                let mut xt = x;
                let mut yt = y;
                let mut borders = 0;
                while xt <= max_x && yt <= max_y {
                    let ct = Coord::new(xt, yt);
                    let c = self.get_pipe(ct);
                    if set.contains(&ct) && c != '7' && c != 'L' {
                        borders += 1;
                    }
                    xt += 1;
                    yt += 1;
                }
                if borders % 2 == 1 {
                    inside += 1;
                }
            }
        }
        Ok(inside)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() -> Result<()> {
        for (file, expected) in [("test3", 8), ("test4", 10)] {
            // Arrange
            let input = fs::read_to_string(format!("../../data/day10_data_{file}.txt"))?;
            let pipe_map = PipeMap::from(&input)?;
            // Act
            let actual = pipe_map.get_inside()?;

            // Assert
            assert_eq!(expected, actual);
        }
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        for (file, expected) in [("test1", 4), ("test2", 8)] {
            // Arrange
            let input = fs::read_to_string(format!("../../data/day10_data_{file}.txt"))?;
            let pipe_map = PipeMap::from(&input)?;
            // Act
            let actual = pipe_map.get_max_distance()?;

            // Assert
            assert_eq!(expected, actual);
        }
        Ok(())
    }
}
