use std::{fs, collections::{HashMap, BTreeSet, HashSet}, hash::{Hash, Hasher}};
use anyhow::{Result, anyhow};

fn initialize_cities<'a>(input: &'a str) -> Result<HashMap<&'a str, HashMap<&'a str, u32>>> {
    let mut cities = HashMap::new();

    for line in input.lines() {

        let info = line.split(" = ").collect::<Vec<&str>>();
        let distance = info 
            .get(1)
            .ok_or(anyhow!("error getting distance: {:?}", info))?
            .parse::<u32>()?;
        let mut to_from = info
            .get(0)
            .ok_or(anyhow!("error getting to, from: {:?}", info))?
            .split(" to ");
        let to = to_from.next()
            .ok_or(anyhow!("error getting to: {:?}", info))?;
        let from = to_from.next()
            .ok_or(anyhow!("error getting from: {:?}", info))?;

        let to_state = cities.entry(to)
            .or_insert_with(|| HashMap::new());
        to_state.insert(from, distance);

        let from_state = cities.entry(from)
            .or_insert_with(|| HashMap::new());
        from_state.insert(to, distance);
    }

    Ok(cities)
}

#[derive(Debug,PartialEq, Eq, Clone)]
struct State<'a> {
    city: &'a str,
    visited: BTreeSet<&'a str>
}

impl<'a> State<'a> {
    fn new(start: &'a str) -> Self {
        Self { city: start, visited: BTreeSet::from([start]) }
    }

    fn create_from_this(&self, to: &'a str) -> State<'a> {
        let mut visited = self.visited.clone();
        visited.insert(to);
        return State { city: to, visited }
    }
}

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.city.hash(hasher);
        for city in &self.visited {
            (**city).hash(hasher);
        }
    }
}


fn find_route<'a>(cities: &HashMap<&'a str, HashMap<&'a str, u32>>, use_shortest: bool, debug: bool) -> Result<u32>{
    let mut queue = HashMap::new();
    for city in cities.keys() {
        let states = queue.entry(1)
            .or_insert_with(|| Vec::new());
        states.push(State::new(*city));
    }

    let final_count = cities.len();
    let mut distance = 0;
    let mut visited: HashSet<State> = HashSet::new();
    let mut best_route = if use_shortest {u32::MAX} else {u32::MIN};

    loop {
        distance += 1;
        if queue.is_empty() {
            break;
        }
        if !queue.contains_key(&distance) {
            continue;
        }
        let (_, states) = queue
            .remove_entry(&distance)
            .ok_or(anyhow!("error getting entry at distance: {}", distance))?;

        for state in states {
            if use_shortest && visited.contains(&state) {
                continue;
            }

            let neighbors = cities.get(state.city)
                .ok_or(anyhow!("missing city: {}", state.city))?;

            for (neighbor, neighbor_distance) in neighbors{
                if state.visited.contains(neighbor) {
                    continue;
                }
                let updated_state = state.create_from_this(&neighbor);
                let updated_distance = neighbor_distance + distance;
                
                if use_shortest && updated_distance >= best_route {
                    continue;
                }
                if use_shortest && updated_state.visited.len() == final_count && updated_distance < best_route {
                    if debug {println!("Distance: {}, State: {:?}", updated_distance, updated_state);}
                    best_route = updated_distance;
                }

                if !use_shortest && updated_state.visited.len() == final_count && updated_distance > best_route {
                    if debug {println!("Distance: {}, State: {:?}", updated_distance, updated_state);}
                    best_route = updated_distance;
                }

                if updated_state.visited.len() == final_count {
                    continue;
                }

                let next = queue
                    .entry(updated_distance)
                    .or_insert_with(|| Vec::new());

                next.push(updated_state);
            }

            if use_shortest {visited.insert(state);}
        }
    };
    Ok(best_route-1)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day9_data.txt")?;
    let cities = initialize_cities(&input)?;
    let shortest = find_route(&cities, true, false)?; 

    println!("Part 1: {}", shortest);

    let longest = find_route(&cities, false, false)?; 

    println!("Part 2: {}", longest);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day9_test_data.txt")?;
        let cities = initialize_cities(&input)?;
        
        // Act
        let shortest = find_route(&cities, true, true)?; 
        
        // Assert
        assert_eq!(shortest, 605);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day9_test_data.txt")?;
        let cities = initialize_cities(&input)?;
        
        // Act
        let shortest = find_route(&cities, false, true)?; 
        
        // Assert
        assert_eq!(shortest, 982);
        Ok(())
    }
}