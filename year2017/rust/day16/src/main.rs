use std::{collections::BTreeMap, fs};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day16_data.txt")?;

    let moves = parse_moves(input.trim());
    let mut state = State::new(16);
    state.apply_moves(moves)?;
    let standing = state.program_standing();

    println!("Part 1: {standing}");
    Ok(())
}

fn parse_spin(input: &str) -> Result<usize> {
    let p = &input[1..];
    Ok(p.parse()?)
}

fn parse_exchange(input: &str) -> Result<(usize, usize)> {
    let p = &input[1..];
    let split = p
        .split('/')
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;
    assert_eq!(split.len(), 2);
    Ok((split[0], split[1]))
}

fn parse_partner(input: &str) -> Result<(char, char)> {
    let p = &input[1..];
    let chars = p.chars().filter(|c| *c != '/').collect::<Vec<char>>();
    assert_eq!(chars.len(), 2);

    Ok((chars[0], chars[1]))
}

struct State {
    positions: BTreeMap<usize, char>,
    programs: BTreeMap<char, usize>,
}

impl State {
    fn new(lenght: usize) -> Self {
        let tuples = Self::make_initial_line(lenght);
        let mut positions = BTreeMap::new();
        let mut programs = BTreeMap::new();
        for (p, c) in tuples {
            positions.insert(p, c);
            programs.insert(c, p);
        }
        Self {
            positions,
            programs,
        }
    }

    fn make_move(&mut self, dance_move: &str) -> Result<()> {
        match dance_move.chars().next() {
            Some('x') => {
                let (e1, e2) = parse_exchange(dance_move)?;
                let ec1 = *self
                    .positions
                    .get(&e1)
                    .ok_or_else(|| anyhow!("missing position {e1}"))?;
                let ec2 = *self
                    .positions
                    .get(&e2)
                    .ok_or_else(|| anyhow!("missing position {e2}"))?;
                self.positions.insert(e1, ec2);
                self.positions.insert(e2, ec1);

                self.programs.insert(ec1, e2);
                self.programs.insert(ec2, e1);
            }
            Some('p') => {
                let (pc1, pc2) = parse_partner(dance_move)?;
                let p1 = *self
                    .programs
                    .get(&pc1)
                    .ok_or_else(|| anyhow!("missing program {pc1}"))?;
                let p2 = *self
                    .programs
                    .get(&pc2)
                    .ok_or_else(|| anyhow!("missing program {pc2}"))?;
                self.programs.insert(pc1, p2);
                self.programs.insert(pc2, p1);

                self.positions.insert(p1, pc2);
                self.positions.insert(p2, pc1);
            }
            Some('s') => {
                let spin = parse_spin(dance_move)?;
                let length = self.programs.len();
                for (_, v) in self.programs.iter_mut() {
                    *v = (*v + spin) % length;
                }
                for (e, v) in &self.programs {
                    self.positions.insert(*v, *e);
                }
            }
            _ => return Err(anyhow!("{dance_move}")),
        }
        Ok(())
    }

    fn apply_moves(&mut self, moves: Vec<&str>) -> Result<()> {
        for dance_move in moves {
            self.make_move(dance_move)?;
        }
        Ok(())
    }

    fn program_standing(&self) -> String {
        self.positions.values().collect()
    }

    fn make_initial_line(lenght: usize) -> Vec<(usize, char)> {
        let chars = ('a'..).take(lenght);
        let tuples: Vec<(usize, char)> = (0..lenght).zip(chars).collect();
        tuples
    }
}

fn parse_moves(input: &str) -> Vec<&str> {
    input.split(',').collect::<Vec<&str>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = "s1,x3/4,pe/b";

        // Act
        let moves = parse_moves(input);
        let mut state = State::new(5);
        state.apply_moves(moves)?;
        let standing = state.program_standing();

        // Assert
        assert_eq!(standing, "baedc");
        Ok(())
    }
}
