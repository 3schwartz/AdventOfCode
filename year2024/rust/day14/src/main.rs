use anyhow::Result;

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::{
    collections::BTreeMap,
    fs::{self},
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day14_data.txt")?;

    println!("Count: {}", input.lines().count());

    let hall = Hall::make_map(&input, 101, 103);
    println!("Map made");

    let updated = hall.rotate(100);
    let count = updated.quandrant_count();

    println!("Part 1: {}", count);

    let _ = hall.rotate_with_print(10_000, 300);

    Ok(())
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Robot {
    x: i32,
    y: i32,
}

impl Robot {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Hall {
    map: BTreeMap<(i32, i32), Vec<Robot>>,
    x_size: i32,
    y_size: i32,
}

impl Hall {
    const N: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn quandrant_count(&self) -> usize {
        let mut x_to = self.x_size / 2;
        let x_from = self.x_size / 2 + 1;
        if self.x_size % 2 == 0 {
            x_to -= 1;
        }
        let mut y_to = self.y_size / 2;
        let y_from = self.y_size / 2 + 1;
        if self.y_size % 2 == 0 {
            y_to -= 1;
        }

        let quadrants = [
            ((0, x_to), (0, y_to)),
            ((x_from, self.x_size), (0, y_to)),
            ((0, x_to), (y_from, self.y_size)),
            ((x_from, self.x_size), (y_from, self.y_size)),
        ];
        let mut counts = [0; 4];
        for (i, q) in quadrants.iter().enumerate() {
            let mut q_count = 0;
            for x in q.0 .0..q.0 .1 {
                for y in q.1 .0..q.1 .1 {
                    q_count += self.map.get(&(x, y)).map(|v| v.len()).unwrap_or(0);
                }
            }
            counts[i] = q_count;
        }
        counts.iter().product::<usize>()
    }

    #[cfg(test)]
    fn rotate_memo(&self, rotations: usize) -> Self {
        let mut current = self.clone();
        let mut seen = BTreeMap::new();
        let mut r = 0;
        let mut has_jumped = false;
        loop {
            if r == rotations {
                break;
            }
            if has_jumped {
                current = current.update_map();
            } else if let Some(step) = seen.get(&current) {
                let jump = r - step;
                let inc = (rotations - r) / jump;
                r += jump * inc;
                has_jumped = true;
                continue;
            } else {
                seen.insert(current.clone(), r);
                current = current.update_map();
            }

            r += 1;
        }
        for _ in 0..rotations {
            if seen.contains_key(&current) {}
        }
        current
    }

    fn rotate_with_print(&self, rotations: usize, area_threshold: u32) -> Self {
        let mut current = self.clone();
        let mut file = File::create("output.txt").unwrap();
        for r in 0..rotations {
            current = current.update_map();
            if current.areas() < area_threshold {
                writeln!(file, "Rotations: {}", r + 1).unwrap();
                current.print(&mut file);
            }
        }
        current
    }

    fn areas(&self) -> u32 {
        let mut seen = HashSet::new();
        let mut areas = 0;
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                if !self.map.contains_key(&(x, y)) {
                    continue;
                }
                if !seen.insert((x, y)) {
                    continue;
                }
                areas += 1;
                let mut queue = vec![(x, y)];
                while let Some(next) = queue.pop() {
                    for n in Hall::N {
                        let next_n: (i32, i32) = (next.0 + n.0, next.1 + n.1);
                        if !self.map.contains_key(&next_n) {
                            continue;
                        }
                        if !seen.insert(next_n) {
                            continue;
                        }
                        queue.push(next_n);
                    }
                }
            }
        }
        areas
    }

    fn print(&self, file: &mut File) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                if self.map.contains_key(&(x, y)) {
                    write!(file, "#").unwrap();
                } else {
                    write!(file, ".").unwrap();
                }
            }
            writeln!(file).unwrap();
        }
        writeln!(file).unwrap();
        writeln!(file).unwrap();
    }

    fn rotate(&self, rotations: usize) -> Self {
        let mut current = self.clone();
        for _ in 0..rotations {
            current = current.update_map();
        }
        current
    }

    fn update_map(&self) -> Self {
        let mut new_map: BTreeMap<(i32, i32), Vec<Robot>> = BTreeMap::new();
        for (c, robots) in &self.map {
            for robot in robots {
                let x_n = (c.0 + robot.x + self.x_size) % self.x_size;
                let y_n = (c.1 + robot.y + self.y_size) % self.y_size;

                new_map
                    .entry((x_n, y_n))
                    .and_modify(|e| e.push(*robot))
                    .or_insert_with(|| vec![*robot]);
            }
        }
        Self {
            map: new_map,
            x_size: self.x_size,
            y_size: self.y_size,
        }
    }
    fn make_map(input: &str, x_size: i32, y_size: i32) -> Self {
        let mut map: BTreeMap<(i32, i32), Vec<Robot>> = BTreeMap::new();
        for line in input.lines() {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            assert_eq!(parts.len(), 2);

            let start = parts[0]
                .strip_prefix("p=")
                .unwrap()
                .split(',')
                .map(|n| n.parse())
                .collect::<Result<Vec<i32>, _>>()
                .unwrap();
            assert_eq!(start.len(), 2);

            let velocity = parts[1]
                .strip_prefix("v=")
                .unwrap()
                .split(',')
                .map(|n| n.parse())
                .collect::<Result<Vec<i32>, _>>()
                .unwrap();
            assert_eq!(velocity.len(), 2);

            map.entry((start[0], start[1]))
                .and_modify(|e| e.push(Robot::new(velocity[0], velocity[1])))
                .or_insert_with(|| vec![Robot::new(velocity[0], velocity[1])]);
        }

        Hall {
            map,
            x_size,
            y_size,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_equal() {
        assert_eq!(vec![4, 2, 42], vec![4, 2, 42])
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day14_test_data.txt")?;

        // Act
        let hall = Hall::make_map(&input, 11, 7);
        let updated = hall.rotate(100);
        let count = updated.quandrant_count();

        // Assert
        assert_eq!(count, 12);
        Ok(())
    }

    #[test]
    fn test_part_1_memo() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day14_test_data.txt")?;

        // Act
        let hall = Hall::make_map(&input, 11, 7);
        let updated = hall.rotate_memo(100);
        let count = updated.quandrant_count();

        // Assert
        assert_eq!(count, 12);
        Ok(())
    }

    #[test]
    fn test_memo_compare() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day14_data.txt")?;

        // Act
        let hall = Hall::make_map(&input, 101, 103);

        // Assert
        assert_eq!(
            hall.rotate_memo(1_000).quandrant_count(),
            hall.rotate(1_000).quandrant_count()
        );
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }
}
