use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day23_data.txt")?;
    let grid = create_grid(&input);
    let part_1 = find_max_steps(true, &grid);

    println!("Part 1: {}", part_1);

    let vertices = find_vertices(&grid);
    let edges = find_edges(vertices, &grid);
    let part_2 = dfs(0, (0, 1), &edges, &mut HashSet::new(), grid.len() - 1, 0);

    println!("Part 2: {}", part_2);

    Ok(())
}

#[allow(clippy::type_complexity)]
fn dfs(
    distance: u32,
    coord: (usize, usize),
    edges: &HashMap<(usize, usize), Vec<(usize, usize, u32)>>,
    visisted: &mut HashSet<(usize, usize)>,
    final_row: usize,
    mut max_distance: u32,
) -> u32 {
    if final_row == coord.0 {
        let max = std::cmp::max(distance, max_distance);
        // println!("{}", max);
        return max;
    }
    if !visisted.insert(coord) {
        return max_distance;
    }
    for (r, c, d) in edges.get(&coord).unwrap() {
        let updated = dfs(
            distance + d,
            (*r, *c),
            edges,
            visisted,
            final_row,
            max_distance,
        );
        max_distance = std::cmp::max(max_distance, updated);
    }
    visisted.remove(&coord);
    max_distance
}

#[allow(clippy::type_complexity)]
fn find_edges(
    vertices: HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) -> HashMap<(usize, usize), Vec<(usize, usize, u32)>> {
    let mut edges: HashMap<(usize, usize), Vec<(usize, usize, u32)>> = HashMap::new();
    for (row, col) in &vertices {
        let mut edge = vec![];
        let mut queue = VecDeque::from([(*row, *col, 0u32)]);
        let mut visited = HashSet::new();
        while let Some((r, c, distance)) = queue.pop_front() {
            if !visited.insert((r, c)) {
                continue;
            }
            if vertices.contains(&(r, c)) && (r, c) != (*row, *col) {
                edge.push((r, c, distance));
                continue;
            }
            for neighbor in neighbors((r as i32, c as i32)) {
                if !is_within_grid(neighbor, grid) {
                    continue;
                }
                let next = (neighbor.0 as usize, neighbor.1 as usize);
                if grid[next.0][next.1] == '#' {
                    continue;
                }
                queue.push_back((next.0, next.1, distance + 1));
            }
        }

        edges.insert((*row, *col), edge);
    }
    edges
}

fn find_vertices(grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut vertices = HashSet::new();
    vertices.insert((0, 1));
    vertices.insert((grid.len() - 1, grid[0].len() - 2));
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '#' {
                continue;
            }
            let mut neighbors_count = 0;
            for neighbor in neighbors((r as i32, c as i32)) {
                if is_within_grid(neighbor, grid)
                    && grid[neighbor.0 as usize][neighbor.1 as usize] != '#'
                {
                    neighbors_count += 1;
                }
            }
            if neighbors_count > 2 {
                vertices.insert((r, c));
            }
        }
    }
    vertices
}

fn neighbors(coord: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (coord.0 - 1, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0, coord.1 - 1),
        (coord.0, coord.1 + 1),
    ]
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    coord: (i32, i32),
    visited: BTreeSet<(i32, i32)>,
}

impl Position {
    fn new(coord: (i32, i32), visited: BTreeSet<(i32, i32)>) -> Self {
        Self { coord, visited }
    }

    fn inc(&self, coord: (i32, i32)) -> Self {
        let mut next = self.visited.clone();
        next.insert(self.coord);
        Self {
            coord,
            visited: next,
        }
    }
}

struct State {
    position: Position,
    distance: u32,
}

impl State {
    fn new(position: Position, distance: u32) -> Self {
        Self { position, distance }
    }
}

fn find_max_steps(icy: bool, grid: &Vec<Vec<char>>) -> u32 {
    let mut queue = Vec::from([State::new(
        Position::new((0, 1), BTreeSet::from([(0, 1)])),
        0,
    )]); // dfs
    let mut cache: BTreeMap<(i32, i32), u32> = BTreeMap::new();

    let mut max: u32 = 0;
    while let Some(state) = queue.pop() {
        if !is_within_grid(state.position.coord, grid) {
            continue;
        }
        if let Some(c) = cache.get(&state.position.coord) {
            if state.distance <= *c {
                continue;
            }
        }
        cache.insert(state.position.coord, state.distance);
        if state.position.coord.0 == grid.len() as i32 - 1
            && state.position.coord.1 == grid[0].len() as i32 - 2
        {
            if state.distance > max {
                max = state.distance;
            }
            continue;
        }
        for next in get_next(state.position.coord, icy, grid) {
            if state.position.visited.contains(&next) {
                continue;
            }
            let inc = state.position.inc(next);
            queue.push(State::new(inc, state.distance + 1));
        }
    }
    max
}

fn get_next(coord: (i32, i32), icy: bool, grid: &[Vec<char>]) -> Vec<(i32, i32)> {
    match (grid[coord.0 as usize][coord.1 as usize], icy) {
        ('#', _) => vec![],
        ('>', true) => vec![(coord.0, coord.1 + 1)],
        ('<', true) => vec![(coord.0, coord.1 - 1)],
        ('v', true) => vec![(coord.0 + 1, coord.1)],
        ('^', true) => vec![(coord.0 - 1, coord.1)],
        _ => vec![
            (coord.0 - 1, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 + 1),
        ],
    }
}

fn is_within_grid(coord: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    coord.0 >= 0 && coord.0 < grid.len() as i32 && coord.1 >= 0 && coord.1 < grid[0].len() as i32
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}
