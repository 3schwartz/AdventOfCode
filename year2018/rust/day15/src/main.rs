use anyhow::{anyhow, Result};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day15_data.txt")?;
    let mut game = Game::from(input)?;

    let part_1 = game.play(false)?;

    println!("Part 1: {}", part_1);
    Ok(())
}

struct Visits {
    first_move: (usize, usize),
    location: (usize, usize),
    steps: u32,
}

impl Visits {
    fn new(first_move: (usize, usize)) -> Self {
        Self {
            first_move,
            location: first_move.clone(),
            steps: 1,
        }
    }

    fn next_visit(&self, new_location: (usize, usize)) -> Visits {
        Visits {
            first_move: self.first_move,
            location: new_location,
            steps: self.steps + 1,
        }
    }
}

#[derive(Clone, Debug)]
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
            (Self::Empty, Self::Empty) => true,
            _ => false,
        }
    }
}

impl Elem {
    fn damage(&self, damage: u8) -> Elem {
        let mut cloned = self.clone();
        match cloned {
            Elem::Elf(ref mut life) | Elem::Goblin(ref mut life) => {
                *life = life.saturating_sub(damage);
                if *life == 0 {
                    cloned = Elem::Empty;
                }
            }
            Elem::Empty => (),
        };
        cloned
    }

    fn opponents_exist(&self, map: &HashMap<(usize, usize), Elem>) -> bool {
        if self == &Elem::Empty {
            return true;
        }
        for (_, elem) in map {
            if self.is_opposite(elem) {
                return true;
            }
        }
        return false;
    }

    fn find_opponent<'a>(
        &self,
        location: &(usize, usize),
        map: &'a HashMap<(usize, usize), Elem>,
    ) -> Option<(&'a Elem, (usize, usize))> {
        if self == &Elem::Empty {
            return None;
        }

        let neighbors = Elem::get_neigbors(&location);
        let mut possible: Option<(&Elem, (usize, usize))> = None;
        for neighbor in &neighbors {
            let Some(n) = map.get(neighbor) else { continue; };
            if !self.is_opposite(n) {
                continue;
            }

            let Some((possible_elem, possible_location)) = possible else {
                possible = Some((n, *neighbor));
                continue;
            };

            let Some(possible_life) = possible_elem.get_life() else {continue;};
            let Some(neighbor_life) = n.get_life() else {continue;};

            if possible_life < neighbor_life {
                continue;
            }
            if neighbor_life < possible_life {
                possible = Some((n, *neighbor));
                continue;
            }
            if possible_location.1 < neighbor.1 {
                continue;
            }
            if neighbor.1 < possible_location.1 {
                possible = Some((n, *neighbor));
                continue;
            }
            if neighbor.0 < possible_location.0 {
                possible = Some((n, *neighbor));
            }
        }
        possible
    }

    fn get_life(&self) -> Option<&u8> {
        match self {
            Elem::Elf(life) | Elem::Goblin(life) => Some(life),
            Elem::Empty => None,
        }
    }

    /// Returns next location.
    /// If no target exist then return None
    fn find_next_location(
        &self,
        location: &(usize, usize),
        map: &HashMap<(usize, usize), Elem>,
    ) -> Option<(usize, usize)> {
        if self == &Elem::Empty {
            return Some(*location);
        }
        if !self.opponents_exist(map) {
            return None;
        }

        let neighbors = Elem::get_neigbors(location);

        for neighbor in &neighbors {
            let Some(n) = map.get(neighbor) else { continue;};

            if self.is_opposite(n) {
                return Some(*location);
            }
        }

        let mut visited = HashSet::new();
        let mut queue = neighbors
            .into_iter()
            .map(|n| Visits::new(n))
            .collect::<VecDeque<Visits>>();
        let mut reachable = vec![];
        let mut min_steps = u32::MAX;
        while let Some(next_visit) = queue.pop_front() {
            if next_visit.steps > min_steps {
                continue;
            }

            if !visited.insert((next_visit.first_move, next_visit.location)) {
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

        let Some(mut best_guess) = reachable.pop() else { return Some(*location)};
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

        Some(best_guess.first_move)
    }

    fn is_opposite(&self, other: &Elem) -> bool {
        match (self, other) {
            (Elem::Elf(_), Elem::Goblin(_)) => true,
            (Elem::Goblin(_), Elem::Elf(_)) => true,
            _ => false,
        }
    }

    fn get_neigbors(location: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neigh = vec![(location.0 + 1, location.1), (location.0, location.1 + 1)];
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

    fn all_elves_dead(&self) -> bool {
        for (_, v) in &self.map {
            match v {
                Elem::Elf(_) => return false,
                _ => (),
            }
        }
        return true;
    }

    fn all_goblins_dead(&self) -> bool {
        for (_, v) in &self.map {
            match v {
                Elem::Goblin(_) => return false,
                _ => (),
            }
        }
        return true;
    }

    fn play(&mut self, debug: bool) -> Result<u32> {
        let mut round = 1;
        loop {
            let mut visited = HashSet::new();
            let mut early_done = false;
            for y in 0..=self.y_max {
                if early_done {
                    break;
                }
                for x in 0..=self.x_max {
                    let location = (x, y);
                    if visited.contains(&location) {
                        continue;
                    }

                    let Some(elem) = self.map.remove(&location) else {continue;};

                    let Some(next_location) = elem.find_next_location(&location, &self.map) else {
                        round -=1;
                        self.map.insert(location, elem);
                        early_done = true;
                        break;
                    };

                    self.map.insert(next_location, elem.clone());
                    if !self.map.contains_key(&location) {
                        self.map.insert(location, Elem::Empty);
                    };

                    match elem {
                        Elem::Elf(_) | Elem::Goblin(_) => {
                            visited.insert(next_location);
                        }
                        Elem::Empty => (),
                    };

                    let Some((opponent, opponent_location)) = elem.find_opponent(&next_location, &self.map) else {continue;};
                    let opppnent_damaged = opponent.damage(3);
                    self.map.insert(opponent_location, opppnent_damaged);
                }
            }

            if debug {
                self.print_map(round);
            }

            if self.all_elves_dead() || self.all_goblins_dead() {
                break;
            }
            round += 1;
        }
        let hit_point_sum = self.get_hit_point_sum();
        let outcome = round * hit_point_sum;
        Ok(outcome)
    }

    fn get_hit_point_sum(&self) -> u32 {
        let hit_point_sum: u32 = self
            .map
            .values()
            .map(|v| match v {
                Elem::Goblin(life) | Elem::Elf(life) => *life as u32,
                _ => 0,
            })
            .sum::<u32>();
        hit_point_sum
    }

    fn print_map(&self, round: u32) -> () {
        println!("---------------------");
        println!("Rounds: {}", round);
        println!("---------------------");
        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                let location = (x, y);
                match self.map.get(&location) {
                    None => print!("#"),
                    Some(el) => match el {
                        Elem::Elf(_) => print!("E"),
                        Elem::Goblin(_) => print!("G"),
                        Elem::Empty => print!("."),
                    },
                }
            }
            println!("")
        }
    }
}

#[cfg(test)]
mod test {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let data = vec![(27730, "1"), (36334, "2"), (39514, "3"), (27755, "4")];

        for (expected, file) in data {
            let input = fs::read_to_string(format!("../../data/day15_test{}_data.txt", file))?;
            // let input = fs::read_to_string(format!("../data/day15_test{}_data.txt", file))?;

            // Act
            let mut game = Game::from(input)?;

            let part_1 = game.play(false)?;

            // Assert
            assert_eq!(expected, part_1);
        }

        Ok(())
    }
}
