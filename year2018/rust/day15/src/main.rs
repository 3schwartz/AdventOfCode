use anyhow::{anyhow, Result};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day15_data.txt")?;

    for line in input.lines() {
        println!("{}", line);
    }

    Ok(())
}

struct Visits {
    first_move: (usize, usize),
    location: (usize, usize),
    elem: Elem,
    steps: u32,
}

impl Visits {
    fn new(elem: Elem, first_move: (usize, usize)) -> Self {
        Self {
            first_move,
            location: first_move.clone(),
            elem,
            steps: 1,
        }
    }

    fn next_visit(&self, new_location: (usize, usize)) -> Visits {
        Visits {
            first_move: self.first_move,
            location: new_location,
            elem: self.elem.clone(),
            steps: self.steps + 1,
        }
    }
}

#[derive(Clone)]
enum Elem {
    Elf(u8),
    Goblin(u8),
    Empty,
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Elf(_), Self::Elf(_)) => true,
            (Self::Goblin(_), Self::Goblin(_)) => true,
            _ => false,
        }
    }
}

impl Elem {

    fn find_opponent<'a>(&self, location: &(usize, usize), map: &'a HashMap<(usize, usize), Elem>) -> Option<&'a Elem> {
        let neighbors = Elem::get_neigbors(&location);
        let mut possible: Option<&Elem> = None;
        for neighbor in &neighbors {
            let Some(n) = map.get(neighbor) else { continue; };
            if !self.is_opposite(n) {
                continue;
            }

            let Some(other) = possible else {
                possible = Some(n);
                continue;
            };

            match possible {
                Some(other) => {
                    // if other
                },
                None => possible = Some(n),
            }
        };
        possible
    }

    fn find_next_location(
        &self,
        location: &(usize, usize),
        map: &HashMap<(usize, usize), Elem>,
    ) -> Result<(usize, usize)> {

        let neighbors = Elem::get_neigbors(location);

        for neighbor in &neighbors {
            let Some(n) = map.get(neighbor) else { continue;};

            if self.is_opposite(n) {
                return Ok(*location);
            }
        }

        let mut visited = HashSet::new();
        let mut queue = neighbors
            .into_iter()
            .map(|n| Visits::new(self.clone(), n))
            .collect::<VecDeque<Visits>>();
        let mut reachable = vec![];
        let mut min_steps = u32::MAX;
        while let Some(next_visit) = queue.pop_front() {
            if next_visit.steps > min_steps {
                continue;
            }

            if !visited.insert(next_visit.location) {
                continue;
            }

            let Some(next) = map.get(&next_visit.location) else {continue;};

            match next {
                Elem::Elf(_) | Elem::Goblin(_) => {
                    if next == self {
                        continue;
                    }
                    min_steps = next_visit.steps;
                    reachable.push(next_visit);
                }
                Elem::Empty => {
                    let next_neighbors = Elem::get_neigbors(&next_visit.location);
                    for next_neighbor in next_neighbors {
                        queue.push_back(next_visit.next_visit(next_neighbor));
                    }
                }
            }
        }

        let Some(mut best_guess) = reachable.pop() else { return Ok(*location)};
        for possible in reachable {
            if possible.first_move.1 > best_guess.first_move.1 {
                continue;
            }
            if possible.first_move.1 < best_guess.first_move.1 {
                best_guess = possible;
                continue;
            }
            if possible.first_move.0 < best_guess.first_move.0 {
                best_guess = possible;
                continue;
            }
        }

        Ok(best_guess.first_move)
    }

    fn is_opposite(&self, other: &Elem) -> bool {
        match (self, other) {
            (Elem::Elf(_), Elem::Goblin(_)) => true,
            (Elem::Goblin(_), Elem::Elf(_)) => true,
            _ => false
        }
    }

    fn get_neigbors(location: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neigh = vec![(location.0, location.1), (location.0, location.1)];
        if location.0 != 0 {
            neigh.push((location.0 - 1, location.1))
        }
        if location.1 != 0 {
            neigh.push((location.0, location.1 - 1))
        }
        neigh
    }
}

struct Game {
    x_max: usize,
    y_max: usize,
    map: HashMap<(usize, usize), Elem>,
}

impl Game {

    fn from(input: String) -> Result<Game> {
        let mut map: HashMap<(usize, usize), Elem> = HashMap::new();

        let mut y_max = 0;
        let mut x_max = 0;
        for (y, line) in input.lines().enumerate() {
            y_max = y;
            for (x, char) in line.chars().enumerate() {
                x_max = x;
                match char {
                    'E' => {
                        map.insert((x, y), Elem::Elf(200));
                    }
                    '#' => (),
                    'G' => {
                        map.insert((x, y), Elem::Goblin(200));
                    }
                    '.' => {
                        map.insert((x, y), Elem::Empty);
                    }
                    _ => return Err(anyhow!("character not known: {}", char)),
                }
            }
        }

        Ok(Game { x_max, y_max, map })
    }

    fn move_elemements(&mut self) -> Result<()> {
        let mut moved_map: HashMap<(usize, usize), Elem> = HashMap::new();

        for (location, elem) in &self.map {
            match elem {
                Elem::Elf(_) | Elem::Goblin(_) => {
                    let next_location = elem.find_next_location(location, &self.map)?;
                    moved_map.insert(next_location, elem.clone());
                }
                Elem::Empty => {
                    if !moved_map.contains_key(location) {
                        moved_map.insert(*location, Elem::Empty);
                    };
                }
            }
        }

        self.map = moved_map;
        Ok(())
    }

    fn attack_with_elements(&mut self) -> Result<()> {

        for x in 0..=self.x_max {
            for y in 0..=self.y_max {
                let location = (x,y);
                let Some(next) = self.map.get(&location) else {continue;};

                let neighbors = Elem::get_neigbors(&location);
                // for neighbor in 
            }
        }

        Ok(())
    }
}

/// Round
/// For each
///     if none in range then move
///         find in range & reachable
///         find nearest
///         chose best using reading order
///     attack
///         find and in range
///         smallest hit point
///         if tie then reading order
///         if dead then out
///
/// Find number of full rounds completed before none elf
///     
///
fn round() -> () {}
// Round
// For each
//  find in range
// find


#[cfg(test)]
mod test {

    enum Foo {
        Bar(u16)
    }

    #[test]
    fn test_enum() -> (){
        let b = Foo::Bar(1);
        let a = Foo::Bar(2);

        match b {
            Foo::Bar(a) => todo!(),
        }

    }
}