use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day20_data.txt")?;

    let i = find_particle_with_lowest_magnitute(&input)?;

    println!("Part 1: {i}");

    let i = find_particle_with_lowest_manhatten_at(&input, 1_000_000_000_000)?;

    println!("Part 1: {i}");

    let remaining = find_particles_remaining(&input, 10_000)?;

    println!("Part 2: {remaining}");

    Ok(())
}

fn find_particle_with_lowest_magnitute(input: &str) -> Result<usize> {
    let particles = make_particles(input)?;

    let (index, _) = particles
        .iter()
        .enumerate()
        .min_by(|(_, p1), (_, p2)| {
            p1.get_acceleration_magnitude()
                .partial_cmp(&p2.get_acceleration_magnitude())
                .unwrap()
        })
        .ok_or_else(|| anyhow!("not able to find min magnitude"))?;

    Ok(index)
}

fn make_particles(input: &str) -> Result<Vec<Particle>> {
    let mut particles = vec![];
    for line in input.lines() {
        particles.push(Particle::from_str(line)?);
    }
    Ok(particles)
}

fn find_particle_with_lowest_manhatten_at(input: &str, t: i128) -> Result<usize> {
    let particles = make_particles(input)?;

    let (index, _) = particles
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| p.find_manhatten_at(t))
        .ok_or_else(|| anyhow!("not able to find min manhattan"))?;

    Ok(index)
}

fn find_particles_remaining(input: &str, after: i128) -> Result<usize> {
    let inital = make_particles(input)?;

    let mut particles: Vec<&Particle> = inital.iter().collect();
    for t in 0..after {
        let mut positions: HashMap<(i128, i128, i128), Vec<usize>> = HashMap::new();

        for (i, particle) in particles.iter().enumerate() {
            let p = particle.find_position_at(t);
            positions
                .entry(p)
                .and_modify(|e| {
                    e.push(i);
                })
                .or_insert_with(|| vec![i]);
        }

        let to_remove: Vec<usize> = positions
            .values()
            .filter(|i| i.len() > 1)
            .flat_map(|i| i.iter().copied())
            .collect();

        particles = particles
            .iter()
            .enumerate()
            .filter(|(i, _)| !to_remove.contains(i))
            .map(|(_, p)| *p)
            .collect();
    }

    Ok(particles.len())
}

struct Particle {
    position: (i128, i128, i128),
    velocity: (i128, i128, i128),
    acceleration: (i128, i128, i128),
}

impl Particle {
    fn parse_coord(input: &str) -> Result<(i128, i128, i128)> {
        let mut i = &input[1..];
        i = i.trim_start_matches("=<");
        i = i.trim_end_matches('>');
        let parts: Vec<&str> = i.split(',').collect();
        if parts.len() != 3 {
            return Err(anyhow!("cooord: s',' wrong lenght: {:?}", parts));
        }
        let c = parts
            .iter()
            .map(|n| n.trim().parse())
            .collect::<Result<Vec<i128>, _>>()?;
        Ok((c[0], c[1], c[2]))
    }

    fn find_manhatten_at(&self, t: i128) -> i128 {
        let p = self.find_position_at(t);
        Self::manhattan_distance(p)
    }

    fn find_position_at(&self, t: i128) -> (i128, i128, i128) {
        let calculate = |pos, vel, acc| pos + vel * t + acc * (t * (t + 1)) / 2;
        (
            calculate(self.position.0, self.velocity.0, self.acceleration.0),
            calculate(self.position.1, self.velocity.1, self.acceleration.1),
            calculate(self.position.2, self.velocity.2, self.acceleration.2),
        )
    }

    fn manhattan_distance(coord: (i128, i128, i128)) -> i128 {
        coord.0.abs() + coord.1.abs() + coord.2.abs()
    }

    fn get_acceleration_magnitude(&self) -> f64 {
        ((self.acceleration.0.pow(2) + self.acceleration.1.pow(2) + self.acceleration.2.pow(2))
            as f64)
            .sqrt()
    }
}

impl FromStr for Particle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        if parts.len() != 3 {
            return Err(anyhow!("', ' wrong lenght: {:?}", parts));
        }
        let p = parts
            .iter()
            .map(|p| Particle::parse_coord(p))
            .collect::<Result<Vec<(i128, i128, i128)>, _>>()?;
        Ok(Self {
            position: p[0],
            velocity: p[1],
            acceleration: p[2],
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day20_test_data.txt")?;

        // Act
        let i = find_particle_with_lowest_magnitute(&input)?;

        // Assert
        assert_eq!(i, 0);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day20_test2_data.txt")?;

        // Act
        let remaining = find_particles_remaining(&input, 1_000_000)?;

        // Assert
        assert_eq!(remaining, 1);
        Ok(())
    }
}
