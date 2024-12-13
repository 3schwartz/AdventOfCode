use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day12_data.txt")?;

    let grid = Grid::new(&input);
    let gardens = grid.find_gardes();

    let total_price = gardens.iter().fold(0, |acc, g| acc + g.price());

    println!("Part 1: {}", total_price);

    let total_price = gardens.iter().fold(0, |acc, g| acc + g.price_sides());

    println!("Part 2: {}", total_price);

    Ok(())
}

struct Grid {
    grid: HashMap<(i32, i32), char>,
    x_max: i32,
    y_max: i32,
}

impl Grid {
    const N: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn new(input: &str) -> Grid {
        let mut grid: HashMap<(i32, i32), char> = HashMap::new();
        let mut x_max: i32 = 0;
        let mut y_max: i32 = 0;
        for (y, line) in input.lines().enumerate() {
            y_max = y as i32;
            for (x, c) in line.chars().enumerate() {
                grid.insert((x as i32, y as i32), c);
                x_max = x as i32;
            }
        }
        Self { grid, x_max, y_max }
    }

    fn neighbors() -> [(i32, i32); 4] {
        Grid::N
    }

    fn find_gardes(&self) -> Vec<Garden> {
        let mut visited = HashSet::new();
        let mut gardens = Vec::new();
        for x in 0..=self.x_max {
            for y in 0..=self.y_max {
                let coord = (x, y);
                let c = *self
                    .grid
                    .get(&coord)
                    .unwrap_or_else(|| panic!("{:?}", coord));
                if !visited.insert(coord) {
                    continue;
                }

                let mut garden = HashSet::from([coord]);
                let mut queue = vec![coord];
                while let Some(next) = queue.pop() {
                    for n_c in Grid::neighbors() {
                        let n = (next.0 + n_c.0, next.1 + n_c.1);
                        if garden.contains(&n) {
                            continue;
                        }
                        if !self.grid.contains_key(&n) {
                            continue;
                        }
                        let n_char = self.grid.get(&n).unwrap();
                        if *n_char != c {
                            continue;
                        }
                        visited.insert(n);
                        garden.insert(n);
                        queue.push(n);
                    }
                }
                gardens.push(Garden::new(garden));
            }
        }
        gardens
    }
}

struct Garden {
    g: HashSet<(i32, i32)>,
}

impl Garden {
    fn new(g: HashSet<(i32, i32)>) -> Self {
        Self { g }
    }

    fn price(&self) -> usize {
        self.g.len() * self.perimeter()
    }

    fn price_sides(&self) -> usize {
        self.g.len() * self.sides()
    }

    fn perimeter(&self) -> usize {
        let mut p = 0;
        for c in &self.g {
            for n_c in Grid::neighbors() {
                let n = (c.0 + n_c.0, c.1 + n_c.1);
                if !self.g.contains(&n) {
                    p += 1;
                }
            }
        }
        p
    }

    fn sides(&self) -> usize {
        let mut perimeters: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
        for c in &self.g {
            for n_c in Grid::neighbors() {
                let n = (c.0 + n_c.0, c.1 + n_c.1);
                if self.g.contains(&n) {
                    continue;
                }
                perimeters
                    .entry(n_c)
                    .and_modify(|e| {
                        e.insert(*c);
                    })
                    .or_insert(HashSet::from([*c]));
            }
        }

        let mut sides = 0;
        for perimeter in perimeters.values() {
            let mut seen = HashSet::new();
            for p in perimeter {
                if seen.contains(p) {
                    continue;
                }
                sides += 1;
                let mut queue = Vec::from([*p]);
                while let Some(next) = queue.pop() {
                    if !seen.insert(next) {
                        continue;
                    }
                    for n_c in Grid::neighbors() {
                        let n = (next.0 + n_c.0, next.1 + n_c.1);
                        if !perimeter.contains(&n) {
                            continue;
                        }
                        queue.push(n);
                    }
                }
            }
        }
        sides
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day12_test_data.txt")?;

        // Act
        let grid = Grid::new(&input);
        let gardens = grid.find_gardes();

        let total_price = gardens.iter().fold(0, |acc, g| acc + g.price());

        // Assert
        assert_eq!(total_price, 140);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day12_test_data.txt")?;

        // Act
        let grid = Grid::new(&input);
        let gardens = grid.find_gardes();

        let total_price = gardens.iter().fold(0, |acc, g| acc + g.price_sides());

        // Assert
        assert_eq!(total_price, 80);
        Ok(())
    }
}
