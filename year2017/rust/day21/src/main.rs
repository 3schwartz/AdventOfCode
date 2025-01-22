use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let rules = fs::read_to_string("../data/day21_data.txt")?;
    let input = fs::read_to_string("../data/day21_input.txt")?;

    let grid = parse_grid_from_lines(&input);
    let rules = parse_rules(&rules)?;

    let actual = iterate(&grid, &rules, 5, false)?;

    println!("Part 1: {actual}");

    let actual = iterate(&grid, &rules, 18, false)?;

    println!("Part 2: {actual}");

    Ok(())
}

fn iterate(
    initial_grid: &[Vec<char>],
    rules: &HashMap<String, Vec<Vec<char>>>,
    count: usize,
    debug: bool,
) -> Result<usize> {
    let mut grid = initial_grid.to_owned();
    for _ in 0..count {
        if debug {
            println!("#########################");
            print_grid(&grid);
        }
        grid = grid_apply_rules(grid, rules)?;
    }
    Ok(grid
        .iter()
        .flat_map(|r| r.iter())
        .filter(|c| **c == '#')
        .count())
}

fn grid_apply_rules(
    grid: Vec<Vec<char>>,
    rules: &HashMap<String, Vec<Vec<char>>>,
) -> Result<Vec<Vec<char>>> {
    let grid_splitted = grid_split(&grid);
    let grid_expanded = grid_expand(grid_splitted, rules)?;
    let grid_concatenated = grid_concat(grid_expanded);
    Ok(grid_concatenated)
}

fn grid_split(grid: &[Vec<char>]) -> Vec<Vec<Vec<Vec<char>>>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let subgrid_size = if rows % 2 == 0 && cols % 2 == 0 {
        2
    } else if rows % 3 == 0 && cols % 3 == 0 {
        3
    } else {
        panic!("Grid dimensions are not divisible by 2 or 3.");
    };

    let mut result = vec![];
    for i in (0..rows).step_by(subgrid_size) {
        let mut row_blocks = vec![];
        for j in (0..cols).step_by(subgrid_size) {
            let mut block = vec![];
            for r in 0..subgrid_size {
                let mut sub_row = vec![];
                for c in 0..subgrid_size {
                    sub_row.push(grid[i + r][j + c]);
                }
                block.push(sub_row);
            }
            row_blocks.push(block);
        }
        result.push(row_blocks);
    }
    result
}

fn grid_expand(
    sub_grids: Vec<Vec<Vec<Vec<char>>>>,
    rules: &HashMap<String, Vec<Vec<char>>>,
) -> Result<Vec<Vec<Vec<Vec<char>>>>> {
    let mut grid_to_concat = vec![];
    for sub_grid_row in sub_grids {
        let mut row = vec![];
        for grid in sub_grid_row {
            let key = parse_key_from_grid(&grid);
            let new_grid = rules
                .get(&key)
                .ok_or_else(|| anyhow!("missing key {}", key))?;
            row.push(new_grid.clone());
        }
        grid_to_concat.push(row);
    }
    Ok(grid_to_concat)
}

fn grid_concat(grid: Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<char>> {
    let mut final_grid = vec![];
    for sub_grid_row in grid {
        let mut temp_rows = vec![vec![]; sub_grid_row[0].len()];
        for sub_grid in sub_grid_row {
            for (i, row) in sub_grid.iter().enumerate() {
                temp_rows[i].extend(row);
            }
        }
        final_grid.extend(temp_rows);
    }
    final_grid
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for r in grid {
        for c in r {
            print!("{c}")
        }
        println!()
    }
}

fn parse_rules(input: &str) -> Result<HashMap<String, Vec<Vec<char>>>> {
    let mut rules = HashMap::new();
    for rule in input.lines() {
        let (keys, output_grid) = parse_rule(rule);
        for key in keys {
            if rules.insert(key, output_grid.clone()).is_some() {
                return Err(anyhow!("Key from rule {} already had an entry", rule));
            }
        }
    }
    Ok(rules)
}

fn parse_rule(input: &str) -> (HashSet<String>, Vec<Vec<char>>) {
    let parts = input.split(" => ").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);
    let input_grid = parse_grid_from_string(parts[0]);
    let output_grid = parse_grid_from_string(parts[1]);

    let keys = parse_key_from_grids(input_grid);
    (keys, output_grid)
}

fn parse_grid_from_lines(input: &str) -> Vec<Vec<char>> {
    let mut initial_grid = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        initial_grid.push(row);
    }
    initial_grid
}

fn parse_grid_from_string(input: &str) -> Vec<Vec<char>> {
    let mut initial_grid = vec![];

    for line in input.trim().split('/') {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        initial_grid.push(row);
    }
    initial_grid
}

fn parse_key_from_grid(grid: &Vec<Vec<char>>) -> String {
    let mut key = Vec::with_capacity(grid.len() * grid[0].len() + grid.len() - 1);
    for row in grid {
        for col in row {
            key.push(*col);
        }
        key.push('/');
    }
    key.iter().collect()
}

fn parse_key_from_grids(grid: Vec<Vec<char>>) -> HashSet<String> {
    let mut keys = HashSet::new();

    let mut rotation = grid.clone();
    for _ in 0..4 {
        rotation = action_rotate_90(&rotation);
        keys.insert(parse_key_from_grid(&rotation));

        let flipped_horizontially = action_flip_horizontially(&rotation);
        let flipeed_vertically = action_flip_vertically(&rotation);
        keys.insert(parse_key_from_grid(&flipped_horizontially));
        keys.insert(parse_key_from_grid(&flipeed_vertically));
    }
    keys
}

fn action_flip_horizontially(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut flipped = grid.to_owned();
    flipped.reverse();
    flipped
}

fn action_flip_vertically(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut flipped = grid.to_owned();
    for row in &mut flipped {
        row.reverse();
    }
    flipped
}

fn action_rotate_90(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut rotated = vec![vec![' '; rows]; cols];
    for r in 0..rows {
        for (c, row) in rotated.iter_mut().enumerate() {
            row[rows - 1 - r] = grid[r][c]
        }
    }
    rotated
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let rules = fs::read_to_string("../../data/day21_test_data.txt")?;
        let input = fs::read_to_string("../../data/day21_input.txt")?;
        let grid = parse_grid_from_lines(&input);
        let rules = parse_rules(&rules)?;

        // Act
        let actual = iterate(&grid, &rules, 2, false)?;

        // Assert
        assert_eq!(actual, 12);
        Ok(())
    }
}

// --- Day 21: Fractal Art ---

// You find a program trying to generate some art. It uses a strange process that involves repeatedly enhancing the detail of an image through a set of rules.

// The image consists of a two-dimensional square grid of pixels that are either on (#) or off (.). The program always begins with this pattern:

// .#.
// ..#
// ###
// Because the pattern is both 3 pixels wide and 3 pixels tall, it is said to have a size of 3.

// Then, the program repeats the following process:

// If the size is evenly divisible by 2, break the pixels up into 2x2 squares, and convert each 2x2 square into a 3x3 square by following the corresponding enhancement rule.
// Otherwise, the size is evenly divisible by 3; break the pixels up into 3x3 squares, and convert each 3x3 square into a 4x4 square by following the corresponding enhancement rule.
// Because each square of pixels is replaced by a larger one, the image gains pixels and so its size increases.

// The artist's book of enhancement rules is nearby (your puzzle input); however, it seems to be missing rules. The artist explains that sometimes, one must rotate or flip the input pattern to find a match. (Never rotate or flip the output pattern, though.) Each pattern is written concisely: rows are listed as single units, ordered top-down, and separated by slashes. For example, the following rules correspond to the adjacent patterns:

// ../.#  =  ..
//           .#

//                 .#.
// .#./..#/###  =  ..#
//                 ###

//                         #..#
// #..#/..../#..#/.##.  =  ....
//                         #..#
//                         .##.
// When searching for a rule to use, rotate and flip the pattern as necessary. For example, all of the following patterns match the same rule:

// .#.   .#.   #..   ###
// ..#   #..   #.#   ..#
// ###   ###   ##.   .#.
// Suppose the book contained the following two rules:

// ../.# => ##./#../...
// .#./..#/### => #..#/..../..../#..#
// As before, the program begins with this pattern:

// .#.
// ..#
// ###
// The size of the grid (3) is not divisible by 2, but it is divisible by 3. It divides evenly into a single square; the square matches the second rule, which produces:

// #..#
// ....
// ....
// #..#
// The size of this enhanced grid (4) is evenly divisible by 2, so that rule is used. It divides evenly into four squares:

// #.|.#
// ..|..
// --+--
// ..|..
// #.|.#
// Each of these squares matches the same rule (../.# => ##./#../...), three of which require some flipping and rotation to line up with the rule. The output for the rule is the same in all four cases:

// ##.|##.
// #..|#..
// ...|...
// ---+---
// ##.|##.
// #..|#..
// ...|...
// Finally, the squares are joined into a new grid:

// ##.##.
// #..#..
// ......
// ##.##.
// #..#..
// ......
// Thus, after 2 iterations, the grid contains 12 pixels that are on.

// How many pixels stay on after 5 iterations?
