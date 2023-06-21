use anyhow::{anyhow, Result};
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day17_data.txt")?;

    let mut x_max = u32::MIN;
    let mut y_max = u32::MIN;

    let spring = Coordinate::new(500, 0);

    let mut map = HashSet::new();
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
            map.insert(coord);
        }
    }
    let initial = map.clone();
    let clay_size = map.len();

    // Flow
    loop {
        // println!("-------------------");
        // println!("-------------------");
        // println!("-------------------");
        // for y in 0..=y_max {
        //     for x in 494..=507 {
        //         match map.get(&Coordinate { x, y }) {
        //             Some(_) => print!("#"),
        //             None => print!("."),
        //         }
        //     }
        //     println!()
        // }
        let mut queue = VecDeque::from([(None, spring.clone(), spring.clone())]);
        let mut possibles = vec![];
        while let Some((previous, current)) = queue.pop_front() {
            if current.is_possible_location(&previous, &map) {
                possibles.push(current.clone());
                continue;
            }
            let down = current.get_down();
            if !map.contains(&down) && down.y <= y_max {
                queue.push_back((current, down));
                continue;
            }
            if !map.contains(&down) && current.y == y_max {
                continue;
            }

            for coord in current.get_sides() {
                if !map.contains(&coord) && coord.x < x_max + 2 && coord != previous {
                    queue.push_back((current.clone(), coord));
                }
            }
        }

        possibles.retain(|p| !p.can_go_lower(x_max, y_max, &map));
        if possibles.is_empty() {
            break;
        }

        let mut best = possibles
            .pop()
            .ok_or_else(|| anyhow!("should not be empty"))?;
        for possible in possibles {
            if possible.y > best.y
            // || possible.y == best.y && (possible.x).abs_diff(500) > best.x.abs_diff(500)
            {
                best = possible
            }
        }
        map.insert(best);
    }

    let mut queue = VecDeque::from([spring.clone()]);
    let mut visited = HashSet::new();
    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }
        let down = current.get_down();
        if !map.contains(&down) && down.y <= y_max {
            map.insert(down.clone());
            queue.push_back(down);
            continue;
        }
        if !map.contains(&down) && current.y == y_max {
            continue;
        }

        for coord in current.get_sides() {
            if !map.contains(&coord) && coord.x < x_max + 2 && !visited.contains(&coord) {
                queue.push_back(coord.clone());
                map.insert(coord);
            }
        }
    }

    println!();
    println!("{}", visited.len());
    println!("{}", map.len() - clay_size);
    // map.retain(|c| !initial.contains(c));
    // println!("-------------------");
    //     println!("-------------------");
    //     println!("-------------------");
    //     for y in 0..=y_max {
    //         for x in 494..=507 {
    //             match map.get(&Coordinate { x, y }) {
    //                 Some(_) => print!("#"),
    //                 None => print!("."),
    //             }
    //         }
    //         println!()
    //     }

    Ok(())
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
