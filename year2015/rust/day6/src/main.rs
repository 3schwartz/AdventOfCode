use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("../data/day6_data.txt").unwrap();

    let mut lights: HashMap<(u16, u16), bool> = HashMap::new();
    let mut brigtness: HashMap<(u16, u16), u128> = HashMap::new();

    for line in input.lines() {
        let (start, end, action) = get_action(line);

        for x in start[0]..=end[0] {
            for y in start[1]..=end[1] {
                let entry = lights.entry((x, y)).or_insert(false);

                match action {
                    Action::On => *entry = true,
                    Action::Off => *entry = false,
                    Action::Toggle => *entry = !*entry,
                }

                let brigth = brigtness.entry((x, y)).or_insert(0);

                match action {
                    Action::On => *brigth += 1,
                    Action::Off => *brigth = brigth.checked_sub(1).unwrap_or(0),
                    Action::Toggle => *brigth += 2,
                }
            }
        }
    }

    let lights_on = lights.iter().filter(|(_, &value)| value).count();

    println!("Part 1: {lights_on}");

    let total_brightness: u128 = brigtness
        .iter()
        .map(|(_, v)| v)
        .sum();

    println!("Part 2: {total_brightness}");
}

fn get_action(line: &str) -> (Vec<u16>, Vec<u16>, Action) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if line.starts_with("turn on") || line.starts_with("turn off") {
        let start: Vec<u16> = parts[2]
            .split(",")
            .filter_map(|v| v.parse::<u16>().ok())
            .collect();
        let end: Vec<u16> = parts[4]
            .split(",")
            .filter_map(|v| v.parse::<u16>().ok())
            .collect();

        return (
            start,
            end,
            if line.starts_with("turn on") {
                Action::On
            } else {
                Action::Off
            },
        );
    }

    let start: Vec<u16> = parts[1]
        .split(",")
        .filter_map(|v| v.parse::<u16>().ok())
        .collect();

    let end: Vec<u16> = parts[3]
        .split(",")
        .filter_map(|v| v.parse::<u16>().ok())
        .collect();

    (start, end, Action::Toggle)
}

enum Action {
    On,
    Off,
    Toggle,
}
