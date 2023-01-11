fn main() {
    println!("Hello")
}

struct Game {
    players: u32,
    marbles: u32
}

impl Game{
    fn new(input: &str) -> Self {
        let split = input.split(" ")
            .collect::<Vec<&str>>();
        let players : u32 = split[0]
            .parse()
            .expect("not able to parse player");
        let marbles : u32 = split[6]
            .parse()
            .expect("not able to parse marbles");
        
        return Self { players, marbles }
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
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_part1(){
        // Arrange
        // let file = fs::read_to_string("../data/day8_test_data.txt")
        //     .expect("not able to open file");
        let input = "10 players; last marble is worth 1618 points: high score is 8317";
        
        // Act
        let score = findHighestScore(input);

        // Assert
        let expected: u32 = input.split(" ").collect::<Vec<&str>>()
            .iter()
            .nth(11)
            .expect("not able to get expected")
            .parse()
            .expect("not able to parse expected");
        assert_eq!(score, expected)
    }

    fn findHighestScore(input: &str) -> u32 {
        let game = Game::new(input);

        let mut player = 0;
        let mut next_marble = 0;
        let mut marbles : HashMap<u32, Marble> = HashMap::from([
            (0, Marble::new(0,0,0))
        ]);
        for round in 0..game.marbles {

            let right = marbles.get_mut(&next_marble)
                .map(|l| {
                    let temp = l.right;
                    l.right = round;
                    temp
                })
                .expect("left doesn't exist");
            marbles.get_mut(&right)
                .map(|r|{
                    r.left = round;
                })
                .expect("right doesn't exist");
            
            let new_marble = Marble::new(round, next_marble, right);
            marbles.insert(new_marble.idx, new_marble);

            player = (player + 1) % game.players;
            next_marble = right;
        }


        return 0;
    }
}