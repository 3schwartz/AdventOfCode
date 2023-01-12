use std::{fs, collections::HashMap};

fn main() {
        let input = fs::read_to_string("../data/day9_data.txt")
            .expect("not able to open file");

            let game = Game::new(input.as_str(), 1);
            let score = game.find_highest_score();

            match score {
                Ok(r) => println!("Part 1: {}", r),
                Err(()) => print!("Part 1 error"),
            }            

            let game = Game::new(input.as_str(), 100);
            let score = game.find_highest_score();

            match score {
                Ok(r) => println!("Part 2: {}", r),
                Err(()) => print!("Part 2 error"),
            }            
}

struct Game {
    players: u32,
    marbles: u32
}

impl Game{
    fn new(input: &str, multiple: u32) -> Self {
        let split = input.split(" ")
            .collect::<Vec<&str>>();
        let players : u32 = split[0]
            .parse()
            .expect("not able to parse player");
        let marbles : u32 = split[6]
            .parse()
            .expect("not able to parse marbles");
        
        return Self { players: players, marbles: marbles * multiple }
    }

    fn find_highest_score(&self) -> Result<u128, ()> {
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
                        .map(|m| m.left).ok_or(())?;
                }
                let (left, right) = marbles.get(&current_marble)
                    .map(|m| (m.left, m.right)).ok_or(())?;
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
                .map(|m| m.right).ok_or(())?;
            let right= marbles
                .get_mut(&left)
                .map(|l| {
                    let right = l.right;
                    l.right = round;
                    right
                }).ok_or_else(|| ())?;

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
            .ok_or(());
    }
}


#[derive(Copy, Clone)]
struct Marble {
    idx: u32,
    left : u32,
    right: u32
}

impl Marble {
    fn new(idx: u32, left: u32, right : u32) -> Self {
        return Self { idx, left, right }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!(score, Ok(expected))
    }
}