use std::{collections::HashSet, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day22_data.txt")?;

    let bricks = create_bricks(&input)?;
    let occupied = create_occupied(&bricks);

    let (bricks, occupied, _) = fall(bricks, occupied);

    let (total_still, total_movements) = find_movements(bricks, occupied);

    println!("Part 1: {}", total_still);
    println!("Part 2: {}", total_movements);

    Ok(())
}

fn find_movements(
    initial_bricks: Vec<Vec<(i32, i32, i32)>>,
    initial_occupied: HashSet<(i32, i32, i32)>,
) -> (u32, usize) {
    let mut total_still = 0;
    let mut total_movements = 0;
    for brick in &initial_bricks {
        let bricks = initial_bricks.clone();
        let mut occupied = initial_occupied.clone();

        for (x, y, z) in brick {
            occupied.remove(&(*x, *y, *z));
        }

        let (_, _, falls) = fall(bricks, occupied);
        if falls == 0 {
            total_still += 1;
        }
        total_movements += falls;
    }
    (total_still, total_movements)
}

#[allow(clippy::type_complexity)]
fn fall(
    mut bricks: Vec<Vec<(i32, i32, i32)>>,
    mut occupied: HashSet<(i32, i32, i32)>,
) -> (Vec<Vec<(i32, i32, i32)>>, HashSet<(i32, i32, i32)>, usize) {
    let mut falls = HashSet::new();
    loop {
        let mut fall = false;
        let mut new_bricks = vec![];
        for (idx, brick) in bricks.iter().enumerate() {
            let mut can_fall = true;
            for (x, y, z) in brick {
                // hit ground
                if *z == 1 {
                    can_fall = false;
                }
                let coord_down = (*x, *y, *z - 1);
                // brick below which isn't the brick itself
                if occupied.contains(&coord_down) && !brick.contains(&coord_down) {
                    can_fall = false;
                }
            }
            if !can_fall {
                new_bricks.push(brick.clone());
                continue;
            }
            fall = true;
            falls.insert(idx);
            let mut new_brick = vec![];
            for (x, y, z) in brick {
                occupied.remove(&(*x, *y, *z));
                occupied.insert((*x, *y, *z - 1));
                new_brick.push((*x, *y, z - 1));
            }
            new_bricks.push(new_brick);
        }
        bricks = new_bricks;
        if !fall {
            break;
        }
    }
    (bricks, occupied, falls.len())
}

fn create_occupied(bricks: &Vec<Vec<(i32, i32, i32)>>) -> HashSet<(i32, i32, i32)> {
    let mut occupied = HashSet::new();
    for brick in bricks {
        for coord in brick {
            occupied.insert(*coord);
        }
    }
    occupied
}

/// Bricks are always a straight line so they only extend in one direction.
fn create_bricks(input: &str) -> Result<Vec<Vec<(i32, i32, i32)>>> {
    let mut bricks: Vec<Vec<(i32, i32, i32)>> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split('~').collect();
        let start: Vec<&str> = parts[0].split(',').collect();
        let end: Vec<&str> = parts[1].split(',').collect();
        let (sx, sy, sz) = get_coordinates(&start)?;
        let (ex, ey, ez) = get_coordinates(&end)?;

        let mut brick = vec![];
        if sx == ex && sy == ey {
            for z in sz..=ez {
                brick.push((sx, sy, z))
            }
        } else if sx == ex && sz == ez {
            for y in sy..=ey {
                brick.push((sx, y, sz))
            }
        } else if sz == ez && sy == ey {
            for x in sx..=ex {
                brick.push((x, sy, sz))
            }
        }
        bricks.push(brick);
    }
    Ok(bricks)
}

fn get_coordinates(coord: &[&str]) -> Result<(i32, i32, i32)> {
    Ok((coord[0].parse()?, coord[1].parse()?, coord[2].parse()?))
}
