use anyhow::Result;
use std::{collections::BTreeMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day14_data.txt")?;

    let hall = Hall::make_map(&input, 101, 103);
    let updated = hall.rotate(100);
    let count = updated.quandrant_count();

    println!("Part 1: {}", count);

    Ok(())
}

#[derive(Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
}

impl Robot {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Hall {
    map: BTreeMap<(i32, i32), Vec<Robot>>,
    x_size: i32,
    y_size: i32,
}

impl Hall {
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
        counts.iter().fold(1, |acc, e| acc * e)
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
    fn test_part_2() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }
}
