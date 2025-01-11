use std::collections::VecDeque;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let steps = 324;

    let v = run_curcuit(steps, 2017, false);

    println!("Part 1: {v}");

    let v = run_curcuit_deque(steps, 2017, 2017, false)?;
    println!("Part 1: {v}");

    let v = position_after(steps, 0)?;
    println!("Part 1: {v}");

    let v = run_curcuit_deque(steps, 50_000_000, 0, false)?;

    println!("Part 2: {v}");
    Ok(())
}

fn position_after(step: usize, target: usize) -> Result<usize> {
    let mut result_after = None;
    let mut position = 0;
    let mut size = 1;

    for i in 0..50_000_001 {
        position = (position + step) % size;

        if i == target {
            // If the current number is the target, track the next number
            result_after = Some((position + 1) % size);
        } else if let Some(target_position) = result_after {
            // If we found the target's position, track the next number
            if position == target_position {
                result_after = Some(i)
            }
        }

        position += 1;
        size += 1;
    }

    result_after.ok_or_else(|| anyhow!("missing result"))
}

fn run_curcuit_deque(steps: usize, size: usize, target: usize, debug: bool) -> Result<usize> {
    let mut locations = VecDeque::with_capacity(size);
    locations.push_back(0);
    for i in 1..=size {
        if i % 100_000 == 0 && debug {
            println!("{i}");
        }
        let len = locations.len();
        let pos = (len + len - steps % len - 1) % len;
        locations.rotate_right(pos);
        locations.push_front(i)
    }
    let i = locations
        .iter()
        .position(|&x| x == target)
        .ok_or_else(|| anyhow!("{target} missing"))?;
    Ok(locations[(i + 1) % locations.len()])
}

fn run_curcuit(steps: usize, size: usize, debug: bool) -> usize {
    let mut locations = vec![0];
    let mut position = 0;
    for i in 1..=size {
        if i % 100_000 == 0 && debug {
            println!("{i}");
        };
        position = (position + steps) % locations.len();
        if locations.len() == position + 1 {
            locations.push(i);
        } else {
            locations.insert(position + 1, i);
        }
        position += 1;
    }
    let i = (position + 1) % locations.len();
    locations[i]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        let steps = 3;

        // Act
        let v = run_curcuit(steps, 2017, false);

        // Assert
        assert_eq!(v, 638);
    }

    #[test]
    fn test_part_1_deque() -> Result<()> {
        // Arrange
        let steps = 3;

        // Act
        let v = run_curcuit_deque(steps, 2017, 2017, false)?;

        // Assert
        assert_eq!(v, 638);
        Ok(())
    }
}
