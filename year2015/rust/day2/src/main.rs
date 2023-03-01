use std::vec;

use common;
fn main() {
    let lines = common::read_lines("../data/day2_data.txt");

    for line in lines.iter().take(10) {
        println!("{line}")
    }

    let mut paper = 0;
    for line in lines.iter() {
        let dim = line.split('x')
            .flat_map(|c| c.parse::<i32>())
            .collect::<Vec<i32>>();
        assert_eq!(dim.len(), 3, "length not correct");
        let sides = vec![dim[0]*dim[1], dim[1]*dim[2], dim[2]*dim[0]];
        let min = sides.iter()
            .min().unwrap();
        let multiple = sides.iter()
            .map(|c| c * 2)
            .sum::<i32>();
        paper += min + multiple;
    }

    println!("Part 1: {paper}");

    let mut ribbon = 0;
    for line in lines {
        let mut dim = line.split('x')
            .flat_map(|c| c.parse::<i32>())
            .collect::<Vec<i32>>();
        assert_eq!(dim.len(), 3, "length not correct");

        let multiple = dim.iter()
            .fold(1, |total, item| total * item);
        dim.sort();
        let min = dim.iter()
            .take(2)
            .map(|s| s*2)
            .sum::<i32>();
        ribbon += min + multiple; 
    }

    println!("Part 2: {ribbon}");

}
