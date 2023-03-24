use std::{fs, collections::{HashSet, HashMap}};

fn main() {
    let file = fs::read_to_string("../data/day3_data.txt").unwrap();

    let santa_visited = part_1(&file);
   
    println!("Part 1: {}", santa_visited);

    let helpers_visited = part_2(&file, 2);

    println!("Part 2: {}", helpers_visited);
}

struct Helper {
    state: (i32, i32),
    visited: HashSet<(i32, i32)>
}

fn part_2(file: &str, helpers: usize) -> usize {
    let mut states = HashMap::new();

    for (i, c) in file.chars().enumerate() {
        let idx = i % helpers;
        let helper = states.entry(idx)
            .or_insert(Helper { state: (0,0), visited: HashSet::new() });
        helper.state = lookup(c, helper.state);
        helper.visited.insert(helper.state);
    }

    let union = states.drain()
        .fold(HashSet::new(), |acc, (_, helper)| {
            acc.union(&helper.visited).cloned().collect()
        });

    return union.len();
}

fn part_1(file: &str) -> usize {
    let mut visited = HashSet::new();
    let mut state = (0,0);
    visited.insert(state);

    for c in file.chars() {
        state = lookup(c, state);
        visited.insert(state);
    }
    
    visited.len()
}


fn lookup(c: char, state: (i32, i32)) -> (i32, i32) {
    match c {
        '>' => (state.0, state.1 + 1),
        'v' => (state.0 - 1, state.1),
        '<' => (state.0, state.1 - 1),
        '^' => (state.0 + 1, state.1),
        _ => panic!("{}", c)
    }
}