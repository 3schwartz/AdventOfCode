use std::{fs};
use std::collections::HashSet;

fn main() {
    let file = match fs::read_to_string("../../data/day1_data.txt") {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file
    };

    let lines = file.split("\r\n");

    let mut sum :i32 = 0;
    for s in lines.clone() {
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

    let mut visited = HashSet::new();
    sum = 0;
    let mut found = false;
    loop {
        for s in lines.clone() {
            let number_string = &s[1..];
            let mut number : i32 = number_string.parse().unwrap();
            let sign = s.chars().next().unwrap();
            match sign {
                '+'=> number = number,
                '-'=> number = -1*number,
                _ => println!("{} not known", sign)
            }
            sum+= number;

            if visited.contains(&sum) {
                found = true;
                break
            }
            visited.insert(sum);
        }
        if found {
            break
        }
    }
    println!("Part 2: {}", sum);
}
