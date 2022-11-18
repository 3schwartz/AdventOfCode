use std::{fs, collections::HashMap};

fn get_date_as_type(line : &str) -> (i16, i8, i8, i8, i8){
    let (first, _) = line.split_at(18);
    let date = first.trim_matches(|c| c == '[' || c == ']');
    let (date_part, time_part) = date.split_at(10);
    let date_parts : Vec<&str>= date_part.split("-").collect();

    let time_parts = time_part.split(":").collect::<Vec<&str>>();

    (
        date_parts[0].parse::<i16>().expect("not able to parse year"),
        date_parts[1].parse::<i8>().expect("not able to parse month"), 
        date_parts[2].parse::<i8>().expect("not able to parse date"),
        time_parts[0].trim().parse::<i8>().expect("not able to parse hour"),
        time_parts[1].parse::<i8>().expect("not able to parse minute")
    )
}


fn main() {
    let file: String = fs::read_to_string("../../data/day4_data.txt")
        .expect("couldn't open file");
    let lines = file.split("\r\n");
    let lines_iter : Vec<&str> = lines.collect();

    let mut current_id = -1;
    let mut last_hour = -1;
    // let mut is_sleeping : bool = false;

    let mut guardMap: HashMap<i32, HashMap<i8, i32>> = HashMap::new();

    assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");

    let mut foo = lines_iter.clone();
    foo.sort_by(|a, b| {
        let first_timestamp = get_date_as_type(a);
        let second_timestamp = get_date_as_type(b);

        first_timestamp.0.cmp(&second_timestamp.0)
            .then(first_timestamp.1.cmp(&second_timestamp.1))
            .then(first_timestamp.2.cmp(&second_timestamp.2))
            .then(first_timestamp.3.cmp(&second_timestamp.3))
            .then(first_timestamp.4.cmp(&second_timestamp.4))
    });

    for line in &foo {
        println!("{}", line)
    }

    for line in &foo {
        let time_and_instruction: Vec<&str> = line.split("] ")
            .collect();

        let (year, month, day, hour, minute) = get_date_as_type(line);
        
        match time_and_instruction[1].chars().next().unwrap() {
            'G' => {
                let index_part: Vec<&str> = time_and_instruction[1].split(" #")
                    .collect();
                let index: Vec<&str> = index_part[1].split(" ").collect();
                let id : i32 = index[0].parse()
                    .expect("being able to parse");

                guardMap.entry(id).or_insert(HashMap::new());
                current_id = id;
                
            },
            'f' => {
                // is_sleeping = true;
            },
            'w' => {
                // if !is_sleeping {
                //     break;
                // }
                let entry = guardMap.entry(current_id).or_insert(HashMap::new());
                let mut first_hour = last_hour;

                if last_hour >= hour {
                    for h in last_hour..= 60 {
                        entry.entry(h)
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                    }
                    first_hour = 0;

                }

                for h in first_hour..hour {
                    entry.entry(h)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }
            _ => println!("not known")
        }

        last_hour = hour;
    }

    let mut max = 0;
    let mut id_max = 0;
    for (id, hours) in &guardMap {
        let mut sum = 0;
        for (_, count) in hours {
            sum += count;
        }
        if sum > max {
            max = sum;
            id_max = *id;
        }
    }
    let mut max_count = 0;
    let mut max_hour : i8 = -1;
    for (h, count) in guardMap.entry(id_max).or_default() {
        if *count > max_count {
            max_count = *count;
            max_hour = *h;
        }
    }

    println!("Part 1: {}", max_hour)


}
