use std::{fs, collections::{HashMap, HashSet}};

use anyhow::{Result, anyhow};

fn main() -> Result<()>
{
    let input = fs::read_to_string("../data/day13_data.txt")?;
    let mut name_idx_lookup = HashMap::new();
    let mut gains = HashMap::new();
    let mut neighbors = HashMap::new();
    let mut idx = 0;

    for line in input.lines() {
        println!("{}", line);
        let parts : Vec<&str> = line.split(" ").collect();
        let name = *parts.get(0).ok_or(anyhow!("{line}"))?;
        let neighbor = parts.get(10).ok_or(anyhow!("{line}"))?
            .trim_end_matches('.');

        let name_idx = *name_idx_lookup.entry(name)
            .or_insert_with(|| {
                idx+= 1;
                idx
            });
        let neighbor_idx = *name_idx_lookup.entry(neighbor)
        .   or_insert_with(|| {
                idx+= 1;
                idx
            });
        neighbors
            .entry(name_idx)
            .or_insert_with(|| vec![])
            .push(neighbor_idx);

        let mut unit: i64 = parts.get(3).ok_or(anyhow!("{line}"))?
            .parse()?;
        unit = match *parts.get(2).ok_or(anyhow!("{line}"))? {
            "gain" => unit,
            "lose" => -1 * unit,
            _ => return Err(anyhow!("gain/lose: {line}"))
        };

        gains.insert((name_idx, neighbor_idx), unit);
    }

    // let mut queue: Vec<State> = vec![];
    // for (_, idx) in &name_idx_lookup {
    //     queue.push(State { start: *idx, next: *idx, visited: HashSet::from([*idx]), total: 0 });
    // }

    let mut queue: Vec<State> = vec![State { start: 0, next: 0, visited: HashSet::from([0]), total: 0 }];
    neighbors.insert(0, neighbors.keys().cloned().collect::<Vec<i64>>());

    // let final_person_count = &name_idx_lookup.len();
    let final_person_count = &name_idx_lookup.len() + 1;
    let mut max_total = i64::MIN;
    println!("gains: {:?}", gains);

    while let Some(next) = queue.pop() {

        if next.visited.len() == final_person_count {
            // let first = *gains.get(&(next.next, next.start)).ok_or_else(|| anyhow!("error neighbors: {:?}", next))?;
            // let second = *gains.get(&(next.start, next.next)).ok_or_else(|| anyhow!("error neighbors: {:?}", next))?;
            // let total = next.total + first + second;
            let total = next.total;

            if total > max_total {
                max_total = total;
            }
            continue;
        }

        for neighbor in neighbors.get(&next.next).ok_or(anyhow!("issue with state: {:?}", next))? {
            if next.visited.contains(&neighbor) {
                continue;
            }
            let (first, second) = if next.next != 0 {
                (*gains.get(&(next.next, *neighbor)).ok_or_else(|| anyhow!("error neighbors: {:?}", next))?,
                *gains.get(&(*neighbor, next.next)).ok_or_else(|| anyhow!("error neighbors: {:?}", next))?)
            } else {
                (0,0)
            };

            // let first = *gains.get(&(next.next, *neighbor)).ok_or_else(|| anyhow!("error neighbors: {:?}", next))?;
            // let second = *gains.get(&(*neighbor, next.next)).ok_or_else(|| anyhow!("error neighbors: {:?}", next))?;
            let total = next.total + first + second;

            let mut cloned_visisted = next.visited.clone();
            cloned_visisted.insert(*neighbor);

            let state = State{start: next.start, next: *neighbor, visited: cloned_visisted, total };
            queue.push(state);
        }
    }

    println!("Part 1: {}", max_total);

    Ok(())
}

#[derive(Debug)]
struct State {
    start: i64,
    next: i64,
    visited: HashSet<i64>,
    total: i64,
}
