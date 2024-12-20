use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day20_data.txt")?;

    for line in input.lines() {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{
        collections::{HashSet, VecDeque},
        str::FromStr,
    };

    use anyhow::anyhow;

    use super::*;

    struct Labyrint {
        walls: HashSet<(i32, i32)>,
        x_max: i32,
        y_max: i32,
        start: (i32, i32),
        end: (i32, i32),
    }

    impl Labyrint {
        const N: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        fn find_fastest(&self) -> u32 {
            let mut queue = VecDeque::from([(0, self.start)]);
            let mut seen = HashSet::new();
            while let Some((steps, next)) = queue.pop_front() {
                if next.0 < 0 || next.0 > self.x_max || next.1 < 0 || next.1 > self.y_max {
                    continue;
                }
                if next == self.end {
                    return steps;
                }
                if self.walls.contains(&next) {
                    continue;
                }
                if !seen.insert(next) {
                    continue;
                }
                for n in Self::N {
                    let neighbor = (next.0 + n.0, next.1 + n.1);
                    queue.push_back((steps + 1, neighbor));
                }
            }
            panic!("no result found")
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
    fn test_part_2() -> Result<()> {
        // Arrange
        // Act
        // Assert
        Ok(())
    }
}
