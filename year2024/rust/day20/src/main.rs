use anyhow::anyhow;
use anyhow::Result;
use std::collections::BTreeSet;
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

    Ok(())
}

struct Labyrint {
    walls: HashSet<(i32, i32)>,
    x_max: i32,
    y_max: i32,
    start: (i32, i32),
    end: (i32, i32),
}

struct State {
    steps: i32,
    cheat_start: (i32, i32),
    cheat_end: Option<(i32, i32)>,
    cheat_path: BTreeSet<(i32, i32)>,
    coord: (i32, i32),
}

impl Labyrint {
    const N: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

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

    fn find_multiple_cheats_below(&self, below: i32) -> i32 {
        let fastest = self.find_fastest();
        let cheats = self.find_multiple_cheats(fastest, below);
        cheats.iter().fold(0, |acc, (_, v)| acc + v)
    }

    fn find_multiple_cheats(&self, fastest: i32, limit: i32) -> HashMap<i32, i32> {
        let mut queue = VecDeque::from([State {
            steps: 0,
            cheat_start: self.start,
            cheat_end: None,
            cheat_path: BTreeSet::new(),
            coord: self.start,
        }]);
        let mut seen_with_end = HashSet::new();
        let mut seen_without_end = HashSet::new();
        let mut seen_cheats = HashSet::new();
        let mut rutes = HashMap::new();
        while let Some(state) = queue.pop_front() {
            let next = state.coord;
            let steps = state.steps;
            let mut cheat_end = state.cheat_end;
            let mut cheat_path = state.cheat_path;
            if next.0 < 0 || next.0 > self.x_max || next.1 < 0 || next.1 > self.y_max {
                continue;
            }
            if steps > fastest - limit {
                break;
            }
            if next == self.end {
                rutes.entry(steps).and_modify(|e| *e += 1).or_insert(1);
                continue;
            }
            // Either cheat is done and uniqueness should be on current, start and end or
            // we need to look at the cheat path.
            if state.cheat_end.is_some() {
                if !seen_with_end.insert((state.cheat_start, state.cheat_end, state.coord)) {
                    continue;
                }
            }
            if cheat_path.contains(&state.coord) {
                continue;
            }
            // Already cheated
            if self.walls.contains(&next) && state.cheat_end.is_some() {
                continue;
            }
            // Can't cheat more
            if self.walls.contains(&next) && cheat_path.len() == 19 {
                continue;
            }
            // Can cheat and push one more on the path
            if self.walls.contains(&next) {
                cheat_path.insert(state.coord);
            }
            // Currently cheating and end
            if !self.walls.contains(&next) && cheat_path.len() > 0 && state.cheat_end.is_none() {
                if !seen_cheats.insert((state.cheat_start, next)) {
                    continue;
                }
                cheat_end = Some(state.coord)
            }
            let cheat_start = if cheat_path.len() == 0 {
                next
            } else {
                state.cheat_start
            };

            for n in Self::N {
                let neighbor = (next.0 + n.0, next.1 + n.1);
                // and prior as start if some condition
                queue.push_back(State {
                    steps: steps + 1,
                    cheat_start,
                    cheat_end,
                    cheat_path: cheat_path.clone(),
                    coord: neighbor,
                });
            }
        }
        rutes
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
    fn test_part_2() -> Result<()> {
        // Arrange
        let expected_below = [(3, 76)];
        let input = fs::read_to_string("../../data/day20_test_data.txt")?;

        // Act
        let labyrint = Labyrint::from_str(&input)?;
        for (expected, below) in expected_below {
            let actual = labyrint.find_multiple_cheats_below(below);
            // Assert
            assert_eq!(actual, expected);
        }
        Ok(())
    }
}