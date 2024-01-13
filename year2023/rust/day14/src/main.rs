use std::{collections::BTreeMap, fs};

use anyhow::Result;

fn tilt(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    for c in 0..cols {
        for _ in 0..rows {
            for r in 0..rows {
                if r > 0 && grid[r][c] == 'O' && grid[r - 1][c] == '.' {
                    grid[r - 1][c] = 'O';
                    grid[r][c] = '.';
                }
            }
        }
    }
}

fn score(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let mut score = 0;
    for (r, row) in grid.iter().enumerate() {
        for col in row.iter() {
            if *col == 'O' {
                score += rows - r;
            }
        }
    }
    score
}

fn empty_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for _ in 0..rows {
        let row: Vec<char> = vec![Default::default(); cols];
        grid.push(row);
    }
    grid
}

/// Rotate counter clockwise in coordinate system is going from
/// (x,y) to (y,-x). In our case it will be going from
/// [row][column], (y,x) to [-column][row] , (-x,y).
/// To only have positive coordinate we shift the [row] index
/// by R - 1 - [column].
fn rotate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut empty = empty_grid(cols, rows);
    for (r, row) in grid.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            empty[c][rows - 1 - r] = *item;
        }
    }
    empty
}
/// Stores as [row][column] which in a coordinate system
/// can be interpreted as [y][x].
fn make_grid_from_input(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    for (row, line) in input.lines().enumerate() {
        grid.push(vec![]);
        for c in line.chars() {
            grid[row].push(c)
        }
    }
    grid
}

fn create_cache_key(grid: &[Vec<char>]) -> BTreeMap<(usize, usize), char> {
    let mut key = BTreeMap::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            key.insert((r, c), *col);
        }
    }
    key
}

fn run_cycles(total_cycles: u32, mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut cycles = 0;
    let mut cache: BTreeMap<BTreeMap<(usize, usize), char>, u32> = BTreeMap::new();
    while cycles < total_cycles {
        cycles += 1;
        for _ in 0..4 {
            tilt(&mut grid);
            grid = rotate(&grid);
        }
        let key = create_cache_key(&grid);
        if let Some(cycle) = cache.get(&key) {
            let period = cycles - cycle;
            let remaining = (total_cycles - cycles) / period;
            cycles += remaining * period;
        }
        cache.insert(key, cycles);
    }

    grid
}

fn part_2(input: &str) -> usize {
    let mut grid = make_grid_from_input(input);
    grid = run_cycles(1_000_000_000, grid);
    score(&grid)
}

fn part_1(input: &str) -> usize {
    let mut grid = make_grid_from_input(input);
    tilt(&mut grid);
    score(&grid)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day14_data.txt")?;

    let part_1 = part_1(&input);

    println!("Part 1: {}", part_1);

    let part_2 = part_2(&input);

    println!("Part 2: {}", part_2);

    Ok(())
}
