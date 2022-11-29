pub mod simple;
pub mod value;

#[cfg(test)]
mod test {
    use super::simple;
    use super::value;
    use super::simple::LengthFinder;

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("../../data/day5_part2_data.txt")
            .expect("couldn't open file");
    
        let polymer = simple::PolymerImprover::new(input);
        let min_length = polymer.find_polymer_length();
    
        assert_eq!(min_length, 4996);
    }
    
    #[test]
    fn test_part1_with_write_of_part2() {
        let path = "../../data/day5_part2_data.txt";
        let input = std::fs::read_to_string("../../data/day5_data.txt")
                .expect("couldn't open file");
    
        let mut polymer = simple::Polymer::new(&input);
        let length = polymer.find_polymer_length();
        
        polymer.write_to_file(&path);
    
        assert_eq!(length, 9348);
    }
    
    #[test]
    fn test_simple_polymer_improver() {
        let input = "dabCBAcaDA".to_string();
    
        let polymer = simple::PolymerImprover::new(input);
        let min_length = polymer.find_polymer_length();
    
        assert_eq!(min_length, 4);
    }
    
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
        let output = std::fs::read_to_string(&path)
            .expect("couldn't open file");
    
        assert_eq!(output, "dabCBAcaDA");
    }
    
    #[test]
    fn test_value_polymer_length() {
        let input = "dabAcCaCBAcCcaDA";
        let length = value::get_polymer_length(input);
        assert_eq!(length, 10);
    }
}
