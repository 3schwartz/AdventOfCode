use std::fs;

pub mod simple;
pub mod value;

#[test]
fn test_simple_polymer_length() {
    let input = "dabAcCaCBAcCcaDA";
    let mut polymer = simple::Polymer::new(input);
    let length = polymer.find_polymer_length();
    assert_eq!(length, 10);
}

#[test]
fn test_simple_write() {
    let path = "../../data/day5_test_simple_data.txt";
    let input = "dabAcCaCBAcCcaDA";
    let mut polymer = simple::Polymer::new(input);
    let _ = polymer.find_polymer_length();
    
    polymer.write_to_file(&path);
    let output = fs::read_to_string(&path)
        .expect("couldn't open file");

    assert_eq!(output, "dabCBAcaDA");
}

#[test]
fn test_value_polymer_length() {
    let input = "dabAcCaCBAcCcaDA";
    let length = value::get_polymer_length(input);
    assert_eq!(length, 10);
}