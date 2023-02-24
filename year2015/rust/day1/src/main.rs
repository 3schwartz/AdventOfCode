use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("../data/day1_data.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let movements = lines
        .filter_map(|l| l.ok())
        .flat_map(|l| l.as_bytes().to_vec())
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => panic!("not known: {}", b.to_string()),
        })
        .collect::<Vec<i32>>();

    let floor = movements
        .iter()
        .sum::<i32>();

    println!("Part 1: {}", floor);

    let base = 0;
    let position = movements
        .iter()
        .scan(base, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .position(|x| x == -1)
        .unwrap() + 1;
    println!("Part 2: {}", position);
}
