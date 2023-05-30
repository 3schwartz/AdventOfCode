use std::{fs, collections::BTreeSet};

use anyhow::Result;

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq, Clone)]
struct State {
    places: [BTreeSet<u32>; 3]
}

impl State {
    fn get_overview(&self) -> BTreeSet<BTreeSet<u32>> {
        return BTreeSet::from([self.places[0].clone(),self.places[1].clone(),self.places[2].clone()])
    }

    fn any_sum_above(&self, max: u32) -> bool {
        return self.places.iter().any(|s| s.iter().sum::<u32>() > max)
    }

    fn is_valid(&self) -> bool {
        let first: u32 = self.places[0].iter().sum();
        for set in &self.places {
            let sum: u32 = set.iter().sum();
            if first != sum {
                return false;
            }
        }
        return true
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;

    let presents: Vec<u32> = input.lines()
        .flat_map(|l| l.parse())
        .collect();

    let max: u32 = presents.iter().sum::<u32>() / 3;

    let state = State{places: [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()]};
    let mut states = BTreeSet::from([state]);
    let mut visited = BTreeSet::new();

    for present in presents {
        let mut new_state = BTreeSet::new();

        for set in &states {
            if !visited.insert(set.get_overview()) {
                continue;
            }
            if set.any_sum_above(max) {
                continue;
            }
            for i in 0..3 {
                let mut cloned = set.clone();
                let cloned_group = &mut cloned.places[i];
                cloned_group.insert(present);
                new_state.insert(cloned);
            }
        }
        states = new_state;
    }

    let mut valids = BTreeSet::new();
    for state in states {
        if state.is_valid() {
            valids.insert(state);
        }
    }

    println!("{:?}", valids);

    Ok(())
}
