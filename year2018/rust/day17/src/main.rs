use anyhow::{anyhow, Result};
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day17_data.txt")?;

    let (y_min, y_max, mut map) = generate_map(input)?;

    let spring = Coordinate::new(500, 0);
    spring.fill(y_max, &mut map);

    let part_1 = get_water_count(y_min, y_max, &map);
    let part_2 = get_still_waiter_count(y_min, y_max, &map);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_still_waiter_count(y_min: u32, y_max: u32, map: &HashMap<Coordinate, Ground>) -> usize {
    map.iter()
        .filter(|(c, _)| c.y >= y_min && c.y <= y_max)
        .filter(|(_, g)| match g {
            Ground::Still => true,
            _ => false,
        })
        .count()
}

fn get_water_count(y_min: u32, y_max: u32, map: &HashMap<Coordinate, Ground>) -> usize {
    map.iter()
        .filter(|(c, _)| c.y >= y_min && c.y <= y_max)
        .filter(|(_, g)| match g {
            Ground::Clay => false,
            _ => true,
        })
        .count()
}

fn generate_map(input: String) -> Result<(u32, u32, HashMap<Coordinate, Ground>)> {
    let mut y_min = u32::MAX;
    let mut y_max = u32::MIN;

    let mut map = HashMap::new();
    for line in input.lines() {
        let coords = Coordinate::from(line)?;
        for coord in coords {
            y_max = std::cmp::max(y_max, coord.y);
            y_min = std::cmp::min(y_min, coord.y);

            map.insert(coord, Ground::Clay);
        }
    }
    Ok((y_min, y_max, map))
}

#[derive(PartialEq)]
enum Ground {
    Clay,
    Still,
    Flow,
}

enum Action {
    Add,
    Subtract,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn fill_height(&self, map: &mut HashMap<Coordinate, Ground>) {
        self.fill_side(Action::Add, map);
        self.fill_side(Action::Subtract, map);
    }

    fn fill_side(&self, shift: Action, map: &mut HashMap<Coordinate, Ground>) {
        let mut current = self.x;
        loop {
            let c = Coordinate::new(current, self.y);
            if let Some(ground) = map.get(&c) {
                if ground == &Ground::Clay {
                    return;
                }
            }
            map.insert(c, Ground::Still);
            match shift {
                Action::Add => current += 1,
                Action::Subtract => current = current.saturating_sub(1),
            }
        }
    }

    fn within_boundaries(&self, map: &HashMap<Coordinate, Ground>) -> bool {
        self.has_walls(Action::Add, map) && self.has_walls(Action::Subtract, map)
    }

    fn has_walls(&self, shift: Action, map: &HashMap<Coordinate, Ground>) -> bool {
        let mut current = self.x;
        loop {
            let c = Coordinate::new(current, self.y);

            let Some(ground) = map.get(&c) else { return false; };

            match ground {
                Ground::Clay => return true,
                _ => (),
            };

            match shift {
                Action::Add => current += 1,
                Action::Subtract => current = current.saturating_sub(1),
            }
        }
    }

    fn fill(&self, y_max: u32, map: &mut HashMap<Coordinate, Ground>) {
        if self.y >= y_max {
            return;
        }

        let down = self.get_down();

        // Go down if nothing
        if !map.contains_key(&down) {
            map.insert(down.clone(), Ground::Flow);
            // Will end in clay or still if in boundary
            down.fill(y_max, map);
        }

        // There was something, go to sides.
        let Some(ground) = map.get(&down) else { return; };
        match ground {
            // If going down ended in clay or still then go to sides
            Ground::Clay | Ground::Still => {
                let right = self.get_right();
                if !map.contains_key(&right) {
                    map.insert(right.clone(), Ground::Flow);
                    right.fill(y_max, map);
                }
                let left = self.get_left();
                if !map.contains_key(&left) {
                    map.insert(left.clone(), Ground::Flow);
                    left.fill(y_max, map);
                }
            }
            // Didn't end in boundary
            Ground::Flow => (),
        }

        if self.within_boundaries(map) {
            self.fill_height(map);
        }
    }

    fn get_left(&self) -> Coordinate {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn get_right(&self) -> Coordinate {
        Self {
            x: self.x.saturating_sub(1),
            y: self.y,
        }
    }

    fn get_down(&self) -> Coordinate {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn from(input: &str) -> Result<Vec<Self>> {
        let parts: Vec<&str> = input.split(", ").collect();
        if parts.len() != 2 {
            return Err(anyhow!("not able to map coord: {}", input));
        }

        let (xs, ys) = if parts[0].starts_with("x") {
            (parts[0], parts[1])
        } else {
            (parts[1], parts[0])
        };
        let x_range = Coordinate::map_element(xs)?;
        let y_range = Coordinate::map_element(ys)?;

        let mut coords = Vec::with_capacity(x_range.len() * y_range.len());
        for x in x_range {
            for y in &y_range {
                coords.push(Coordinate::new(x, *y))
            }
        }
        Ok(coords)
    }

    fn map_element(input: &str) -> Result<Vec<u32>> {
        let coord: Vec<&str> = input.split("=").collect();
        if coord.len() != 2 {
            return Err(anyhow!("not able to map elm: {}", input));
        }

        let range: Vec<&str> = coord[1].split("..").collect();
        if range.len() == 1 {
            return Ok(vec![coord[1].parse()?]);
        };

        if range.len() != 2 {
            return Err(anyhow!("not able to map range: {}", input));
        }
        let from = range[0].parse()?;
        let to = range[1].parse()?;
        let mut from_to = Vec::with_capacity((to - from) as usize + 1);
        for i in from..=to {
            from_to.push(i);
        }

        Ok(from_to)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day17_test_data.txt")?;
        let expected = 57;

        let (y_min, y_max, mut map) = generate_map(input)?;

        // Act
        let spring = Coordinate::new(500, 0);
        spring.fill(y_max, &mut map);

        let water = get_water_count(y_min, y_max, &map);

        // Assert
        assert_eq!(water, expected);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day17_test_data.txt")?;
        let expected = 29;

        let (y_min, y_max, mut map) = generate_map(input)?;

        // Act
        let spring = Coordinate::new(500, 0);
        spring.fill(y_max, &mut map);

        let still = get_still_waiter_count(y_min, y_max, &map);

        // Assert
        assert_eq!(still, expected);
        Ok(())
    }
}
