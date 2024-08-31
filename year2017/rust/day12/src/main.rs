use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day12_data.txt")?;

    let connection_map = create_connection_map(&input)?;

    let group_0 = find_group_members(0, &connection_map);

    println!("Part 1: {}", group_0.len());

    let group_count = find_group_count(&connection_map);

    println!("Part 2: {}", group_count);
    Ok(())
}

fn find_group_count(connection_map: &HashMap<u32, HashSet<u32>>) -> u32 {
    let mut visited = HashSet::<u32>::new();
    let mut group_count = 0;
    for start in connection_map.keys() {
        if visited.contains(start) {
            continue;
        }
        group_count += 1;
        let group_visisted = find_group_members(*start, connection_map);
        visited.extend(group_visisted);
    }
    group_count
}

fn find_group_members(start: u32, connection_map: &HashMap<u32, HashSet<u32>>) -> HashSet<u32> {
    let mut visited = HashSet::<u32>::new();
    let mut queue: Vec<u32> = Vec::from([start]);

    while let Some(next) = queue.pop() {
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);

        if let Some(connections) = connection_map.get(&next) {
            for connection in connections {
                queue.push(*connection);
            }
        } else {
            continue;
        }
    }
    visited
}

fn create_connection_map(input: &str) -> Result<HashMap<u32, HashSet<u32>>> {
    let mut connection_map: HashMap<u32, HashSet<u32>> = HashMap::<u32, HashSet<u32>>::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" <-> ").collect();

        let connections: Vec<u32> = parts[1]
            .split(", ")
            .map(|c| c.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;

        let from_connection = parts[0].parse::<u32>()?;

        for connection in connections {
            connection_map
                .entry(from_connection)
                .and_modify(|k| {
                    k.insert(connection);
                })
                .or_insert_with(|| HashSet::from([connection]));

            connection_map
                .entry(connection)
                .and_modify(|k| {
                    k.insert(from_connection);
                })
                .or_insert_with(|| HashSet::from([from_connection]));
        }
    }

    Ok(connection_map)
}
