use std::fs;

use day5::simple;

fn main() {
    let input = fs::read_to_string("../data/day5_data.txt")
        .expect("couldn't open file");
    
    let mut polymer = simple::Polymer::new(&input);
    let length = polymer.find_polymer_length();
    
    println!("Part 1: {}", length);

    let chars = polymer.write_to_chars();
    let improved = simple::PolymerImprover::new_from_vector(chars);

    let improved_length = improved.find_polymer_length();

    println!("Part 2: {}", improved_length);
}