use anyhow::Result;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day23_data.txt")?;
    let connections = make_connections(&input);
    let sets = make_sets(&connections);
    let filtered = sets
        .iter()
        .filter(|s| s.len() == 3)
        .filter(|s| s.iter().any(|n| n.starts_with('t')))
        .count();

    println!("Part 1: {filtered}");

    let best = find_best(&connections);

    println!("Part 2: {best}");

    Ok(())
}

fn make_connections(input: &str) -> HashMap<String, HashSet<String>> {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let parts = line.split('-').collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        connections
            .entry(parts[0].to_string())
            .and_modify(|c| {
                c.insert(parts[1].to_string());
            })
            .or_insert_with(|| HashSet::from([parts[1].to_string()]));
        connections
            .entry(parts[1].to_string())
            .and_modify(|c| {
                c.insert(parts[0].to_string());
            })
            .or_insert_with(|| HashSet::from([parts[0].to_string()]));
    }
    connections
}

fn make_sets(connections: &HashMap<String, HashSet<String>>) -> BTreeSet<BTreeSet<&String>> {
    let indexable: Vec<(&String, &HashSet<String>)> = connections.iter().collect();

    let mut sets = BTreeSet::new();

    for (x, (s_1, _)) in indexable.iter().enumerate() {
        for y in x + 1..indexable.len() {
            for z in y + 1..indexable.len() {
                let (s_2, set_2) = indexable[y];
                let (s_3, set_3) = indexable[z];
                if set_2.contains(*s_1) && set_3.contains(*s_1) && set_3.contains(s_2) {
                    sets.insert(BTreeSet::from([s_1, s_2, s_3]));
                }
            }
        }
    }
    sets
}

fn find_best(connections: &HashMap<String, HashSet<String>>) -> String {
    let mut groups: BTreeSet<BTreeSet<&String>> = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for (k, d) in connections {
        let mut queue = VecDeque::from([(d, BTreeSet::from([k]))]);
        while let Some((prior_connects, prior_group)) = queue.pop_front() {
            if !seen.insert(prior_group.clone()) {
                continue;
            }
            for prior_connect in prior_connects {
                let next_connects = connections.get(prior_connect).unwrap();
                if !prior_group.iter().all(|pg| next_connects.contains(*pg)) {
                    continue;
                }
                let mut cloned = prior_group.clone();
                cloned.insert(prior_connect);

                for nexct_connect in next_connects {
                    if prior_group.contains(nexct_connect) {
                        groups.insert(cloned.clone());
                        continue;
                    }
                }
                queue.push_back((next_connects, cloned));
            }
        }
    }
    let max_group = groups.iter().max_by_key(|group| group.len()).unwrap();
    max_group
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day23_test_data.txt")?;
        // Act

        let connections = make_connections(&input);
        let sets = make_sets(&connections);
        let filtered = sets
            .iter()
            .filter(|s| s.len() == 3)
            .filter(|s| s.iter().any(|n| n.starts_with('t')))
            .count();

        // Assert
        assert_eq!(filtered, 7);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day23_test_data.txt")?;
        // Act

        let connections = make_connections(&input);
        let best = find_best(&connections);

        // Assert
        assert_eq!(best, "co,de,ka,ta");
        Ok(())
    }
}
