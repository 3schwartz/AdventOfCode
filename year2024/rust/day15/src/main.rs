use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day15_data.txt")?;

    let mut warehouse = Warehouse::from_str(&input)?;
    warehouse.invoke(false);
    let count = warehouse.gps_sum();

    println!("Part 1: {}", count);
    Ok(())
}

struct BigWarehouse {
    grid: Vec<Vec<char>>,
    robot: (i32, i32),
    actions: String,
}

impl BigWarehouse {
    fn invoke(&mut self, debug: bool) {
        if debug {
            self.print();
            println!("{}", self.actions);
        }

        for a in self.actions.chars() {
            let n = match a {
                '^' => (0, -1),
                'v' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                _ => panic!("Didn't expect movement: {}", a),
            };
            let mut levels = HashMap::new();
            let positions = HashSet::from([self.robot]);
            let updated = self.update(positions, n, 0, &mut levels);
            if !updated {
                continue;
            };

            let mut updates = HashMap::new();
            for (level, u) in levels {
                for u_ in u {
                    let p = (u_.0, u_.1);
                    let next = (p.0 + n.0, p.1 + n.1);
                    let next_c = self.grid[p.1 as usize][p.0 as usize];
                    updates.insert(next, next_c);
                }
            }
            for (k, v) in updates {
                self.grid[k.1 as usize][k.0 as usize] = v;
            }

            let next = (self.robot.0 + n.0, self.robot.1 + n.1);
            self.robot = next;

            if debug {
                println!("Move {}", a);
                self.print();
            }
        }
    }

    fn update(
        &self,
        positions: HashSet<(i32, i32)>,
        d: (i32, i32),
        level: u32,
        levels: &mut HashMap<u32, HashSet<(i32, i32)>>,
    ) -> bool {
        let mut shifts: HashSet<(i32, i32)> =
            positions.iter().map(|p| (p.0 + d.0, p.1 + d.1)).collect();
        if shifts
            .iter()
            .any(|&(x, y)| self.grid[y as usize][x as usize] == '#')
        {
            return false;
        }
        if shifts
            .iter()
            .all(|&(x, y)| self.grid[y as usize][x as usize] == '.')
        {
            levels.insert(level, positions);
            return true;
        }

        if d.0 != 0 {
            levels.insert(level, positions);
            return self.update(shifts, d, level + 1, levels);
        }

        let mut updates = HashSet::new();
        for shift in &shifts {
            if self.grid[shift.1 as usize][shift.0 as usize] == '[' {
                updates.insert((shift.0 + 1, shift.1));
            }
            if self.grid[shift.1 as usize][shift.0 as usize] == ']' {
                updates.insert((shift.0 - 1, shift.1));
            }
        }
        for update in updates {
            shifts.insert(update);
        }

        return self.update(shifts, d, level + 1, levels);
    }

    fn gps_sum(&self) -> usize {
        let mut gps_sum = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.grid[y][x] == '[' {
                    gps_sum += 100 * y + x
                }
            }
        }
        gps_sum
    }

    fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.robot == (x as i32, y as i32) {
                    print!("{}", '@');
                } else {
                    print!("{}", self.grid[y][x])
                }
            }
            println!();
        }
        println!();
        println!();
    }
}

impl FromStr for BigWarehouse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<&str>>();
        assert_eq!(2, parts.len());

        let mut grid = vec![];
        let mut robot = None;
        for (y, line) in parts[0].lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                let element = match c {
                    '@' => {
                        robot = Some((2 * x as i32, y as i32));
                        ['.', '.']
                    }
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '#' => ['#', '#'],
                    _ => panic!("Unexpected element: {}", c),
                };
                for e in element {
                    row.push(e);
                }
            }
            grid.push(row);
        }
        assert!(robot.is_some());

        let actions = parts[1].lines().collect();

        Ok(Self {
            grid,
            robot: robot.unwrap(),
            actions,
        })
    }
}

struct Warehouse {
    grid: Vec<Vec<char>>,
    robot: (i32, i32),
    actions: String,
}

impl Warehouse {
    fn invoke(&mut self, debug: bool) {
        if debug {
            self.print();
            println!("{}", self.actions);
        }

        for a in self.actions.chars() {
            let n = match a {
                '^' => (0, -1),
                'v' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                _ => panic!("Didn't expect movement: {}", a),
            };
            let mut updates = vec![];
            let updated = self.update(self.robot, n, &mut updates);
            if !updated {
                continue;
            };
            let next = (self.robot.0 + n.0, self.robot.1 + n.1);
            for update in updates {
                if update == next {
                    continue;
                }
                self.grid[update.1 as usize][update.0 as usize] = 'O';
            }
            self.grid[next.1 as usize][next.0 as usize] = '.';
            self.robot = next;

            if debug {
                println!("Move {}", a);
                self.print();
            }
        }
    }

    fn gps_sum(&self) -> usize {
        let mut gps_sum = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.grid[y][x] == 'O' {
                    gps_sum += 100 * y + x
                }
            }
        }
        gps_sum
    }

    fn update(
        &self,
        position: (i32, i32),
        direction: (i32, i32),
        updates: &mut Vec<(i32, i32)>,
    ) -> bool {
        let next = (position.0 + direction.0, position.1 + direction.1);
        let element = self.grid[next.1 as usize][next.0 as usize];
        match element {
            '.' => {
                updates.push(next);
                true
            }
            'O' => {
                updates.push(next);
                self.update(next, direction, updates)
            }
            '#' => false,
            _ => panic!("Didn't expect element: {}", element),
        }
    }

    fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.robot == (x as i32, y as i32) {
                    print!("{}", '@');
                } else {
                    print!("{}", self.grid[y][x])
                }
            }
            println!();
        }
        println!();
        println!();
    }
}

impl FromStr for Warehouse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<&str>>();
        assert_eq!(2, parts.len());

        let mut grid = vec![];
        let mut robot = None;
        for (y, line) in parts[0].lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                let element = match c {
                    '@' => {
                        robot = Some((x as i32, y as i32));
                        '.'
                    }
                    e => e,
                };
                row.push(element);
            }
            grid.push(row);
        }
        assert!(robot.is_some());

        let actions = parts[1].lines().collect();

        Ok(Self {
            grid,
            robot: robot.unwrap(),
            actions,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day15_test_data.txt")?;

        // Act
        let mut warehouse = Warehouse::from_str(&input)?;
        warehouse.invoke(false);
        let count = warehouse.gps_sum();

        // Assert
        assert_eq!(count, 10092);
        Ok(())
    }

    #[test]
    fn test_part_1_v2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day15_test2_data.txt")?;

        // Act
        let mut warehouse = Warehouse::from_str(&input)?;
        warehouse.invoke(true);
        let count = warehouse.gps_sum();

        // Assert
        assert_eq!(count, 2028);
        Ok(())
    }

    #[test]
    fn test_part_2_v2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day15_test3_data.txt")?;

        // Act
        let mut warehouse = BigWarehouse::from_str(&input)?;
        warehouse.invoke(true);
        let count = warehouse.gps_sum();

        // Assert
        assert_eq!(count, 2028);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day15_test_data.txt")?;

        // Act
        let mut warehouse = BigWarehouse::from_str(&input)?;
        warehouse.invoke(false);
        let count = warehouse.gps_sum();

        // Assert
        assert_eq!(count, 9021);
        Ok(())
    }
}
