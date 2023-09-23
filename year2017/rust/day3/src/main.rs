use std::collections::HashMap;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let final_position = 265149;

    let manhattan = part1(final_position);
    println!("Part 1: {}", manhattan);

    let above = part2(final_position);
    println!("Part 2: {}", above);

    Ok(())
}

fn part2(final_position: i32) -> i32 {
    let mut last_sum = 1;
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut dx, mut dy) = (1, 0);
    let mut found = false;
    let mut steps = 1;
    let mut map = HashMap::from([((0, 0), 1)]);

    loop {
        for _ in 1u8..=2 {
            for __ in 1..=steps {
                x = x + dx;
                y = y + dy;
                last_sum = get_neigbors(x, y, &map);
                if last_sum >= final_position {
                    found = true;
                    break;
                }
                map.insert((x, y), last_sum);
            }
            if found {
                break;
            }
            (dx, dy) = (-dy, dx);
        }
        if found {
            break;
        }
        steps += 1;
    }
    last_sum
}

fn get_neigbors(x: i32, y: i32, map: &HashMap<(i32, i32), i32>) -> i32 {
    let mut sum = 0;
    for _x in x - 1..=x + 1 {
        for _y in y - 1..=y + 1 {
            if _x == x && _y == y {
                continue;
            }
            let Some(neighbors) = map.get(&(_x, _y)) else {
                continue;
            };
            sum += neighbors;
        }
    }
    sum
}

fn part1(final_position: i32) -> i32 {
    let mut position = 1;
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut dx, mut dy) = (1, 0);
    let mut found = false;
    let mut steps = 1;

    loop {
        for _ in 1u8..=2 {
            for __ in 1..=steps {
                x = x + dx;
                y = y + dy;
                position += 1;
                if position == final_position {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
            (dx, dy) = (-dy, dx);
        }
        if found {
            break;
        }
        steps += 1;
    }

    let manhattan = x.abs() + y.abs();
    return manhattan;
}
