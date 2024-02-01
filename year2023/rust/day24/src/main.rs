use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day24_data.txt")?;

    let hailstones = Hailstone::list(&input)?;

    let mut intersections = 0;
    for (i, first) in hailstones.iter().enumerate() {
        for second in hailstones.iter().skip(i + 1) {
            if has_intersection_forward(first, second) {
                intersections += 1;
            }
        }
    }

    println!("Part 1: {}", intersections);
    Ok(())
}

/// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
fn has_intersection_forward(first: &Hailstone, second: &Hailstone) -> bool {
    let x1 = first.xp;
    let x2 = x1 + first.xv;
    let y1 = first.yp;
    let y2 = y1 + first.yv;

    let x3 = second.xp;
    let x4 = x3 + second.xv;
    let y3 = second.yp;
    let y4 = y3 + second.yv;

    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denominator == 0 {
        return false;
    }

    let mut px = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
    px /= denominator;
    let mut py = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
    py /= denominator;
    if !(200000000000000..=400000000000000).contains(&px)
        || !(200000000000000..=400000000000000).contains(&py)
    {
        return false;
    }
    let first_correct_direction = (px <= x1) ^ (x2 > x1);
    let second_correct_direction = (px <= x3) ^ (x4 > x3);

    first_correct_direction && second_correct_direction
}

#[derive(Debug)]
struct Hailstone {
    xp: i128,
    yp: i128,
    xv: i128,
    yv: i128,
}

impl Hailstone {
    fn list(input: &str) -> Result<Vec<Hailstone>> {
        let mut hailstones = vec![];
        for line in input.lines() {
            let hailstone = Hailstone::from(line)?;
            hailstones.push(hailstone);
        }
        Ok(hailstones)
    }

    fn from(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split(" @ ").collect();
        let positions = parts[0]
            .split(", ")
            .map(|p| p.parse::<i128>())
            .collect::<Result<Vec<i128>, _>>()?;
        let velocities = parts[1]
            .split(", ")
            .map(|p| p.parse::<i128>())
            .collect::<Result<Vec<i128>, _>>()?;
        Ok(Self {
            xp: positions[0],
            yp: positions[1],
            xv: velocities[0],
            yv: velocities[1],
        })
    }
}
