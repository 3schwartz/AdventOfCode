use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use anyhow::Result;

fn add_type_to_modules(modules: &Vec<String>, types: &HashMap<String, String>) -> Vec<String> {
    let mut updates: Vec<String> = Vec::with_capacity(modules.len());
    for module in modules {
        if let Some(module_type) = types.get(module) {
            updates.push(format!("{}{}", module_type, module))
        } else {
            updates.push(module.to_string());
        }
    }
    updates
}

fn get_modules_and_types(input: &str) -> (HashMap<String, Vec<String>>, HashMap<String, String>) {
    let mut modules = HashMap::new();
    let mut types = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let destinations: Vec<String> = parts[1].split(", ").map(|s| s.to_string()).collect();
        types.insert(parts[0][1..].to_string(), parts[0][..1].to_string());
        modules.insert(parts[0].to_string(), destinations);
    }
    (modules, types)
}

#[allow(clippy::type_complexity)]
fn get_connections_opposites(
    modules: &mut HashMap<String, Vec<String>>,
    types: &HashMap<String, String>,
) -> (
    HashMap<String, HashMap<String, String>>,
    HashMap<String, Vec<String>>,
) {
    let mut opposite: HashMap<String, Vec<String>> = HashMap::new();
    let mut connections: HashMap<String, HashMap<String, String>> = HashMap::new();
    for (source, destinations) in modules {
        *destinations = add_type_to_modules(destinations, types);
        for destination in destinations {
            if &destination[..1] == "&" {
                connections
                    .entry(destination.clone())
                    .and_modify(|m| {
                        m.insert(source.to_string(), "lo".to_string());
                    })
                    .or_insert(HashMap::from([(source.to_string(), "lo".to_string())]));
            }
            opposite
                .entry(destination.to_string())
                .and_modify(|d| d.push(source.to_string()))
                .or_insert(Vec::from([source.to_string()]));
        }
    }
    (connections, opposite)
}

/// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// https://en.wikipedia.org/wiki/Least_common_multiple
fn common_lcm(lcms: Vec<i64>) -> i64 {
    let mut result = 1;
    for values in lcms {
        result = (result * values) / gcd(values, result)
    }
    result
}

fn iterate(
    modules: &mut HashMap<String, Vec<String>>,
    connections: &mut HashMap<String, HashMap<String, String>>,
    conjuction_connections: HashSet<String>,
    part_1: bool,
) -> i64 {
    let mut queue = VecDeque::new();
    let mut low_pulse = 0;
    let mut high_pulse = 0;
    let mut flip_flop_on = HashSet::new();

    let mut conjuction_visisted_count = HashMap::new();
    let mut conjuction_previous_visisted = HashMap::new();

    let mut lcms = vec![];

    let mut iterations: i64 = 0;
    loop {
        iterations += 1;

        queue.push_back(("broadcaster", "button", "lo"));

        while let Some((to, from, pulse)) = queue.pop_front() {
            if !part_1 && pulse == "lo" {
                if conjuction_connections.contains(to)
                    && conjuction_previous_visisted.contains_key(to)
                    && conjuction_visisted_count.get(to).unwrap() == &2
                {
                    lcms.push(iterations - conjuction_previous_visisted.get(to).unwrap())
                }
                conjuction_previous_visisted.insert(to, iterations);
                conjuction_visisted_count
                    .entry(to)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            if !part_1 && lcms.len() == conjuction_connections.len() {
                return common_lcm(lcms);
            }

            if pulse == "lo" {
                low_pulse += 1;
            } else {
                high_pulse += 1;
            }
            if !modules.contains_key(to) {
                continue;
            }
            let next_pulse = if to == "broadcaster" {
                pulse
            } else if &to[..1] == "%" {
                if pulse == "hi" {
                    continue;
                }
                match flip_flop_on.contains(to) {
                    true => {
                        flip_flop_on.remove(to);
                        "lo"
                    }
                    false => {
                        flip_flop_on.insert(to);
                        "hi"
                    }
                }
            } else if &to[..1] == "&" {
                let connection = connections.get_mut(to).unwrap();
                let connection_pulse = connection.get_mut(from).unwrap();
                *connection_pulse = pulse.to_string();

                if connection.values().all(|p| p == "hi") {
                    "lo"
                } else {
                    "hi"
                }
            } else {
                panic!("{}", to)
            };
            for next in modules.get(to).unwrap() {
                queue.push_back((next, to, next_pulse));
            }
        }

        if iterations == 1_000 && part_1 {
            return low_pulse * high_pulse;
        }
    }
}

/// 'rx' depends on a conjuction module. Hence all connections
/// needs to be in state 'high pulse' before a low is send to 'rx'.
fn find_conjuction_connections(opposite: HashMap<String, Vec<String>>) -> HashSet<String> {
    opposite
        .get(&opposite.get("rx").unwrap()[0])
        .unwrap()
        .iter()
        .cloned()
        .collect()
}

fn actions(input: &str, part_1: bool) -> i64 {
    let (mut modules, types) = get_modules_and_types(input);
    let (mut connections, opposite) = get_connections_opposites(&mut modules, &types);
    let conjuction_connections = find_conjuction_connections(opposite);
    iterate(
        &mut modules,
        &mut connections,
        conjuction_connections,
        part_1,
    )
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day20_data.txt")?;
    let part_1 = actions(&input, true);

    println!("Part 1: {}", part_1);

    let part_2 = actions(&input, false);
    println!("Part 2: {}", part_2);
    Ok(())
}
