use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day12_data.txt")?;

    let connection_map = create_connection_map(&input)?;

    let mut visited = HashSet::<u32>::new();
    let mut queue: Vec<u32> = Vec::from([0]);
    loop {
        if let Some(next) = queue.pop() {
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
        } else {
            break;
        }
    }

    println!("Part 1: {}", visited.len());
    Ok(())
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

    return Ok(connection_map);
}
