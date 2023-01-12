
use day9;
use std::{fs};

fn main() {
        let input = fs::read_to_string("../data/day9_data.txt")
            .expect("not able to open file");

            let game = day9::Game::new(input.as_str(), 1);
            let score = game.find_highest_score();

            match score {
                Ok(r) => println!("Part 1: {}", r),
                Err(e) => print!("Part 1 error: {}", e),
            }            

            let game = day9::Game::new(input.as_str(), 100);
            let score = game.find_highest_score();

            match score {
                Ok(r) => println!("Part 2: {}", r),
                Err(e) => print!("Part 2 error: {}", e),
            }            
}