use std::{fs, collections::{HashMap, HashSet}};
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day18_data.txt")?;

    let map = create_map(&input)?;

    let part_1 = part_1(map, 50);

    println!("Part 1: {}", part_1);

    Ok(())
}

fn part_1(mut map: HashMap<Coord, Acre>, debug: usize) -> usize {
    let mut new_map = HashMap::new();
    for _ in 0..10 {
        for (c, a) in &map {
            let next = a.next(c, &map);
            new_map.insert(c.clone(), next);
        }
        map = new_map;
        new_map = HashMap::new();
    }

    let trees = map.iter().filter(|(_, a)| *a == &Acre::Trees).count();
    let lumberyards = map.iter().filter(|(_, a)| *a == &Acre::Lumberyard).count();

    if debug != 0 {
        for y in 0..=debug {
            for x in 0..=debug {
                match map.get(&Coord { x, y }) {
                    Some(n) => match n {
                        Acre::Open => print!("."),
                        Acre::Trees => print!("|"),
                        Acre::Lumberyard => print!("#"),
                    },
                    None => (),
                }
            }
            println!()
        }
    }

    trees * lumberyards
}

fn create_map(input: &str) -> Result<HashMap<Coord, Acre>> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let acre = match c {
                '.' => Acre::Open,
                '#' => Acre::Lumberyard,
                '|' => Acre::Trees,
                _ => Err(anyhow!("{} not known at ({},{})",c, x, y))?,
            };
            map.insert(Coord::new(x, y), acre);
        }
    }
    Ok(map)
}

#[derive(PartialEq, Clone, Debug)]
enum Acre {
    Open,
    Trees,
    Lumberyard
}

impl Acre {
    fn next(&self, coord: &Coord, map: &HashMap<Coord, Acre>) -> Acre {
        let neighbors = coord.neighbors();
        let neighbor_types = self.neighbors(neighbors, map);
        match self {
            Acre::Open => {
                let trees = neighbor_types.iter().filter(|&s| s == &Acre::Trees).count();
                if trees >= 3 {
                    return Acre::Trees;
                }
                return Acre::Open;
            },
            Acre::Trees => {
                let lumberyard = neighbor_types.iter().filter(|&s| s == &Acre::Lumberyard).count();
                if lumberyard >= 3 {
                    return Acre::Lumberyard;
                }
                return Acre::Trees;
            },
            Acre::Lumberyard => {
                let lumberyard = neighbor_types.iter().filter(|&s| s == &Acre::Lumberyard).count();
                let trees = neighbor_types.iter().filter(|&s| s == &Acre::Trees).count();
                // if coord.x == 0 && coord.y == 15 {
                //     println!("{:?}", neighbor_types);
                //     println!("{:?}", neighbors);
                // }
                if lumberyard > 0 && trees > 0 {
                    return Acre::Lumberyard;
                }
                return Acre::Open;
            },
        }


    }

    fn neighbors(&self, neighbors: Vec<Coord>, map: &HashMap<Coord, Acre>) -> Vec<Acre> {
        let mut types = vec![];
        for n in neighbors {
            match map.get(&n) {
                Some(next) => types.push(next.clone()),
                None => (),
            }
        }
        types
    }

    
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self) -> Vec<Coord> {
        let mut neighbors = HashSet::new();
        for x in -1..=1 {
            for y in -1..=1 {
                let x_n = if x < 0 { self.x.saturating_sub(1) } else { self.x.saturating_add(x as usize)};
                let y_n = if y < 0 { self.y.saturating_sub(1) } else { self.y.saturating_add(y as usize)};
                if x_n == self.x && y_n == self.y {
                    continue;
                }
                let n = Coord::new(x_n, y_n);
                neighbors.insert(n);
            }
        };
        // if self.x == 0 && self.y == 15 {
        //     println!("{:?}", neighbors)
        // }

        neighbors.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let expected = 1147;
        let input = fs::read_to_string("../data/day18_test_data.txt")?;
        let map = create_map(&input)?;

        // Act
        let part_1 = part_1(map, 10);

        // Assert
        assert_eq!(expected, part_1);
        Ok(())
    }

}