use anyhow::{anyhow, Result};
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day17_data.txt")?;

    let mut x_max = u32::MIN;
    let mut y_max = u32::MIN;

    let spring = Coordinate::new(500, 0);

    let mut map = HashMap::new();
    for line in input.lines() {
        let coords = Coordinate::from(line)?;
        println!("{}", line);
        for coord in coords {
            if coord.x > x_max {
                x_max = coord.x;
            }
            if coord.y > y_max {
                y_max = coord.y;
            }
            map.insert(coord, Ground::Clay);
        }
    }

    fill(spring, y_max, &mut map);

    let part_1 = map
        .iter()
        .filter(|(_, g)| match g {
            Ground::Clay => false,
            _ => true,
        })
        .count();

    println!("Part 1: {}", part_1);

    Ok(())
}

#[derive(PartialEq)]
enum Ground {
    Clay,
    Still,
    Flow,
}

fn fill(current: Coordinate, y_max: u32, map: &mut HashMap<Coordinate, Ground>) {
    if current.y >= y_max {
        return;
    }
    let down = current.get_down();

    if !map.contains_key(&down) {
        map.insert(down.clone(), Ground::Flow);
        fill(down.clone(), y_max, map);
    }

    match map.get(&down) {
        Some(ground) => match ground {
            Ground::Clay | Ground::Still => {
                let right = Coordinate::new(current.x + 1, current.y);
                if !map.contains_key(&right) {
                    map.insert(right.clone(), Ground::Flow);
                    fill(right, y_max, map);
                }
                let left = Coordinate::new(current.x.saturating_sub(1), current.y);
                if !map.contains_key(&left) {
                    map.insert(left.clone(), Ground::Flow);
                    fill(left, y_max, map);
                }
            }
            Ground::Flow => (),
        },
        None => (),
    }
    match map.get(&down) {
        Some(ground) => match ground {
            Ground::Clay | Ground::Still => {
                let right = Coordinate::new(current.x + 1, current.y);
                if !map.contains_key(&right) {
                    map.insert(right.clone(), Ground::Flow);
                    fill(right, y_max, map);
                }
                let left = Coordinate::new(current.x.saturating_sub(1), current.y);
                if !map.contains_key(&left) {
                    map.insert(left.clone(), Ground::Flow);
                    fill(left, y_max, map);
                }
            }
            Ground::Flow => (),
        },
        None => (),
    }

    if within_boundaries(&current, map) {
        fill_height(&current, map);
    }
}

fn fill_height(coord: &Coordinate, map: &mut HashMap<Coordinate, Ground>) {
    fill_side(coord, Action::Add, map);
    fill_side(coord, Action::Subtract, map);
}

fn fill_side(coord: &Coordinate, shift: Action, map: &mut HashMap<Coordinate, Ground>) {
    let mut current = coord.x;
    loop {
        let c = Coordinate::new(current, coord.y);
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

fn within_boundaries(coord: &Coordinate, map: &HashMap<Coordinate, Ground>) -> bool {
    has_walls(coord, Action::Add, map) && has_walls(coord, Action::Subtract, map)
}

enum Action {
    Add,
    Subtract,
}

fn has_walls(coord: &Coordinate, shift: Action, map: &HashMap<Coordinate, Ground>) -> bool {
    let mut current = coord.x;
    loop {
        let c = Coordinate::new(current, coord.y);
        match map.get(&c) {
            Some(ground) => match ground {
                Ground::Clay => return true,
                _ => (),
            },
            None => return false,
        }
        match shift {
            Action::Add => current += 1,
            Action::Subtract => current = current.saturating_sub(1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn can_go_lower(&self, x_max: u32, y_max: u32, map: &HashSet<Coordinate>) -> bool {
        let mut visited = HashSet::new();
        let mut queue = Vec::from([self.clone()]);
        while let Some(current) = queue.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }
            if current.y > self.y {
                return true;
            }
            for coord in current.get_surrounding() {
                if !map.contains(&coord) && coord.y < y_max && coord.x < x_max + 2 {
                    queue.push(coord);
                }
            }
        }
        false
    }

    fn is_possible_location(&self, previous: &Coordinate, map: &HashSet<Coordinate>) -> bool {
        let mut possible = 0;
        let surrounding = self.get_surrounding();
        for coord in &surrounding {
            possible += map.contains(&coord) as u8;
        }
        if possible == 3 {
            return true;
        }
        if possible == 2 && surrounding.iter().any(|s| s == previous) {
            return true;
        }
        return false;
    }

    fn get_sides(&self) -> [Coordinate; 2] {
        [
            Coordinate::new(self.x.saturating_sub(1), self.y),
            Coordinate::new(self.x + 1, self.y),
        ]
    }

    fn get_surrounding(&self) -> [Coordinate; 3] {
        [
            Coordinate::new(self.x.saturating_sub(1), self.y),
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x, self.y + 1),
        ]
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
