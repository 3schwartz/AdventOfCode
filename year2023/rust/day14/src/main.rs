use std::fs;

use anyhow::Result;

fn tilt(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    for c in 0..cols {
        for _ in 0..rows {
            for r in 0..rows {
                if r > 0 && grid[r][c] == 'O' && grid[r-1][c] == '.' {
                    grid[r-1][c] = 'O';
                    grid[r][c] = '.';
                }
            }
        }
    }
}

fn score(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut score = 0;
    for r in 0..rows {
        let row = &grid[r];
        for c in 0..cols {
            if row[c] == 'O' {
                score += rows - r;
            }
        }
    }
    score
}

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

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day14_data.txt")?;

    let mut grid = make_grid_from_input(&input);
    tilt(&mut grid);
    let score = score(&grid);
    
    println!("Part 1: {}", score);

    Ok(())
}
