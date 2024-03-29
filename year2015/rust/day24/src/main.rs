use core::iter::Iterator;
use std::fs;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;
    let presents: Vec<u64> = input.lines().flat_map(|l| l.parse()).collect();

    let groups = Groups::new(&presents, 3);
    let min_qe = groups.find_qe();

    println!("Part 1: {}", min_qe);

    let groups = Groups::new(&presents, 4);
    let min_qe = groups.find_qe();

    println!("Part 2: {}", min_qe);

    Ok(())
}

struct Groups {
    packages: Vec<u64>,
    split: u64,
    size: u64,
}

impl Groups {
    /// Sort array such that first group which is returned on iteration also is the one with the
    /// smallest product of it's elements.
    fn new(presents: &Vec<u64>, size: u64) -> Self {
        let split = presents.iter().sum::<u64>() / size;
        let mut cloned = presents.clone();
        cloned.sort();
        Self {
            packages: cloned,
            split,
            size,
        }
    }

    fn from(used: Vec<u64>, groups: &Groups) -> Self {
        let presents: Vec<u64> = groups
            .packages
            .iter()
            .filter(|v| !used.contains(v))
            .map(|v| *v)
            .collect();
        Self {
            packages: presents,
            split: groups.split,
            size: groups.size - 1,
        }
    }

    fn can_group(&self, group: Vec<u64>) -> bool {
        let groups = Groups::from(group, self);

        if groups.size == 0 {
            return true;
        }
        if groups.size == 1 {
            let ok = groups.packages.iter().sum::<u64>() == groups.split;
            return ok;
        }

        for group in &groups {
            if groups.can_group(group) {
                return true;
            }
        }

        return false;
    }

    /// Find quantum entanglement. Since the initial packages are sorted the first group which
    /// which is packable is also the one with the smallest product of elements.
    fn find_qe(&self) -> u64 {
        let mut min_qe = u64::MAX;

        for group in self {
            let qe = group.iter().fold(1, |acc, n| acc * n);
            if qe >= min_qe {
                continue;
            }

            let is_able_to_group = self.can_group(group);

            if !is_able_to_group {
                break;
            }

            min_qe = qe;
            break;
        }
        min_qe
    }
}

impl<'a> IntoIterator for &'a Groups {
    type Item = Vec<u64>;
    type IntoIter = GroupIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GroupIterator::new(self)
    }
}

struct GroupIterator<'a> {
    idx: usize,
    groups: &'a Groups,
    combinations: Option<Box<dyn Iterator<Item = Vec<u64>> + 'a>>,
}

impl<'a> GroupIterator<'a> {
    fn new(groups: &'a Groups) -> Self {
        Self {
            idx: 0,
            groups,
            combinations: None,
        }
    }
}

impl<'a> Iterator for GroupIterator<'a> {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.idx == self.groups.packages.len() {
                return None;
            }

            let group_option = match &mut self.combinations {
                Some(g) => g.next(),
                None => {
                    let mut g = Box::new(
                        self.groups
                            .packages
                            .iter()
                            .map(|v| *v)
                            .combinations(self.idx),
                    );
                    let next = g.next();
                    self.combinations = Some(g);
                    self.idx += 1;
                    next
                }
            };

            match group_option {
                Some(group) => {
                    if group.iter().sum::<u64>() == self.groups.split {
                        return Some(group);
                    }
                }
                None => self.combinations = None,
            }
        }
    }
}
