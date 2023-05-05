use std::{fs, collections::{HashMap, HashSet}};

use anyhow::{Result, anyhow};

fn generate_lookups(input: &str) -> Result<(HashMap<&str, i32>, HashMap<(i32,i32), i32>, HashMap<i32, Vec<i32>>)> {
    let mut name_idx_lookup = HashMap::new();
    let mut gains = HashMap::new();
    let mut neighbors = HashMap::new();
    let mut idx = 0;

    for line in input.lines() {
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

        let mut unit: i32 = parts.get(3).ok_or(anyhow!("{line}"))?
            .parse()?;
        unit = match *parts.get(2).ok_or(anyhow!("{line}"))? {
            "gain" => unit,
            "lose" => -1 * unit,
            _ => return Err(anyhow!("gain/lose: {line}"))
        };

        gains.insert((name_idx, neighbor_idx), unit);
    }
    return Ok((name_idx_lookup, gains, neighbors))
}

fn generate_start_all(name_idx_lookup: &HashMap<&str, i32>) -> Vec<State> {
    let mut queue: Vec<State> = vec![];
    for (_, idx) in name_idx_lookup {
        queue.push(State { start: *idx, next: *idx, visited: HashSet::from([*idx]), total: 0 });
    }
    queue   
}

fn dfs(
    mut queue: Vec<State>,
    name_idx_lookup: &HashMap<&str, i32>,
    gains: &HashMap<(i32,i32), i32>,
    neighbors: &HashMap<i32, Vec<i32>>) 
    -> anyhow::Result<i32> {
    let final_person_count =  if queue.get(0).ok_or_else(|| anyhow!("queue is empty"))?.start == 0 { &name_idx_lookup.len() + 1 } else { name_idx_lookup.len()};
    let mut max_total = i32::MIN;

    while let Some(next) = queue.pop() {

        if next.visited.len() == final_person_count {
            let first = *gains.get(&(next.next, next.start)).unwrap_or(&0);
            let second = *gains.get(&(next.start, next.next)).unwrap_or(&0);
            let total = next.total + first + second;

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

            let total = next.total + first + second;

            let mut cloned_visisted = next.visited.clone();
            cloned_visisted.insert(*neighbor);

            let state = State{start: next.start, next: *neighbor, visited: cloned_visisted, total };
            queue.push(state);
        }
    };
    Ok(max_total)
}

fn main() -> Result<()>
{
    let input = fs::read_to_string("../data/day13_data.txt")?;
    let (name_idx_lookup, gains, mut neighbors) = generate_lookups(&input)?;
    
    // Part 1
    let all_start = generate_start_all(&name_idx_lookup);
    let part_1 = dfs(all_start, &name_idx_lookup, &gains, &neighbors)?;

    println!("Part 1: {}", part_1);

    // Part 2
    let queue: Vec<State> = vec![State { start: 0, next: 0, visited: HashSet::from([0]), total: 0 }];
    neighbors.insert(0, neighbors.keys().cloned().collect::<Vec<i32>>());
    let part_2 = dfs(queue, &name_idx_lookup, &gains, &neighbors)?;

    println!("Part 2: {}", part_2);

    Ok(())
}

#[derive(Debug)]
struct State {
    start: i32,
    next: i32,
    visited: HashSet<i32>,
    total: i32,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = fs::read_to_string("../../data/day13_test_data.txt")?;

        // Act
        let (name_idx_lookup, gains, neighbors) = generate_lookups(&input)?;
        let all_start = generate_start_all(&name_idx_lookup);
        let actual = dfs(all_start, &name_idx_lookup, &gains, &neighbors)?;

        // Assert
        assert_eq!(actual, 330);
        Ok(())
    }
}