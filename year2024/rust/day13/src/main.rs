use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day13_data.txt")?;

    let games = Game::make_games(&input);
    let mut cost = 0;
    for game in games {
        if let Some(c) = game.find_cost() {
            cost += c;
        }
    }

    println!("Part 1: {}", cost);

    Ok(())
}

struct Game {
    a: (u32, u32),
    b: (u32, u32),
    f: (u32, u32),
}

impl Game {
    fn make_games(input: &str) -> Vec<Game> {
        let mut games = vec![];
        let games_str = input.trim().split("\n\n").collect::<Vec<&str>>();

        for game in games_str {
            let lines = game.split("\n").collect::<Vec<&str>>();
            if lines.len() > 3 {
                assert_eq!(lines.len(), 3);
            }
            assert_eq!(lines.len(), 3);
            let a_parts = lines[0]
                .strip_prefix("Button A: ")
                .unwrap()
                .split(", ")
                .collect::<Vec<&str>>();
            let a_x = a_parts[0]
                .strip_prefix("X+")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let a_y = a_parts[1]
                .strip_prefix("Y+")
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let b_parts = lines[1]
                .strip_prefix("Button B: ")
                .unwrap()
                .split(", ")
                .collect::<Vec<&str>>();
            let b_x = b_parts[0]
                .strip_prefix("X+")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let b_y = b_parts[1]
                .strip_prefix("Y+")
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let f_parts = lines[2]
                .strip_prefix("Prize: ")
                .unwrap()
                .split(", ")
                .collect::<Vec<&str>>();
            let f_x = f_parts[0]
                .strip_prefix("X=")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let f_y = f_parts[1]
                .strip_prefix("Y=")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            games.push(Game {
                a: (a_x, a_y),
                b: (b_x, b_y),
                f: (f_x, f_y),
            });
        }
        games
    }

    fn find_cost(&self) -> Option<u32> {
        let mut queue = HashMap::new();
        queue.insert(0, HashSet::from([(0, 0)]));
        let mut seen = HashSet::from([(0, 0)]);

        let mut cost = 0;
        let movement = [(3, self.a), (1, self.b)];
        loop {
            if queue.is_empty() {
                break;
            }
            // if cost > 500 {
            //     break;
            // }
            if !queue.contains_key(&cost) {
                cost += 1;
                continue;
            }

            for c in queue.remove(&cost).unwrap() {
                for (t, m) in movement {
                    let next = (c.0 + m.0, c.1 + m.1);
                    let next_cost = t + cost;
                    if next.0 == self.f.0 && next.1 == self.f.1 {
                        return Some(next_cost);
                    }
                    if next.0 > self.f.0 || next.1 > self.f.1 {
                        continue;
                    }
                    if !seen.insert(next) {
                        continue;
                    }

                    queue
                        .entry(next_cost)
                        .and_modify(|v| {
                            v.insert(next);
                        })
                        .or_insert_with(|| HashSet::from([next]));
                }
            }

            cost += 1;
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day13_test_data.txt")?;

        // Act
        let games = Game::make_games(&input);
        let mut cost = 0;
        for game in games {
            if let Some(c) = game.find_cost() {
                cost += c;
            }
        }

        // Assert
        assert_eq!(cost, 480);
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
