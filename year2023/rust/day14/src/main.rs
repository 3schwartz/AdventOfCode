use std::fs;

use anyhow::Result;

fn tilt(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid
}

fn score(grid: Vec<Vec<char>>) -> u32 {
    0
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day14_data.txt")?;

    let mut grid: Vec<Vec<char>> = vec![];
    for (row, line) in input.lines().enumerate() {
        grid.push(vec![]);
        for c in line.chars() {
            grid[row].push(c)
        }
    }

    println!("{:?}", grid);

    Ok(())
}
