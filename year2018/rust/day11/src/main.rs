use std::{collections::HashMap, iter::repeat};

fn main() {
    let size = 300;
    let serial = 9110;

    let powers = generate_powers(size, serial);
    let max = find_max(size, &powers, 1);

    println!("Part 1: {:?} with power {}", max.top, max.max_power);

    let max_square = find_max_square(size, &powers);

    println!("Part 2: {:?} with power {}", max_square.top, max_square.max_power);
}


fn get_power(x : usize, y : usize, serial : i32) -> i32 {
    let rack_id = x as i32+ 10;
    let mut power = rack_id * y as i32;
    power += serial;
    power = power * rack_id;
    power = power / 100;
    power = power % 10;
    power -= 5;
    return power
}

fn get_coords(from: usize, to: usize) -> impl Iterator<Item=(usize,usize)> {
    let xs = (from..=to)
        .cycle()
        .take(to * to);
    let ys = (from..=to)
        .flat_map(move |y| repeat(y).take(to));
    let coords = xs.zip(ys);
    return coords
}

fn generate_powers(size: usize, serial: i32) -> HashMap<(usize, usize), i32> {
    let coords = get_coords(1, size);
    let mut powers: HashMap<(usize, usize), i32> = HashMap::new();
    coords.for_each(|(x,y)| {
        let power = get_power(x, y, serial);
        powers.insert((x,y), power);
    });
    return powers
}

struct MaxPower {
    top: (usize, usize, usize),
    max_power: i32
}

fn get_neighbors(xc: usize, yc: usize, square_size: usize) -> impl Iterator<Item=(usize,usize)> {
    let n = 0..square_size;
    let neighbors = n.clone()
        .cycle()
        .take(square_size * square_size)
        .zip(n
            .flat_map(move |y| repeat(y).take(square_size)))
        .map(move |(x,y)| (x + xc, y + yc));
    return neighbors;
}

fn find_max(size: usize, powers: &HashMap<(usize, usize), i32>, square_size: usize) -> MaxPower {
    let middle = get_coords(1, size-square_size+1);
    let mut max = i32::MIN;
    let (mut xm, mut ym) = (0,0);
    for (x, y) in middle {
        let p: i32 = get_neighbors(x, y, square_size)
            .flat_map(|n| powers.get(&n))
            .sum();
        if p > max {
            max = p;
            (xm, ym) = (x,y);
        }
    }
    return MaxPower { top: (xm, ym, square_size), max_power: max }
}

fn find_max_square(size: usize, powers: &HashMap<(usize, usize), i32>) -> MaxPower {
    let mut max_temp = MaxPower{max_power: i32::MIN, top: (0,0,0)};
    for i in 1..=size {
        let max = find_max(size, &powers, i);
        if max.max_power > max_temp.max_power {
            max_temp = max;
        }
    }
    return max_temp;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_when_calculate_power_then_correct() {
        // Arrange
        let x = 3;
        let y = 5;
        let serial = 8;

        // Act
        let power = get_power(x, y, serial);

        // Assert
        assert_eq!(power, 4)
    }

    #[test]
    fn test_par1() {
        // Arrange
        let size = 300;
        let serial = 18;

        // Act
        let powers = generate_powers(size, serial);
        let max = find_max(size, &powers, 3);

        // Assert
        assert_eq!(max.top, (33,45, 3));
        assert_eq!(max.max_power, 29);
    }

    #[test]
    fn test_par2() {
        // Arrange
        let size = 300;
        let serial = 18;

        // Act
        let powers = generate_powers(size, serial);
        let max = find_max_square(size, &powers);

        // Assert
        assert_eq!(max.top, (90,269,16));
        assert_eq!(max.max_power, 113);
    }
}