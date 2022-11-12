use std::{fs, collections::HashMap};

fn main() {
    let file = fs::read_to_string("../../data/day3_data.txt")
        .expect("couldn't open file");
    let lines = file.split("\r\n");

    let mut fabrics = HashMap::new();

    for s in lines {
        let claim_fabric_definition: Vec<&str> = s.split(" @ ")
            .collect();
        
        let definition_split: Vec<&str> = claim_fabric_definition[1]
            .split(": ")
            .collect();
        
        let pixel_start: Vec<&str> = definition_split[0].split(",")
            .collect();
        let pixel_length: Vec<&str> = definition_split[1].split("x")
            .collect();
        
        let pixel_start_column : i32 = pixel_start[0].parse()
            .expect("couldn't parse pixel start column");
        let pixel_start_row : i32 = pixel_start[1].parse()
            .expect("couldn't parse pixel start row");            

        let pixel_length_row : i32 = pixel_length[1].parse()
            .expect("couldn't parse pixel length row");                        
        let pixel_length_column : i32 = pixel_length[0].parse()
            .expect("couldn't parse pixel length column");            
        
        for r in pixel_start_row..pixel_start_row+pixel_length_row {
            for j in pixel_start_column..pixel_start_column+pixel_length_column {
                *fabrics.entry((r,j)).or_insert(0) += 1;
            }
        }
    }

    let mut sum = 0;
    for (_, count) in &fabrics {
        if (*count) > 1 {
            sum += 1;
        }
    }

    println!("Part 1 {}", sum)
}
