use std::{collections::{HashMap, VecDeque}};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;


pub struct Game {
    players: u32,
    marbles: u32
}

impl Game{
    pub fn new(input: &str, multiple: u32) -> Self {
        let split = input.split(" ")
            .collect::<Vec<&str>>();
        let players : u32 = split[0]
            .parse()
            .expect("not able to parse player");
        let marbles : u32 = split[6]
            .parse()
            .expect("not able to parse marbles");
        
        return Self { players, marbles: marbles * multiple }
    }

    pub fn find_highest_score_vec(&self) -> GenericResult<u128> {
        let mut player = 0;
        let mut current_idx = 0;
        let mut marbles = vec![0];
        let mut players = vec![0; self.players as usize];

        for round in 1..=self.marbles {
            player = (player + 1) % self.players;
            if round % 23 == 0 {
                current_idx = (current_idx + marbles.len() - 7) % marbles.len();
                let removed_marble = marbles.remove(current_idx);
                players[player as usize] += round + removed_marble;
                continue;
            }
            current_idx = (current_idx + 1) % marbles.len();
            marbles.insert(current_idx + 1, round);
            current_idx += 1;
        }

        return players
            .iter()
            .max()
            .ok_or_else(|| GenericError::from("Not able to find max"))
            .map(|&x| x as u128);
    }

    pub fn find_highest_score_deque(&self) -> GenericResult<u128> {
        let mut player = 0;
        let mut current_idx = 0;
        let mut marbles = VecDeque::with_capacity((self.marbles + 1) as usize);
        marbles.push_back(0);
        let mut players = vec![0; self.players as usize];

        for round in 1..=self.marbles {
            player = (player + 1) % self.players;
            if round % 23 == 0 {
                current_idx = (current_idx + marbles.len() - 7) % marbles.len();

                let removed_marble = marbles.remove(current_idx).unwrap();
                players[player as usize] += round + removed_marble;
                continue;
            }
            current_idx = (current_idx + 1) % marbles.len();
            marbles.insert(current_idx + 1, round);
            current_idx += 1;
        }

        return players.iter().max().map(|&x| x as u128).ok_or_else(|| GenericError::from("Not able to find max"));
    }

    pub fn find_highest_score(&self) -> GenericResult<u128> {
        let mut player = 0;
        let mut current_marble = 0;
        let mut marbles : HashMap<u32, Marble> = HashMap::from([
            (0, Marble::new(0,0,0))
        ]);
        let mut players : HashMap<u32, u128> = HashMap::new();
        for round in 1..=self.marbles {
            player = (player + 1) % self.players;
            if round % 23 == 0 {
                for _ in 0..7 {
                    current_marble = marbles.get(&current_marble)
                        .map(|m| m.left)
                        .ok_or_else(|| GenericError::from("counter clockwise"))?;
                }
                let (left, right) = marbles.get(&current_marble)
                    .map(|m| (m.left, m.right))
                    .ok_or_else(|| GenericError::from("missing counter clockwise"))?;
                marbles.get_mut(&left)
                    .map(|m| m.right = right);
                marbles.get_mut(&right)
                    .map(|m| m.left = left);
                let score = (round + current_marble) as u128;
                players.entry(player)
                    .and_modify(|s| *s += score)
                    .or_insert(score);
                marbles.remove(&current_marble);
                current_marble = right;
                continue;
            }
            let left = marbles.get(&current_marble)
                .map(|m| m.right)
                .ok_or_else(|| GenericError::from("Not able to find left"))?;
            let right= marbles
                .get_mut(&left)
                .map(|l| {
                    let right = l.right;
                    l.right = round;
                    right
                })
                .ok_or_else(|| GenericError::from("Not able to find right"))?;

            marbles.get_mut(&right)
                .map(|r|{
                    r.left = round;
                })
                .expect("right doesn't exist");
            
            let new_marble = Marble::new(round, left, right);
            marbles.insert(new_marble.idx, new_marble);

            current_marble = round;
        }

        return players.iter()
            .max_by_key(|k| k.1)
            .map(|m| *m.1)
            .ok_or_else(|| GenericError::from("Not able to find max"));
    }
}


struct Marble {
    idx: u32,
    left : u32,
    right: u32
}

impl Marble {
    pub fn new(idx: u32, left: u32, right : u32) -> Self {
        return Self { idx, left, right }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_vec(){
        // Arrange
        let input = "10 players; last marble is worth 1618 points: high score is 8317";
        let game = Game::new(input, 1);

        // Act
        let score = game.find_highest_score_vec();

        // Assert
        let expected: u128 = input.split(" ").collect::<Vec<&str>>()
            .iter()
            .nth(11)
            .expect("not able to get expected")
            .parse()
            .expect("not able to parse expected");
        match score {
            Ok(r) => assert_eq!(r, expected),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_part1_deque(){
        // Arrange
        let input = "10 players; last marble is worth 1618 points: high score is 8317";
        let game = Game::new(input, 1);

        // Act
        let score = game.find_highest_score_deque();

        // Assert
        let expected: u128 = input.split(" ").collect::<Vec<&str>>()
            .iter()
            .nth(11)
            .expect("not able to get expected")
            .parse()
            .expect("not able to parse expected");
        match score {
            Ok(r) => assert_eq!(r, expected),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_part1(){
        // Arrange
        let input = "10 players; last marble is worth 1618 points: high score is 8317";
        let game = Game::new(input, 1);

        // Act
        let score = game.find_highest_score();

        // Assert
        let expected: u128 = input.split(" ").collect::<Vec<&str>>()
            .iter()
            .nth(11)
            .expect("not able to get expected")
            .parse()
            .expect("not able to parse expected");
        match score {
            Ok(r) => assert_eq!(r, expected),
            Err(_) => assert!(false),
        }
    }
}