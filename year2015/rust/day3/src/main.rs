use std::{fs, collections::{HashSet, HashMap}};

fn main() {
    let file = fs::read_to_string("../data/day3_data.txt").unwrap();

    let santa_visited = solution(&file, 1);
   
    println!("Part 1: {}", santa_visited);

    let helpers_visited = solution(&file, 2);

    println!("Part 2: {}", helpers_visited);
}

struct Helper {
    state: (i32, i32),
    visited: HashSet<(i32, i32)>
}

fn solution(file: &str, helpers: usize) -> usize {
    let mut states = HashMap::new();

    for (i, c) in file.chars().enumerate() {
        let idx = i % helpers;
        let helper = states.entry(idx)
            .or_insert(Helper { state: (0,0), visited: HashSet::new() });
        helper.state = lookup(c, helper.state);
        helper.visited.insert(helper.state);
    }

    let mut union = HashSet::new();
    for (_, helper) in states.drain() {
        union.extend(helper.visited);
    }
    
    return union.len();
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