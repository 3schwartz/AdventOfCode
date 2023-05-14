use std::{fs, collections::BTreeSet};
use anyhow::{Result, Ok, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day17_data.txt")?;

    let combinations = pack(&input, 150)?;

    println!("Part 1: {}", combinations.len());

    let min_lenght = combinations.iter()
        .map(|set| set.len())
        .min()
        .ok_or(anyhow!("not able to find length"))?;
    let min_length_sets: u32 = combinations
        .iter()
        .filter(|set| set.len() == min_lenght)
        .map(|_| 1)
        .sum();

    println!("Part 2: {}", min_length_sets);

    Ok(())
}


fn pack(input: &str, liter: u32) -> Result<BTreeSet<BTreeSet<usize>>>{
    let mut queue = BTreeSet::new();
    let mut containers = vec![];

    for (idx, line) in input.lines().enumerate() {
        let container: u32 = line.parse()?;
        containers.push((idx, container));
    }

    queue.insert((containers, BTreeSet::new(), 0));

    let mut combinations = BTreeSet::new();
    let mut visited: BTreeSet<BTreeSet<usize>> = BTreeSet::new();

    while let Some(comb) = queue.pop_first() {
        let cont = comb.0;
        let idxs = comb.1;
        let sum = comb.2;

        for (i, (idx, container)) in cont.iter().enumerate() {
            let sum_updated = container + sum;

            if sum_updated > liter {
                continue;
            }
            
            let mut idxs_cloned = idxs.clone();
            idxs_cloned.insert(*idx);

            if sum_updated < liter && !visited.contains(&idxs_cloned) {
                let mut cont_cloned = cont.clone();
                cont_cloned.remove(i);

                queue.insert((cont_cloned, idxs_cloned.clone(), sum_updated));
                visited.insert(idxs_cloned);
                continue;
            }

            if sum_updated == liter {
                combinations.insert(idxs_cloned);
            }
        };
    }

    Ok(combinations)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::{BTreeSet, BTreeMap};

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../data/day17_test_data.txt")?;

        // Act
        let combinations = pack(&input, 25)?;

        // Assert
        assert_eq!(combinations.len(), 4);

        Ok(())
    }

    #[test]
    fn test_vec() {
        let first = vec![1,2];
        let second = vec![1,2];

        let mut map_set = BTreeSet::new();

        let first_i = map_set.insert(first);
        let second_i = map_set.insert(second);

        assert!(first_i);
        assert!(!second_i);
    }

    #[test]
    fn test_bmap() {
        let mut map_set = BTreeSet::new();

        let map_first = BTreeMap::from([(1,1), (2,4)]);
        let map_second = BTreeMap::from([(1,1), (2,4)]);
        let map_third = BTreeMap::from([(1,1), (2,5)]);

        let first_i = map_set.insert(map_first);
        let second_i = map_set.insert(map_second);
        let third_i = map_set.insert(map_third);

        assert!(first_i);
        assert!(!second_i);
        assert!(third_i);
        assert_eq!(map_set.len(), 2);
    }

    #[test]
    fn test_btree() {
        let mut btree_set = BTreeSet::new();

        let btree_first = BTreeSet::from([1,2]);
        let btree_second = BTreeSet::from([1,2]);
        let btree_third = BTreeSet::from([1,2,1]);

        let first_insert = btree_set.insert(btree_first);
        let second_insert = btree_set.insert(btree_second);
        let third_insert = btree_set.insert(btree_third);

        let length = btree_set.len();
        assert!(first_insert);
        assert!(!second_insert);
        assert!(!third_insert);
        assert_eq!(length, 1);
    }
}