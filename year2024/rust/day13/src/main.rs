use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day13_data.txt")?;

    let games = Game::make_games(&input);
    let mut cost = 0;
    let mut cost_det = 0;
    let mut part_2 = 0;
    for game in games {
        if let Some(c) = game.find_cost() {
            cost += c;
        }
        if let Some(c) = game.find_cost_using_determinant(0) {
            cost_det += c;
        }
        if let Some(c) = game.find_cost_using_determinant(10000000000000) {
            part_2 += c;
        }
    }

    println!("Part 1: {}", cost);
    println!("Part 1: {}", cost_det);
    println!("Part 2: {}", part_2);

    Ok(())
}

struct Game {
    a: (i128, i128),
    b: (i128, i128),
    f: (i128, i128),
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
                .parse::<i128>()
                .unwrap();
            let a_y = a_parts[1]
                .strip_prefix("Y+")
                .unwrap()
                .parse::<i128>()
                .unwrap();

            let b_parts = lines[1]
                .strip_prefix("Button B: ")
                .unwrap()
                .split(", ")
                .collect::<Vec<&str>>();
            let b_x = b_parts[0]
                .strip_prefix("X+")
                .unwrap()
                .parse::<i128>()
                .unwrap();
            let b_y = b_parts[1]
                .strip_prefix("Y+")
                .unwrap()
                .parse::<i128>()
                .unwrap();

            let f_parts = lines[2]
                .strip_prefix("Prize: ")
                .unwrap()
                .split(", ")
                .collect::<Vec<&str>>();
            let f_x = f_parts[0]
                .strip_prefix("X=")
                .unwrap()
                .parse::<i128>()
                .unwrap();
            let f_y = f_parts[1]
                .strip_prefix("Y=")
                .unwrap()
                .parse::<i128>()
                .unwrap();
            games.push(Game {
                a: (a_x, a_y),
                b: (b_x, b_y),
                f: (f_x, f_y),
            });
        }
        games
    }

    /// The game has unique solutions, which can be determined by solving the
    /// corresponding system of linear equations.
    fn find_cost_using_determinant(&self, shift: i128) -> Option<i128> {
        let f_x = self.f.0 + shift;
        let f_y = self.f.1 + shift;

        let coef = [[self.a.0, self.b.0], [self.a.1, self.b.1]];
        let determinant = coef[0][0] * coef[1][1] - coef[0][1] * coef[1][0];
        if determinant == 0 {
            return None;
        }

        let a = (coef[1][1] * f_x - coef[0][1] * f_y) / determinant;
        let b = (-coef[1][0] * f_x + coef[0][0] * f_y) / determinant;
        if self.a.0 * a + self.b.0 * b != f_x || self.a.1 * a + self.b.1 * b != f_y {
            return None;
        }
        Some(a * 3 + b)
    }

    fn find_cost(&self) -> Option<i128> {
        let mut queue = HashMap::new();
        queue.insert(0, HashSet::from([(0, 0)]));
        let mut seen = HashSet::from([(0, 0)]);

        let mut cost = 0;
        let movement = [(3, self.a), (1, self.b)];
        loop {
            if queue.is_empty() {
                break;
            }
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
