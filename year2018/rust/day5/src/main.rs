mod simple;

use std::{fs};

fn main() {
    let input = fs::read_to_string("../../data/day5_data.txt")
        .expect("couldn't open file");
    let length = simple::get_polymer_length(input.trim());
    println!("Part 1: {}", length)
}

#[test]
fn test_get_polymer_length() {
    let input = "dabAcCaCBAcCcaDA";
    let length = simple::get_polymer_length(input);
    assert_eq!(length, 10);
}
