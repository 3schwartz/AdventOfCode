use anyhow::anyhow;
use anyhow::Result;
use std::fs;
use std::time::Instant;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day20_data.txt")?;

    let labyrint = Labyrint::from_str(&input)?;

    let instant = Instant::now();
    let below = labyrint.find_cheats_below(100);
    println!(
        "Milliseconds: {}",
        Instant::now().duration_since(instant).as_millis()
    );
    println!("Part 1: {below}");

    let actual = labyrint.find_multiple_cheats_below(100, 2)?;
    println!("Part 1: {actual}");

    let actual = labyrint.find_multiple_cheats_below(100, 20)?;

    println!("Part 2: {actual}");

    Ok(())
}

struct Labyrint {
    walls: HashSet<(i32, i32)>,
    x_max: i32,
    y_max: i32,
    start: (i32, i32),
    end: (i32, i32),
}

impl Labyrint {
    const N: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    #[allow(dead_code)]
    fn find_cheats_below(&self, below: i32) -> i32 {
        let fastest = self.find_fastest();
        let cheats = self.find_cheats((None, self.start), Some((fastest, below)));
        cheats.iter().fold(0, |acc, (_, v)| acc + v)
    }

    fn find_fastest(&self) -> i32 {
        let f = self.find_cheats((Some(self.start), self.start), None);
        if f.len() != 1 {
            panic!("should only have found one result")
        };
        let (steps, count) = f.iter().next().unwrap();
        if *count != 1 {
            panic!("should only have found one route")
        }
        *steps
    }

    fn find_multiple_cheats_below(&self, below: i32, max_cheats: u8) -> Result<usize> {
        let fastest = self.find_fastest();
        let distances = self.find_distances();
        self.find_multiple_cheats(&distances, fastest, below, max_cheats)
    }

    fn find_distances(&self) -> HashMap<(i32, i32), i32> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::from([(0, self.end)]);
        while let Some((distance, next)) = queue.pop_front() {
            if next.0 < 0 || next.0 > self.x_max || next.1 < 0 || next.1 > self.y_max {
                continue;
            }
            if self.walls.contains(&next) {
                continue;
            }
            if distances.contains_key(&next) {
                continue;
            }
            distances.insert(next, distance);
            for n in Self::N {
                let neighbor = (next.0 + n.0, next.1 + n.1);
                queue.push_back((distance + 1, neighbor));
            }
        }

        distances
    }

    fn find_multiple_cheats(
        &self,
        distances: &HashMap<(i32, i32), i32>,
        fastest: i32,
        below: i32,
        max_cheats: u8,
    ) -> Result<usize> {
        let mut cheats: HashSet<(Option<(i32, i32)>, (i32, i32))> = HashSet::new();
        let mut queue: VecDeque<(i32, Option<(i32, i32)>, u8, (i32, i32))> =
            VecDeque::from([(0, None, 0, self.start)]);
        let mut seen = HashSet::new();
        while let Some((distance, start, cheat_count, next)) = queue.pop_front() {
            if next.0 < 0 || next.0 > self.x_max || next.1 < 0 || next.1 > self.y_max {
                continue;
            }
            if distance > fastest - below {
                continue;
            }
            if next == self.end {
                if distance <= fastest - below {
                    cheats.insert((start, next));
                }
            }
            if !seen.insert((start, next, cheat_count)) {
                continue;
            }
            if start.is_none() {
                assert!(!self.walls.contains(&next));
                queue.push_back((distance, Some(next), 0, next));
            }
            if start.is_some() && !self.walls.contains(&next) {
                let distance_from = distances
                    .get(&next)
                    .ok_or_else(|| anyhow!("{:?} missing", next))?;
                if *distance_from <= fastest - below - distance {
                    cheats.insert((start, next));
                }
            }
            if cheat_count == max_cheats {
                continue;
            }
            for n in Self::N {
                let neighbor = (next.0 + n.0, next.1 + n.1);
                if start.is_some() {
                    queue.push_back((distance + 1, start, cheat_count + 1, neighbor));
                    continue;
                }
                if !self.walls.contains(&neighbor) {
                    queue.push_back((distance + 1, None, 0, neighbor));
                }
            }
        }
        Ok(cheats.len())
    }

    fn find_cheats(
        &self,
        start: (Option<(i32, i32)>, (i32, i32)),
        prior: Option<(i32, i32)>,
    ) -> HashMap<i32, i32> {
        let mut queue = VecDeque::from([(0, start.0, start.1)]);
        let mut seen = HashSet::new();
        let mut rutes = HashMap::new();
        while let Some((steps, mut cheated, next)) = queue.pop_front() {
            if next.0 < 0 || next.0 > self.x_max || next.1 < 0 || next.1 > self.y_max {
                continue;
            }
            if let Some((fastest, limit)) = prior {
                if steps > fastest - limit {
                    break;
                }
            }
            if next == self.end {
                if let Some((fastest, limit)) = prior {
                    if steps <= fastest - limit {
                        rutes.entry(steps).and_modify(|e| *e += 1).or_insert(1);
                    }
                    continue;
                } else {
                    rutes.entry(steps).and_modify(|e| *e += 1).or_insert(1);
                    break;
                }
            }
            if !seen.insert((cheated, next)) {
                continue;
            }
            if self.walls.contains(&next) {
                if cheated.is_some() {
                    continue;
                } else {
                    cheated = Some(next);
                }
            }
            for n in Self::N {
                let neighbor = (next.0 + n.0, next.1 + n.1);
                queue.push_back((steps + 1, cheated, neighbor));
            }
        }
        rutes
    }
}

impl FromStr for Labyrint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut y_max = 0;
        let mut x_max = 0;
        let mut start = None;
        let mut end = None;
        let mut walls = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            y_max = y as i32;
            for (x, c) in line.chars().enumerate() {
                x_max = x as i32;
                match c {
                    'S' => start = Some((x as i32, y as i32)),
                    'E' => end = Some((x as i32, y as i32)),
                    '#' => {
                        walls.insert((x as i32, y as i32));
                    }
                    _ => continue,
                }
            }
        }
        Ok(Self {
            walls,
            x_max,
            y_max,
            start: start.ok_or_else(|| anyhow!("start is missing"))?,
            end: end.ok_or_else(|| anyhow!("end is missing"))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_fastest() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day20_test_data.txt")?;

        // Act
        let labyrint = Labyrint::from_str(&input)?;
        let fastest = labyrint.find_fastest();

        // Assert
        assert_eq!(fastest, 84);
        Ok(())
    }

    #[test]
    fn test_part_1_below() -> Result<()> {
        // Arrange
        let expected_below = [
            (0, 65),
            (1, 64),
            (5, 20),
            (8, 12),
            (10, 10),
            (14, 8),
            (16, 6),
            (30, 4),
            (44, 2),
        ];
        let input = fs::read_to_string("../../data/day20_test_data.txt")?;

        // Act
        let labyrint = Labyrint::from_str(&input)?;
        for (expected, below) in expected_below {
            let actual = labyrint.find_cheats_below(below);
            // Assert
            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let expected_below = [
            (0, 65),
            (1, 64),
            (5, 20),
            (8, 12),
            (10, 10),
            (14, 8),
            (16, 6),
            (30, 4),
            (44, 2),
        ];
        let input = fs::read_to_string("../../data/day20_test_data.txt")?;

        // Act
        let labyrint = Labyrint::from_str(&input)?;
        for (expected, below) in expected_below {
            let actual = labyrint.find_multiple_cheats_below(below, 2);
            // Assert
            assert_eq!(actual.unwrap(), expected);
        }
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let expected_below: Vec<(Result<usize>, i32)> = vec![
            (Ok(193), 56),
            (Ok(154), 58),
            (Ok(129), 60),
            (Ok(106), 62),
            (Ok(86), 64),
            (Ok(67), 66),
            (Ok(55), 68),
            (Ok(41), 70),
            (Ok(29), 72),
            (Ok(7), 74),
            (Ok(3), 76),
        ];
        let input = fs::read_to_string("../../data/day20_test_data.txt")?;

        let labyrint = Labyrint::from_str(&input)?;
        for (expected, below) in expected_below {
            // Act
            let actual = labyrint.find_multiple_cheats_below(below, 20);

            // Assert
            assert_eq!(actual.unwrap(), expected.unwrap());
        }
        Ok(())
    }
}
