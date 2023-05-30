use std::{collections::BTreeSet, fs};

use anyhow::Result;

fn dfs(
    states: [BTreeSet<u32>; 3],
    mut missing: BTreeSet<u32>,
    split: u32,
    smallest: u32,
    visited: &mut BTreeSet<(BTreeSet<BTreeSet<u32>>, BTreeSet<u32>)>,
) -> u32 {
    let compare = BTreeSet::from(states.clone());
    if !visited.insert((compare, missing.clone())) {
        return u32::MAX;
    }

    let Some(next) = missing.pop_first() else { return u32::MAX; };

    let mut smallest_current = smallest;

    let first_sum = states[0].iter().sum::<u32>();
    let second_sum = states[1].iter().sum::<u32>();
    let third_sum = states[2].iter().sum::<u32>();

    if first_sum == split && first_sum == second_sum && second_sum == third_sum {
        let mut min_len = states[0].len();
        let mut min_idx = 0;
        for i in 0..3 {
            if states[i].len() < min_len {
                min_idx = i;
                min_len = states[i].len();
            }
        }
        return states[min_idx].iter().fold(1, |acc, &num| acc * num);
    }

    if first_sum + next > split && first_sum != split
        || second_sum + next > split && second_sum != split
        || third_sum + next > split && third_sum != split
    {
        return smallest_current;
    }

    println!("{}, {}, {}, {}", first_sum, second_sum, third_sum, next);

    if first_sum + next < split {
        let mut cloned = states.clone();
        cloned[0].insert(next);
        let qe = dfs(cloned, missing.clone(), split, smallest_current, visited);
        if qe < smallest_current {
            smallest_current = qe;
        }
    }

    if second_sum + next < split {
        let mut cloned = states.clone();
        cloned[1].insert(next);
        let qe = dfs(cloned, missing.clone(), split, smallest_current, visited);
        if qe < smallest_current {
            smallest_current = qe;
        }
    }
    if third_sum + next < split {
        let mut cloned = states.clone();
        cloned[2].insert(next);
        let qe = dfs(cloned, missing.clone(), split, smallest_current, visited);
        if qe < smallest_current {
            smallest_current = qe;
        }
    }

    return smallest_current;
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;

    let presents: BTreeSet<u32> = input.lines().flat_map(|l| l.parse()).collect();

    let sum = presents.iter().sum::<u32>();
    let value: u32 = sum / 3;

    let qe = dfs(
        [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()],
        presents,
        value,
        u32::MAX,
        &mut BTreeSet::new(),
    );

    println!("Part 1: {}", qe);

    Ok(())
}
