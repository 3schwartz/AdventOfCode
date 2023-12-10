use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day7_data.txt")?;

    let mut game = Game::from(&input, false)?;
    let total_winning = game.total_winning();

    println!("Part 1: {}", total_winning);

    let mut game = Game::from(&input, true)?;
    let total_winning = game.total_winning();

    println!("Part 2: {}", total_winning);

    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u16,
    rank: u8,
    with_joker: bool,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.rank > other.rank {
            return Ordering::Greater;
        }
        if self.rank < other.rank {
            return Ordering::Less;
        }

        for i in 0..self.cards.len() {
            let self_strength = self.get_strength(&self.cards[i]);
            let other_strength = self.get_strength(&other.cards[i]);
            if self_strength > other_strength {
                return Ordering::Greater;
            }
            if self_strength < other_strength {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn from(input: &str, with_joker: bool) -> Result<Self> {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        let cards: Vec<char> = parts[0].chars().collect();
        let bid: u16 = parts[1].parse()?;
        let mut map = HashMap::new();
        for c in &cards {
            if !Hand::validate(c) {
                return Err(anyhow!("{} not able to be mapped", c));
            }
            map.entry(c).and_modify(|e| *e += 1).or_insert(1_u8);
        }
        if with_joker {
            Hand::apply_jokers(&mut map)?;
        }
        let rank = Hand::get_rank(map);
        Ok(Self {
            cards,
            bid,
            rank,
            with_joker,
        })
    }

    fn apply_jokers(map: &mut HashMap<&char, u8>) -> Result<()> {
        if let Some(jokers) = map.remove(&'J') {
            if jokers == 5 {
                map.insert(&'A', jokers);
                return Ok(());
            }
            for _ in 0..jokers {
                let max_key = map
                    .iter()
                    .max_by(|a, b| a.1.cmp(b.1))
                    .map(|(k, _)| k)
                    .ok_or_else(|| anyhow!("{:?} should have some max", map))?;
                map.entry(max_key).and_modify(|e| *e += 1);
            }
        }
        Ok(())
    }

    fn validate(c: &char) -> bool {
        let mut valids = HashSet::from([
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ]);
        !valids.insert(*c)
    }

    fn get_strength(&self, c: &char) -> u8 {
        match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => {
                if self.with_joker {
                    0
                } else {
                    10
                }
            }
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            _ => 1,
        }
    }

    fn get_rank(map: HashMap<&char, u8>) -> u8 {
        // Five of a kind
        if map.values().any(|&v| v == 5) {
            return 6;
        }
        // Four of a kind
        if map.values().any(|&v| v == 4) {
            return 5;
        }
        if map.values().any(|&v| v == 3) {
            // Full house
            if map.values().any(|&v| v == 2) {
                return 4;
            }
            // Three of a kind
            return 3;
        }
        // Two pairs
        let pairs: Vec<&u8> = map.values().filter(|&&v| v == 2).collect();
        if pairs.len() == 2 {
            return 2;
        }
        // One pair
        if pairs.len() == 1 {
            return 1;
        }
        0
    }
}

struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn from(input: &str, with_joker: bool) -> Result<Game> {
        let mut hands = vec![];
        for line in input.lines() {
            hands.push(Hand::from(line, with_joker)?);
        }
        Ok(Self { hands })
    }

    fn total_winning(&mut self) -> u64 {
        self.hands.sort();
        self.hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u64 + 1) * (h.bid as u64))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day7_data_test.txt")?;

        // Act
        let mut game = Game::from(&input, true)?;
        let total_winning = game.total_winning();

        // Assert
        assert_eq!(5905, total_winning);
        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day7_data_test.txt")?;

        // Act
        let mut game = Game::from(&input, false)?;
        let total_winning = game.total_winning();

        // Assert
        assert_eq!(6440, total_winning);
        Ok(())
    }
}
