use std::{
    collections::{BTreeSet, BinaryHeap},
    fs,
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day17_data.txt")?;
    let grid = grid(&input);
    let is_valid_part_1 =
        |new_steps: usize, _: usize, _: usize, _: usize| -> bool { new_steps <= 3 };
    let part_1 = find_min_loss(&grid, is_valid_part_1);

    println!("Part 1: {}", part_1);

    let is_valid_part_2 =
        |new_steps: usize, prior_steps: usize, direction: usize, prior_direction: usize| -> bool {
            new_steps <= 10
                && (prior_steps >= 4
                    || direction == prior_direction
                    || prior_direction == usize::MAX)
        };
    let part_2 = find_min_loss(&grid, is_valid_part_2);

    println!("Part 2: {}", part_2);
    Ok(())
}

fn find_min_loss(
    grid: &[Vec<char>],
    is_valid: fn(
        new_steps: usize,
        prior_steps: usize,
        direction: usize,
        prior_direction: usize,
    ) -> bool,
) -> u32 {
    let mut heap = BinaryHeap::new();
    heap.push(State::new(0, 0, 0, usize::MAX, usize::MAX));
    let mut visited = BTreeSet::new();
    while let Some(state) = heap.pop() {
        if !visited.insert(state.cache_value()) {
            continue;
        }
        for (direction, (dr, dc)) in State::NEIGHBORS.iter().enumerate() {
            let is_reverse = (direction + 2) % 4 == state.direction;
            if is_reverse {
                continue;
            }
            let r = state.row + dr;
            let c = state.col + dc;
            if r < 0 || r >= grid.len() as i32 || c < 0 || c >= grid[0].len() as i32 {
                continue;
            }

            let new_steps = if direction == state.direction {
                state.steps_direction + 1
            } else {
                1
            };
            if !is_valid(new_steps, state.steps_direction, direction, state.direction) {
                continue;
            }
            let cost = state.loss + grid[r as usize][c as usize] as u32 - '0' as u32;
            if r == grid.len() as i32 - 1 && c == grid[0].len() as i32 - 1 {
                return cost;
            }
            if visited.contains(&(r, c, direction, new_steps)) {
                continue;
            }
            heap.push(State::new(cost, r, c, direction, new_steps));
        }
    }
    0
}

#[derive(Eq, PartialEq)]
struct State {
    loss: u32,
    row: i32,
    col: i32,
    // right: 0, down: 1, left: 2, up: 3
    direction: usize,
    steps_direction: usize,
}

impl State {
    const NEIGHBORS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn new(loss: u32, row: i32, col: i32, direction: usize, steps_direction: usize) -> Self {
        Self {
            loss,
            row,
            col,
            direction,
            steps_direction,
        }
    }

    fn cache_value(&self) -> (i32, i32, usize, usize) {
        (self.row, self.col, self.direction, self.steps_direction)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for column in line.chars() {
            row.push(column);
        }
        grid.push(row);
    }
    grid
}
