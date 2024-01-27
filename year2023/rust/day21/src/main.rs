use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day21_data.txt")?;
    let (grid, start) = create_grid(&input);
    let distances = find_distances(&grid, &start);
    let part_1 = find_reachable_garden_plots(64, &grid, &distances);

    println!("Part 1: {}", part_1);

    let part_2 = find_reachable_garden_plots(26_501_365, &grid, &distances);

    println!("Part 2: {}", part_2);

    Ok(())
}

const MAX_EXPANSION: i64 = 4;
const LIMIT_EXPANSION: i64 = MAX_EXPANSION - 1;
const EXPANSIONS: [i64; 7] = [-3, -2, -1, 0, 1, 2, 3];
const NEIGHBORS: [(i64, i64); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

/// Since the borders are '.', then at some points you can go the the borders, "walk" along them
/// and then go inside the quadrant again.
/// 
/// Hence when we are below the [`MAX_EXPANSION`] we brute-force the solution.
/// When we are at the [`MAX_EXPANSION`] for shift we assume that the shortest path from here
/// would be to walk along the borders.
fn find_reachable_garden_plots(
    steps: u64,
    grid: &Vec<Vec<char>>,
    distances: &HashMap<Position, u64>,
) -> u64 {
    let mut plots = 0;
    let rows = grid.len() as i64;
    let columns = grid[0].len() as i64;
    let mut cached = HashMap::new();
    for row in 0..rows {
        for col in 0..columns {
            let position = Position::new(0, 0, row, col);
            if !distances.contains_key(&position) {
                continue;
            }
            for row_shift in EXPANSIONS {
                for col_shift in EXPANSIONS {
                    let shifted = Position::new(row_shift, col_shift, row, col);
                    let distance = *distances.get(&shifted).unwrap();
                    if distance > steps {
                        continue;
                    }
                    // inside a quadrant
                    if distance % 2 == steps % 2 && distance <= steps {
                        plots += 1;
                    }
                    // corner
                    if row_shift.abs() == LIMIT_EXPANSION && col_shift.abs() == LIMIT_EXPANSION {
                        plots +=
                            find_solutions(distance, true, steps, columns as u64, &mut cached);
                    } 
                    // edge
                    else if row_shift.abs() == LIMIT_EXPANSION
                        || col_shift.abs() == LIMIT_EXPANSION
                    {
                        plots += find_solutions(distance, false, steps, columns as u64, &mut cached);
                    }
                }
            }
        }
    }
    plots
}

fn find_solutions(
    distance: u64,
    is_corner: bool,
    steps: u64,
    grid_size: u64,
    cached: &mut HashMap<(u64, bool, u64), u64>,
) -> u64 {
    let reachable_grids = (steps - distance) / grid_size;

    if let Some(c) = cached.get(&(distance, is_corner, steps)) {
        return *c;
    }
    let mut plots = 0;
    for grid_inc in 1..reachable_grids + 1 {
        let border_grid_inc = distance + grid_size * grid_inc;
        if border_grid_inc <= steps && border_grid_inc % 2 == steps % 2 {
            plots += if is_corner { grid_inc + 1 } else { 1 }
        }
    }
    cached.insert((distance, is_corner, steps), plots);
    plots
}

#[derive(Hash, Clone, PartialEq, Eq)]
struct Position {
    row_shift: i64,
    col_shift: i64,
    row: i64,
    col: i64,
}

impl Position {
    fn new(row_shift: i64, col_shift: i64, row: i64, col: i64) -> Self {
        Self {
            row_shift,
            col_shift,
            row,
            col,
        }
    }

    fn above_limit(&self) -> bool {
        self.col_shift.abs() > MAX_EXPANSION || self.row_shift.abs() > MAX_EXPANSION
    }
}

fn find_distances(grid: &Vec<Vec<char>>, start: &(i64, i64)) -> HashMap<Position, u64> {
    let mut distances: HashMap<Position, u64> = HashMap::new();
    let mut stack: VecDeque<(Position, u64)> =
        VecDeque::from([(Position::new(0, 0, start.0, start.1), 0)]);
    let rows = grid.len() as i64;
    let columns = grid[0].len() as i64;

    while let Some((position, distance)) = stack.pop_front() {
        let mut updated = position.clone();
        if position.row < 0 {
            updated.row_shift -= 1;
            updated.row += rows;
        }
        if position.row >= rows {
            updated.row_shift += 1;
            updated.row -= rows;
        }
        if position.col < 0 {
            updated.col_shift -= 1;
            updated.col += columns;
        }
        if position.col >= columns {
            updated.col_shift += 1;
            updated.col -= columns;
        }
        if !is_inside_grid(updated.row, updated.col, grid)
            || grid[updated.row as usize][updated.col as usize] == '#'
        {
            continue;
        }
        if distances.contains_key(&updated) {
            continue;
        }
        if updated.above_limit() {
            continue;
        }
        distances.insert(updated.clone(), distance);
        let next_distance = distance + 1;
        for neighbor in NEIGHBORS {
            let mut next = updated.clone();
            next.row += neighbor.0;
            next.col += neighbor.1;
            stack.push_back((next, next_distance));
        }
    }
    distances
}

fn is_inside_grid(row: i64, col: i64, grid: &Vec<Vec<char>>) -> bool {
    let rows = grid.len() as i64;
    let columns = grid[0].len() as i64;
    row >= 0 && row < rows && col >= 0 && col < columns
}

fn create_grid(input: &str) -> (Vec<Vec<char>>, (i64, i64)) {
    let mut grid = vec![];
    let mut start = (0, 0);
    for (r, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (col, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                start = (r as i64, col as i64);
            }
        }
        grid.push(row);
    }
    (grid, start)
}
