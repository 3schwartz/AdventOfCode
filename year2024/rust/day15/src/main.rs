use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Write,
    str::FromStr,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day15_data.txt")?;

    let mut warehouse = Warehouse::from_str(&input)?;
    warehouse.invoke(false);
    let count = warehouse.gps_sum();

    println!("Part 1: {}", count);

    let mut warehouse = BigWarehouse::from_str(&input)?;
    warehouse.invoke(false);
    let count = warehouse.gps_sum();

    println!("Part 2: {}", count);
    Ok(())
}

struct BigWarehouse {
    grid: Vec<Vec<char>>,
    robot: (i32, i32),
    actions: String,
}

impl BigWarehouse {
    fn invoke(&mut self, debug: bool) {
        let mut debug = if debug {
            Some(File::create("output.txt").unwrap())
        } else {
            None
        };

        if let Some(ref mut file) = debug {
            self.print(file);
            write!(file, "{}", self.actions).unwrap();
        }

        for a in self.actions.chars() {
            let n = match a {
                '^' => (0, -1),
                'v' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                _ => panic!("Didn't expect movement: {}", a),
            };
            let mut levels = vec![];
            let positions = HashSet::from([self.robot]);
            let updated = self.update(positions, n, &mut levels);
            if !updated {
                if let Some(ref mut file) = debug {
                    writeln!(file, "Move {} (no update)", a).unwrap();
                    self.print(file);
                }
                continue;
            };

            let mut updates = HashMap::new();
            for level in levels {
                for coord in level {
                    let next = (coord.0 + n.0, coord.1 + n.1);
                    updates
                        .entry(next)
                        .or_insert_with(|| self.grid[coord.1 as usize][coord.0 as usize]);
                    updates.entry(coord).or_insert_with(|| '.');
                }
            }
            for (k, v) in updates {
                self.grid[k.1 as usize][k.0 as usize] = v;
            }

            let next = (self.robot.0 + n.0, self.robot.1 + n.1);
            self.robot = next;

            if let Some(ref mut file) = debug {
                writeln!(file, "Move {}", a).unwrap();
                self.print(file);
            }
        }
    }

    fn update(
        &self,
        positions: HashSet<(i32, i32)>,
        d: (i32, i32),
        levels: &mut Vec<HashSet<(i32, i32)>>,
    ) -> bool {
        let shifts: HashSet<(i32, i32)> =
            positions.iter().map(|p| (p.0 + d.0, p.1 + d.1)).collect();
        if shifts
            .iter()
            .any(|&(x, y)| self.grid[y as usize][x as usize] == '#')
        {
            return false;
        }
        levels.push(positions);

        if shifts
            .iter()
            .all(|&(x, y)| self.grid[y as usize][x as usize] == '.')
        {
            return true;
        }

        if d.0 != 0 {
            return self.update(shifts, d, levels);
        }

        let mut updates = HashSet::new();
        for shift in &shifts {
            if self.grid[shift.1 as usize][shift.0 as usize] == '[' {
                updates.insert((shift.0 + 1, shift.1));
                updates.insert(*shift);
            }
            if self.grid[shift.1 as usize][shift.0 as usize] == ']' {
                updates.insert((shift.0 - 1, shift.1));
                updates.insert(*shift);
            }
        }

        return self.update(updates, d, levels);
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

    fn print(&self, file: &mut File) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.robot == (x as i32, y as i32) {
                    write!(file, "{}", '@').unwrap();
                } else {
                    write!(file, "{}", self.grid[y][x]).unwrap();
                }
            }
            writeln!(file).unwrap();
        }
        writeln!(file).unwrap();
        writeln!(file).unwrap();
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
        assert_eq!(count, 618);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day15_test_data.txt")?;

        // Act
        let mut warehouse = BigWarehouse::from_str(&input)?;
        warehouse.invoke(true);
        let count = warehouse.gps_sum();

        // Assert
        assert_eq!(count, 9021);
        Ok(())
    }
}
