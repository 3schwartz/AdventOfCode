use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let (x_target, y_target) = (7, 782);
    let depth = 11820;

    let risk_level = get_risk_level(x_target, y_target, depth)?;

    println!("Part 1: {}", risk_level);

    let fastest_route = get_fastest_route(x_target, y_target, depth)?;

    println!("Part 2: {}", fastest_route);

    Ok(())
}

fn get_risk_level(x_target: usize, y_target: usize, depth: usize) -> Result<usize> {
    let mut geologic_index = vec![vec![0; x_target + 1]; y_target + 1];
    let mut erosion_levels = vec![vec![0; x_target + 1]; y_target + 1];

    let mut risk_level = 0;

    for y in 0..=y_target {
        for x in 0..=x_target {
            geologic_index[y][x] = get_geologic_index(x, y, x_target, y_target, &erosion_levels);
            erosion_levels[y][x] = get_erosion_level(x, y, depth, &geologic_index);

            let region_type = Type::from(erosion_levels[y][x])?;

            risk_level += region_type.get_risk_level();
        }
    }

    Ok(risk_level)
}

fn get_erosion_level(x: usize, y: usize, depth: usize, geologic_index: &Vec<Vec<usize>>) -> usize {
    (geologic_index[y][x] + depth) % 20183
}

fn get_geologic_index(
    x: usize,
    y: usize,
    x_target: usize,
    y_target: usize,
    erosion_levels: &Vec<Vec<usize>>,
) -> usize {
    if x == 0 && y == 0 {
        0
    } else if x == x_target && y == y_target {
        0
    } else if y == 0 {
        x * 16807
    } else if x == 0 {
        y * 48271
    } else {
        erosion_levels[y - 1][x] * erosion_levels[y][x - 1]
    }
}

enum Type {
    Rocky,
    Wet,
    Narrow,
}

impl Type {
    fn from(erosion_level: usize) -> Result<Type> {
        let result = match erosion_level % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => {
                return Err(anyhow!(
                    "not able to map erosion level to type: {}",
                    erosion_level
                ))
            }
        };
        Ok(result)
    }

    fn from_coord(
        x: usize,
        y: usize,
        erosion_levels: &mut ErosionLevelStorage,
        geologic_index: &mut GeologicIndexStorage,
    ) -> Result<Type> {
        let erosion_level = erosion_levels.get(x, y, geologic_index);
        Type::from(erosion_level)
    }

    fn get_risk_level(&self) -> usize {
        match self {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow => 2,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tools {
    ClimbingGear,
    Torch,
    Neither,
}

impl Tools {
    fn get_other(&self, region_type: Type) -> Option<Tools> {
        match (self, region_type) {
            (Tools::ClimbingGear, Type::Rocky) => Some(Tools::Torch),
            (Tools::Torch, Type::Rocky) => Some(Tools::ClimbingGear),
            (Tools::ClimbingGear, Type::Wet) => Some(Tools::Neither),
            (Tools::Neither, Type::Wet) => Some(Tools::ClimbingGear),
            (Tools::Torch, Type::Narrow) => Some(Tools::Neither),
            (Tools::Neither, Type::Narrow) => Some(Tools::Torch),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    x: usize,
    y: usize,
    tool: Tools,
}

impl State {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            tool: Tools::Torch,
        }
    }

    fn from_tool(&self, tool: Tools) -> Self {
        Self {
            x: self.x,
            y: self.y,
            tool,
        }
    }

    fn from_neighbors(&self, neighbor: (usize, usize)) -> Self {
        Self {
            x: neighbor.0,
            y: neighbor.1,
            tool: self.tool,
        }
    }

    fn get_neighbors(&self) -> Vec<(usize, usize)> {
        let neighbors = HashSet::from([
            (self.x, self.y.saturating_sub(1)),
            (self.x, self.y + 1),
            (self.x.saturating_sub(1), self.y),
            (self.x + 1, self.y),
        ]);
        neighbors
            .iter()
            .map(|v| *v)
            .collect::<Vec<(usize, usize)>>()
    }

    fn can_go(&self, region: Type) -> bool {
        match region {
            Type::Rocky => self.tool == Tools::ClimbingGear || self.tool == Tools::Torch,
            Type::Wet => self.tool == Tools::ClimbingGear || self.tool == Tools::Neither,
            Type::Narrow => self.tool == Tools::Torch || self.tool == Tools::Neither,
        }
    }
}

struct ErosionLevelStorage {
    depth: usize,
    erosion_levels: HashMap<(usize, usize), usize>,
}

impl ErosionLevelStorage {
    fn new(depth: usize) -> Self {
        Self {
            depth,
            erosion_levels: HashMap::new(),
        }
    }

    /// Recursive
    fn get(&mut self, x: usize, y: usize, geologic_index: &mut GeologicIndexStorage) -> usize {
        if let Some(level) = self.erosion_levels.get(&(x, y)) {
            return *level;
        }

        let index = geologic_index.get(x, y, self);

        let next = (index + self.depth) % 20183;

        self.erosion_levels.insert((x, y), next);
        next
    }
}

struct GeologicIndexStorage {
    x_target: usize,
    y_target: usize,
    geologic_index: HashMap<(usize, usize), usize>,
}

impl GeologicIndexStorage {
    fn new(x_target: usize, y_target: usize) -> Self {
        Self {
            x_target,
            y_target,
            geologic_index: HashMap::new(),
        }
    }

    /// Recursive
    fn get(&mut self, x: usize, y: usize, erosion_levels: &mut ErosionLevelStorage) -> usize {
        if let Some(index) = self.geologic_index.get(&(x, y)) {
            return *index;
        }

        let mapped = if x == 0 && y == 0 {
            0
        } else if x == self.x_target && y == self.y_target {
            0
        } else if y == 0 {
            x * 16807
        } else if x == 0 {
            y * 48271
        } else {
            erosion_levels.get(x, y - 1, self) * erosion_levels.get(x - 1, y, self)
        };
        self.geologic_index.insert((x, y), mapped);
        return mapped;
    }
}

fn get_fastest_route(x_target: usize, y_target: usize, depth: usize) -> Result<usize> {
    let mut queue = HashMap::from([(0, vec![State::new()])]);

    let mut steps = 0;
    let mut visited = HashSet::new();
    let mut erosion_levels = ErosionLevelStorage::new(depth);
    let mut geologic_index = GeologicIndexStorage::new(x_target, y_target);

    loop {
        if queue.is_empty() {
            return Err(anyhow!("not able to find best path"));
        }
        let Some(mut current) = queue.remove(&steps) else {
            steps+=1;
            continue;
        };

        while let Some(next) = current.pop() {
            if !visited.insert(next) {
                continue;
            }
            if next.x == x_target && next.y == y_target && next.tool == Tools::Torch {
                return Ok(steps);
            }

            let current_region =
                Type::from_coord(next.x, next.y, &mut erosion_levels, &mut geologic_index)?;

            if let Some(other) = next.tool.get_other(current_region) {
                let next_queue = queue.entry(steps + 7).or_insert_with(|| vec![]);

                let next_state = next.from_tool(other);
                next_queue.push(next_state);
            }

            for neighbor in next.get_neighbors() {
                let neighbor_region = Type::from_coord(
                    neighbor.0,
                    neighbor.1,
                    &mut erosion_levels,
                    &mut geologic_index,
                )?;

                if !next.can_go(neighbor_region) {
                    continue;
                }
                let next_queue = queue.entry(steps + 1).or_insert_with(|| vec![]);

                let next_state = next.from_neighbors(neighbor);
                next_queue.push(next_state);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let (x_target, y_target) = (10, 10);
        let depth = 510;
        let expected_risk_level = 114;

        // Act
        let risk_level = get_risk_level(x_target, y_target, depth)?;

        // Assert
        assert_eq!(risk_level, expected_risk_level);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let (x_target, y_target) = (10, 10);
        let depth = 510;
        let expected_fastest_route = 45;

        // Act
        let fastest_route = get_fastest_route(x_target, y_target, depth)?;

        // Assert
        assert_eq!(fastest_route, expected_fastest_route);

        Ok(())
    }
}
