use std::{fs};

fn main() {
    let file = match fs::read_to_string("../data/day1_data.txt") {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file
    };

    let lines = file.split("\r\n");

    let mut sum :i32 = 0;
    for s in lines {
        let number_string = &s[1..];
        let mut number : i32 = number_string.parse().unwrap();
        let sign = s.chars().next().unwrap();
        match sign {
            '+'=> number = number,
            '-'=> number = -1*number,
            _ => println!("{} not known", sign)
        }
        sum+= number;
    }
    println!("Part 1: {}", sum);
}
