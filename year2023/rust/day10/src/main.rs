use std::{collections::HashMap, fs};
use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day10_data.txt")?;
    let pipe_map = PipeMap::from(&input)?;

    let actual = pipe_map.get_steps_to_farthest_point()?;

    println!("Part 1: {}", actual);

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

    fn get_steps_to_farthest_point(&self) -> Result<u64> {
        let mut previous = self.animal;
        let mut next = self.get_next_from_animal()?;
        let mut steps = 1;
        loop {
            let next_c = self.get_pipe(next);
            if next_c == 'S' {
                break;
            }
            steps += 1;
            let temp = next;
            next = next.next(previous, &next_c)?;
            previous = temp;
        }

        Ok(steps / 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        for (file, expected) in [("test1", 4), ("test2", 8)] {
            // Arrange
            let input = fs::read_to_string(format!("../../data/day10_data_{file}.txt"))?;
            let pipe_map = PipeMap::from(&input)?;
            // Act
            let actual = pipe_map.get_steps_to_farthest_point()?;

            // Assert
            assert_eq!(expected, actual);
        }
        Ok(())
    }
}
