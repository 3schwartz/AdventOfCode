use std::{fs, collections::{HashMap, HashSet}, str::Split};

fn create_coors(lines: Split<&str>) -> (HashMap<(u32,u32), usize>, u32, u32, u32, u32) {
    let mut x_min = u32::MAX;
    let mut x_max = u32::MIN;
    let mut y_min = u32::MAX;
    let mut y_max = u32::MIN;
    let mut coords: HashMap<(u32,u32), usize> = HashMap::new();
    for (i, line) in lines.enumerate() {
        let line_ints: Vec<u32> = line.split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let x = line_ints[0];
        let y = line_ints[1];
        coords.insert((x,y), i);
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
        if y < y_min {
            y_min = y;
        }
        if y > y_max {
            y_max = y;
        }
        let id = std::char::from_u32('A' as u32 + i as u32)
            .unwrap();
        println!("{}: {}, {}",id, line_ints[0],line_ints[1])
    }
    return (coords, x_min, x_max, y_min, y_max)
}

fn find_count(coords: HashMap<(u32,u32), usize>, x_min: u32, x_max:u32, y_min:u32, y_max : u32) ->
    (HashSet<usize>, HashMap<usize, u32>) {
    let mut borders = HashSet::new();
    let mut coord_count: HashMap<usize, u32> = HashMap::new();
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let mut optimal_id = &0;
            let mut optimal = i32::MAX;
            let mut has_multiple = false;

            for (coord, d) in &coords {
                let manhattan = (x as i32 - coord.0 as i32).abs() + (y as i32 - coord.1 as i32).abs();
                if manhattan > optimal {
                    continue;
                }
                if manhattan < optimal {
                    optimal = manhattan;
                    optimal_id = d;
                    has_multiple = false;
                    continue;
                }
                has_multiple = true;
            }
            if has_multiple {
                continue;
            }
            if x == x_min || x == x_max || y == y_min || y == y_max {
                borders.insert(*optimal_id);
            }
            coord_count.entry(*optimal_id).and_modify(|f| *f += 1)
                .or_insert(1);
        }
    }
    return (borders, coord_count)

}

fn find_max_count_on_center(borders : HashSet<usize>, coord_count : HashMap<usize, u32>) -> u32 {
    let mut max_count = u32::MIN;
    for (id, count) in &coord_count {
        if borders.contains(id) {
            continue;
        }
        if count > &max_count {
            max_count = *count
        }
    }
    return max_count
}

fn main() {
    let file: String = fs::read_to_string("../data/day6_data.txt")
        .expect("couldn't open file");
    let lines = file.split("\r\n");
    let (coords, x_min, x_max, y_min, y_max) = create_coors(lines);
    let (borders, coord_count) = find_count(coords, x_min, x_max, y_min, y_max);
    let max_count = find_max_count_on_center(borders, coord_count);
    println!("Part 1: {}", max_count)
}

#[cfg(test)]
mod test {
    use std::fs;
    use super::*;

    #[test]
    fn test_part1() {
        let file: String = fs::read_to_string("../../data/day6_test_data.txt")
        .expect("couldn't open file");
        let lines = file.split("\r\n");
        let (coords, x_min, x_max, y_min, y_max) = create_coors(lines);
        let (borders, coord_count) = find_count(coords, x_min, x_max, y_min, y_max);
        let max_count = find_max_count_on_center(borders, coord_count);
        assert_eq!(max_count, 17);
    }
}