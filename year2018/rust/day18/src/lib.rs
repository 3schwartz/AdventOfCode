use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, BTreeSet};

pub struct LumberCollection(BTreeMap<Coord, Acre>);

impl LumberCollection {
    pub fn from(input: &str) -> Result<Self> {
        let mut map = BTreeMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let acre = match c {
                    '.' => Acre::Open,
                    '#' => Acre::Lumberyard,
                    '|' => Acre::Trees,
                    _ => Err(anyhow!("{} not known at ({},{})", c, x, y))?,
                };
                map.insert(Coord::new(x, y), acre);
            }
        }
        Ok(LumberCollection(map))
    }

    pub fn find_resources_after_one_billion(&self) -> usize {
        let mut map = self.0.clone();
        let mut idx = 0;
        let mut cache = BTreeMap::new();
        cache.insert(map.clone(), idx);
        let first: i32;
        loop {
            idx += 1;
            map = self.update_map(map);

            if let Some(former) = cache.insert(map.clone(), idx) {
                first = former;
                break;
            }
        }
        let time = 1_000_000_000;
        let diff = idx - first;
        let remainder = (time - idx) % diff;
        for _ in 0..remainder {
            map = self.update_map(map);
        }
        self.get_resource_value(&map)
    }

    pub fn find_resource_after_iterations(&self, iterations: usize, debug: usize) -> usize {
        let mut map = self.0.clone();
        for _ in 0..iterations {
            map = self.update_map(map);
        }

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

        self.get_resource_value(&map)
    }

    fn update_map(&self, map: BTreeMap<Coord, Acre>) -> BTreeMap<Coord, Acre> {
        let mut new_map = BTreeMap::new();
        for (c, a) in &map {
            let next = a.next(c, &map);
            new_map.insert(c.clone(), next);
        }
        new_map
    }

    pub fn find_resource_after_iterations_using_sumple(&self, iterations: usize) -> usize {
        let mut map = self.0.clone();
        for _ in 0..iterations {
            map = self.update_map_using_simple(map);
        }    

        self.get_resource_value(&map)
    }

    fn update_map_using_simple(&self, map: BTreeMap<Coord, Acre>) -> BTreeMap<Coord, Acre> {
        let mut new_map = BTreeMap::new();
        for (c, a) in &map {
            let next = a.next_simple(c, &map);
            new_map.insert(c.clone(), next);
        }
        new_map
    }

    fn get_resource_value(&self, map: &BTreeMap<Coord, Acre>) -> usize {
        let trees = map.iter().filter(|(_, a)| *a == &Acre::Trees).count();
        let lumberyards = map.iter().filter(|(_, a)| *a == &Acre::Lumberyard).count();
        trees * lumberyards
    }
}

#[derive(Eq, PartialOrd, Ord, PartialEq, Clone, Debug)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

impl Acre {

    fn next_simple(&self, coord: &Coord, map: &BTreeMap<Coord, Acre>) -> Acre {
        match self {
            Acre::Open => {
                let trees = coord.neighbors_of(Acre::Trees, &map);
                if trees >= 3 {
                    return Acre::Trees;
                }
                return Acre::Open;
            }
            Acre::Trees => {
                let lumberyard = coord.neighbors_of(Acre::Lumberyard, &map);
                if lumberyard >= 3 {
                    return Acre::Lumberyard;
                }
                return Acre::Trees;
            }
            Acre::Lumberyard => {
                let lumberyard = coord.neighbors_of(Acre::Lumberyard, &map);
                let trees = coord.neighbors_of(Acre::Trees, &map);
                if lumberyard > 0 && trees > 0 {
                    return Acre::Lumberyard;
                }
                return Acre::Open;
            }
        }
    }

    fn next(&self, coord: &Coord, map: &BTreeMap<Coord, Acre>) -> Acre {
        let neighbors = coord.neighbors();
        let neighbor_types = self.neighbors(neighbors, map);
        match self {
            Acre::Open => {
                let trees = neighbor_types.iter().filter(|&s| s == &Acre::Trees).count();
                if trees >= 3 {
                    return Acre::Trees;
                }
                return Acre::Open;
            }
            Acre::Trees => {
                let lumberyard = neighbor_types
                    .iter()
                    .filter(|&s| s == &Acre::Lumberyard)
                    .count();
                if lumberyard >= 3 {
                    return Acre::Lumberyard;
                }
                return Acre::Trees;
            }
            Acre::Lumberyard => {
                let lumberyard = neighbor_types
                    .iter()
                    .filter(|&s| s == &Acre::Lumberyard)
                    .count();
                let trees = neighbor_types.iter().filter(|&s| s == &Acre::Trees).count();
                if lumberyard > 0 && trees > 0 {
                    return Acre::Lumberyard;
                }
                return Acre::Open;
            }
        }
    }

    fn neighbors(&self, neighbors: Vec<Coord>, map: &BTreeMap<Coord, Acre>) -> Vec<Acre> {
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

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbors_of(&self, acre: Acre, map: &BTreeMap<Coord, Acre>) -> usize {
        let mut seen = BTreeSet::new();
        let mut total = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                let x_n = if x < 0 {
                    self.x.saturating_sub(1)
                } else {
                    self.x.saturating_add(x as usize)
                };
                let y_n = if y < 0 {
                    self.y.saturating_sub(1)
                } else {
                    self.y.saturating_add(y as usize)
                };
                if x_n == self.x && y_n == self.y {
                    continue;
                }

                let n = Coord::new(x_n, y_n);
                if !seen.insert(n.clone()) {
                    continue;
                }
                
                let Some(adjacent) = map.get(&n) else { continue; };
                if *adjacent == acre {
                    total += 1;
                }
            }
        }
        total
    }

    fn neighbors(&self) -> Vec<Coord> {
        let mut neighbors = BTreeSet::new();
        for x in -1..=1 {
            for y in -1..=1 {
                let x_n = if x < 0 {
                    self.x.saturating_sub(1)
                } else {
                    self.x.saturating_add(x as usize)
                };
                let y_n = if y < 0 {
                    self.y.saturating_sub(1)
                } else {
                    self.y.saturating_add(y as usize)
                };
                if x_n == self.x && y_n == self.y {
                    continue;
                }
                let n = Coord::new(x_n, y_n);
                neighbors.insert(n);
            }
        }

        neighbors.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let expected = 1147;
        let input = fs::read_to_string("../data/day18_test_data.txt")?;
        let map = LumberCollection::from(&input)?;

        // Act
        let part_1 = map.find_resource_after_iterations(10, 10);

        // Assert
        assert_eq!(expected, part_1);
        Ok(())
    }
}
