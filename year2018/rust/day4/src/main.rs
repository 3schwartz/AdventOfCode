use std::{fs, collections::HashMap};

fn main() {
    let file: String = fs::read_to_string("../../data/day4_data.txt")
        .expect("couldn't open file");
    let lines = file.split("\r\n");
    let mut lines_iter : Vec<&str> = lines.collect();

    sort_lines(&mut lines_iter);
    print_lines(&lines_iter);

    let mut current_id = -1;
    let mut last_minute = -1;
    let mut guard_map: HashMap<i32, HashMap<i16, i32>> = HashMap::new();

    for line in &lines_iter {
        let time_and_instruction: Vec<&str> = line.split("] ")
            .collect();

        let (_, _, _, _, minute) = get_date_as_type(line);
        
        match time_and_instruction[1].chars().next().unwrap() {
            'G' => {
                let index_part: Vec<&str> = time_and_instruction[1].split(" #")
                    .collect();
                let index: Vec<&str> = index_part[1].split(" ").collect();
                let id : i32 = index[0].parse()
                    .expect("being able to parse");

                guard_map.entry(id).or_insert(HashMap::new());
                current_id = id;
                
            },
            'w' => {
                let entry = guard_map.entry(current_id).or_insert(HashMap::new());

                for h in last_minute..minute {
                    entry.entry(h)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }
            _ => ()
        }

        last_minute = minute;
    }

    let id_max = find_max_id(&guard_map);
    let max_minute = find_max_minute(id_max, &guard_map);

    println!("Part 1: {}", id_max * max_minute);

    let (id_freq_max, minute_max_freq) = find_max_frequency(&guard_map);

    println!("Part 2: {}", id_freq_max * minute_max_freq)
}

fn find_max_frequency(guard_map : &HashMap<i32, HashMap<i16, i32>>) -> (i32, i32) {
    let mut id_max = 0;
    let mut minute_asleep_count_max = 0;
    let mut minute_max: i16 = 0;
    for (id, guard) in guard_map {
        for (minute, count) in guard {
            if *count > minute_asleep_count_max {
                minute_asleep_count_max = *count;
                id_max = *id;
                minute_max = *minute;
            }
        }
    }
    (id_max, minute_max as i32)
}

fn print_lines(lines : &Vec<&str>) -> () {
    for line in lines {
        println!("{}", line)
    }
}

fn sort_lines(lines: &mut Vec<&str>) -> () {
    lines.sort_by(|a, b| {
        let first_timestamp = get_date_as_type(a);
        let second_timestamp = get_date_as_type(b);

        first_timestamp.0.cmp(&second_timestamp.0)
            .then(first_timestamp.1.cmp(&second_timestamp.1))
            .then(first_timestamp.2.cmp(&second_timestamp.2))
            .then(first_timestamp.3.cmp(&second_timestamp.3))
            .then(first_timestamp.4.cmp(&second_timestamp.4))
    });
}

fn find_max_id(guard_map : &HashMap<i32, HashMap<i16, i32>>) -> i32 {
    let mut max = 0;
    let mut id_max = 0;
    for (id, minute) in guard_map {
        let mut sum = 0;
        for (_, count) in minute {
            sum += count;
        }
        if sum > max {
            max = sum;
            id_max = *id;
        }
    }
    id_max
}

fn find_max_minute(id_max : i32, guard_map : &HashMap<i32, HashMap<i16, i32>>) -> i32 {
    let max_id_entry = guard_map.get(&id_max)
        .expect("should exist");
    let mut max_count = 0;
    let mut max_minute = -1;
    for (h, count) in max_id_entry {
        if *count > max_count {
            max_count = *count;
            max_minute = *h;
        }
    }
    max_minute as i32
}

fn get_date_as_type(line : &str) -> (i16, i16, i16, i16, i16){
    let (first, _) = line.split_at(18);
    let date = first.trim_matches(|c| c == '[' || c == ']');
    let (date_part, time_part) = date.split_at(10);
    let date_parts : Vec<&str>= date_part.split("-").collect();

    let time_parts = time_part.split(":").collect::<Vec<&str>>();

    (
        date_parts[0].parse::<i16>().expect("not able to parse year"),
        date_parts[1].parse::<i16>().expect("not able to parse month"), 
        date_parts[2].parse::<i16>().expect("not able to parse date"),
        time_parts[0].trim().parse::<i16>().expect("not able to parse hour"),
        time_parts[1].parse::<i16>().expect("not able to parse minute")
    )
}
