use std::{fs, collections::HashSet};

use anyhow::{Result, Ok};

fn get_neighbors(position: &(i32, i32), set: &HashSet<(i32,i32)>) -> Result<u8> {
    let neighbors = vec![(-1,-1), (0,-1), (1,-1), (-1,0),(1,0),(-1,1),(0,1),(1,1)];

    let mut on = 0;
    for neighbor in neighbors {
        let get = (position.0 + neighbor.0, position.1 + neighbor.1);
        if set.contains(&get) {
            on+=1;
        }
    }

    Ok(on)
}

fn add_corners(set: &mut HashSet<(i32,i32)>) {
    let neighbors = vec![(0,0), (0,99), (99,0), (99,99)];

    for neighbor in neighbors {
        set.insert(neighbor);
    }
}

fn iterate_lights(mut set: HashSet<(i32,i32)>, corners_on: bool) -> Result<HashSet<(i32,i32)>> {
    if corners_on {
        add_corners(&mut set);
    };
    let steps = 100;    
    for _ in 0..steps {
        let mut next_set = HashSet::new();
        for y in 0..100 {
            for x in 0..100 {
                let next = (x,y);
                let neighbors = get_neighbors(&next, &set)?;
                let is_on = set.contains(&next);
                let next_on = match is_on {
                    true => match neighbors {
                        2 | 3 => true,
                        _ => false
                    },
                    false => match neighbors {
                        3 => true,
                        _ => false
                    },
                };
                if next_on {
                    next_set.insert(next);
                }
            }
        }
        if corners_on{
            add_corners(&mut next_set);
        }
        set = next_set;
    }

    Ok(set)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day18_data.txt")?;

    let mut set = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                set.insert((x as i32, y as i32));
            }
        }
    }

    let part_1 = iterate_lights(set.clone(), false)?;

    println!("Part 1: {}", part_1.len());

    let part_2 = iterate_lights(set.clone(), true)?;

    println!("Part 2: {}", part_2.len());

    Ok(())
}
