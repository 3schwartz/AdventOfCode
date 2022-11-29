use std::collections::HashMap;
use std::fs;
use std::str::Split;

fn main() {
    let file = match fs::read_to_string("../data/day2_data.txt")  {
        Err(why) => panic!("couldn't open file {}", why),
        Ok(file) => file
    };
    let lines = file.split("\r\n");

    let mut two_times: i32 = 0;
    let mut three_times: i32 = 0;
    for line in lines.clone() {
        let mut letters = HashMap::new();
        for c in line.chars() {
            *letters.entry(c).or_insert(0) += 1;
        }

        let mut accounted_two : bool = false;
        let mut accounted_three : bool = false;
        for (_, &count) in letters.iter() {
            match count{
                2 => {
                    if !accounted_two {
                        accounted_two = true;
                        two_times +=1
                    }
                },
                3 => {
                    if !accounted_three {
                        accounted_three = true;
                        three_times +=1
                    }
                },
                _ => ()
            }
            if accounted_two && accounted_three {
                break
            }
        }
    }
    println!("Part 1: {}", two_times * three_times);

    let (first_line, second_line) = match find_almost_equal_lines(lines) {
        Some(value) => value,
        None => panic!("Didn't find almost equal")

    };

    print!("Part 2: ");
    let first_chars = first_line.chars();
    let second_chars = second_line.chars();
    for (c1, c2) in first_chars.zip(second_chars){
        if c1 == c2 {
            print!("{}", c1)
        }
    }
    println!()

}

fn find_almost_equal_lines<'a>(lines : Split<'a, &'a str>) -> Option<(&str, &str)> {
    for first_line in lines.clone() {
        for second_line in lines.clone() {
            let mut not_equals: i32 = 0;
            let first_chars = first_line.chars();
            let second_chars = second_line.chars();

            for (c1, c2) in first_chars.zip(second_chars){
                if c1 == c2 {
                    continue
                }
                not_equals += 1
            }

            if not_equals != 1 {
                continue
            }

            return Some((first_line.clone(), second_line.clone()))
        }
    }
    None
}

