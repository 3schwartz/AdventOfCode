use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

fn main() {
    let file = fs::read_to_string("../data/day7_data.txt").expect("couldn't open file");

    let road = Road::new(file);
    let solution = road.find_solution();

    println!("Part 1: {}", solution);

    let seconds = road.find_seconds(50, 60);

    println!("Part 2: {}", seconds);
}

struct Road {
    relations: HashMap<char, Vec<char>>,
    nodes: HashSet<char>,
}

impl Road {
    fn new(file: String) -> Road {
        let mut nodes = HashSet::new();
        let mut begins: HashMap<char, Vec<char>> = HashMap::new();
        for s in file.lines() {
            let split = s
                .split(" must be finished before step ")
                .collect::<Vec<&str>>();
            let step = split[0].chars().next_back().unwrap();
            let to_begin = split[1].chars().next().unwrap();

            nodes.insert(step);
            nodes.insert(to_begin);
            begins.entry(to_begin).or_default().push(step);
        }
        return Self {
            relations: begins,
            nodes: nodes,
        };
    }

    fn char_seconds(c: char) -> u32 {
        return (c as u32 - 'A' as u32) + 1;
    }

    fn find_solution(&self) -> String {
        let mut available = HashSet::<char>::new();
        let mut word = Vec::new();
        while self.nodes.len() != word.len() {
            let mut evaluate = self.find_queue(&available);
            let first = match evaluate.pop() {
                Some(c) => c.0,
                None => panic!("No element found"),
            };
            word.push(first);
            available.insert(first.clone());
        }
        let solution = String::from_iter(word);
        return solution;
    }

    fn find_queue(&self, available: &HashSet<char>) -> BinaryHeap<Reverse<char>> {
        let mut evaluate = BinaryHeap::new();
        for node in &self.nodes ^ available {
            let to_begins = self.relations.get(&node);
            if to_begins.is_none() {
                evaluate.push(Reverse(node));
                continue;
            }

            let mut add = true;
            for l in to_begins.unwrap() {
                if !available.contains(l) {
                    add = false;
                    break;
                }
            }
            if add {
                evaluate.push(Reverse(node))
            }
        }
        return evaluate;
    }

    fn find_seconds(&self, workers: u32, offset: u32) -> u32 {
        let mut available = HashSet::<char>::new();
        let mut seconds = 0;
        let mut workers_map = HashMap::<u32, Work>::new();
        (0..workers).for_each(|w| {
            _ = workers_map.insert(
                w,
                Work {
                    node: None,
                    second: 0,
                },
            )
        });

        loop {
            workers_map
                .values_mut()
                .filter(|c| c.node.is_some())
                .for_each(|mut c| {
                    if c.second == 1 {
                        available.insert(c.node.unwrap());
                        *c = Work {
                            node: None,
                            second: 0,
                        };
                        return;
                    }
                    c.second -= 1
                });

            let zeros = workers_map.values().filter(|c| c.second == 0).count();

            if zeros == workers_map.len() && available.len() == self.nodes.len() {
                break;
            }

            let evaluate = self.find_queue(&available);

            for e in evaluate {
                let exist = workers_map.values().find(|w| match w.node {
                    Some(c) => c == e.0,
                    None => false,
                });
                if let Some(_) = exist {
                    continue;
                }
                let zeros = workers_map.iter().find(|(_, w)| w.node.is_none());
                match zeros {
                    Some((key, _)) => {
                        let seconds = Road::char_seconds(e.0);
                        workers_map.entry(*key).and_modify(|c| {
                            *c = Work {
                                node: Some(e.0),
                                second: seconds + offset,
                            }
                        });
                    }
                    None => break,
                };
            }
            seconds += 1;
        }
        return seconds;
    }
}

struct Work {
    node: Option<char>,
    second: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        // Arrange
        let file = fs::read_to_string("../data/day7_test_data.txt").expect("couldn't open file");

        // Act
        let road = Road::new(file);
        let solution = road.find_seconds(2, 0);

        // Assert
        assert_eq!(15, solution)
    }

    #[test]
    fn test_part1() {
        // Arrange
        let file = fs::read_to_string("../data/day7_test_data.txt").expect("couldn't open file");

        let road = Road::new(file);
        let solution = road.find_solution();
        assert_eq!("CABDFE", solution)
    }

    #[test]
    fn test_char_seconds() {
        // Arrange
        let c = 'A';

        // Act
        let seconds: u32 = Road::char_seconds(c);

        // Assert
        assert_eq!(1, seconds);
    }
}
