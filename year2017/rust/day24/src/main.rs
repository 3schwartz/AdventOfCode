use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

use anyhow::{anyhow, Result};

struct Bridge<'a> {
    ports: [&'a str; 2],
}

impl<'a> Bridge<'a> {
    fn other(&self, port: &'a str) -> &str {
        if self.ports[0] == port {
            self.ports[1]
        } else {
            self.ports[0]
        }
    }
}

impl<'a> TryFrom<&Vec<&'a str>> for Bridge<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<&'a str>) -> std::result::Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(anyhow!("wrong length: {}", value.len()));
        }
        Ok(Self {
            ports: [value[0], value[1]],
        })
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;
    // let input = fs::read_to_string("../data/day24_test_data.txt")?;

    let mut port_bridge_links = HashMap::<&str, HashSet<usize>>::new();
    let mut bridges = HashMap::<usize, Bridge>::new();
    for (i, line) in input.lines().enumerate() {
        let parts = line.split('/').collect::<Vec<&str>>();

        let connection = Bridge::try_from(&parts)?;
        bridges.insert(i, connection);

        port_bridge_links
            .entry(parts[0])
            .and_modify(|e| {
                e.insert(i);
            })
            .or_insert_with(|| HashSet::from([i]));
        port_bridge_links
            .entry(parts[1])
            .and_modify(|e| {
                e.insert(i);
            })
            .or_insert_with(|| HashSet::from([i]));
    }

    let start_port = "0";
    let start_bridges = port_bridge_links
        .get(start_port)
        .ok_or_else(|| anyhow!("missing links: {}", start_port))?;

    let mut queue = Vec::new();
    for bridge_idx in start_bridges {
        let bridge = bridges
            .get(bridge_idx)
            .ok_or_else(|| anyhow!("missing bridge idx {}", bridge_idx))?;
        let other = bridge.other(start_port);
        let seen = BTreeSet::from([bridge_idx]);
        queue.push((other, seen));
    }

    let mut cache = BTreeSet::new();
    let mut final_bridges = vec![];
    while let Some((next, seen)) = queue.pop() {
        if !cache.insert((next, seen.clone())) {
            continue;
        }
        let bridge_idxs = port_bridge_links
            .get(next)
            .ok_or_else(|| anyhow!("missing links: {}", next))?;

        let missing: HashSet<&usize> = bridge_idxs.iter().filter(|b| !seen.contains(b)).collect();
        if missing.is_empty() {
            final_bridges.push(seen);
            continue;
        }

        for bridge_idx in missing {
            let bridge = bridges
                .get(bridge_idx)
                .ok_or_else(|| anyhow!("missing bridge idx {}", bridge_idx))?;
            let other = bridge.other(next);
            let mut seen_cloned = seen.clone();
            seen_cloned.insert(bridge_idx);
            queue.push((other, seen_cloned));
        }
    }
    let max = final_bridges
        .iter()
        .map(|b| get_sum(b, &bridges).map(|sum| (b, sum)))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .max_by_key(|&(_, sum)| sum)
        .map(|(_, b)| b)
        .ok_or_else(|| anyhow!("missing max"))?;

    println!("Part 1: {max}");

    let max_length = final_bridges
        .iter()
        .map(|b| b.len())
        .max()
        .ok_or_else(|| anyhow!("not able to find max length"))?;

    let longest_max = final_bridges
        .iter()
        .filter(|b| b.len() == max_length)
        .map(|b| get_sum(b, &bridges).map(|sum| (b, sum)))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .max_by_key(|&(_, sum)| sum)
        .map(|(_, b)| b)
        .ok_or_else(|| anyhow!("missing max"))?;

    println!("Part 2: {longest_max}");

    Ok(())
}

fn get_sum(seen: &BTreeSet<&usize>, bridges: &HashMap<usize, Bridge>) -> Result<u32> {
    let mut sum = 0;
    for bridge_idx in seen {
        let bridge = bridges
            .get(bridge_idx)
            .ok_or_else(|| anyhow!("missing seen bridge idx {}", bridge_idx))?;
        sum += bridge.ports[0].parse::<u32>()?;
        sum += bridge.ports[1].parse::<u32>()?;
    }
    Ok(sum)
}
