use std::{fs, str::FromStr};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day20_data.txt")?;

    let i = find_particle_with_lowest_magnitute(&input)?;

    println!("Part 1: {i}");

    let i = find_particle_with_lowest_manhatten_at(&input, 1_000_000_000_000)?;

    println!("Part 1: {i}");

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

    fn sub(c_1: (i128, i128, i128), c_2: (i128, i128, i128)) -> (i128, i128, i128) {
        (c_1.0 - c_2.0, c_1.1 - c_2.1, c_1.2 - c_2.2)
    }

    fn mul(c_1: (i128, i128, i128), c_2: (i128, i128, i128)) -> (i128, i128, i128) {
        (c_1.0 * c_2.0, c_1.1 * c_2.1, c_1.2 * c_2.2)
    }

    fn dot(c_1: (i128, i128, i128), c_2: (i128, i128, i128)) -> i128 {
        c_1.0 * c_2.0 + c_1.1 * c_2.1 + c_1.2 * c_2.2
    }

    fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else if discriminant == 0.0 {
            let t = -b / (2.0 * a);
            Some((t, t))
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            Some((t1, t2))
        }
    }

    fn find_intersect(&self, other: &Particle) -> Option<f64> {
        let x_diff = Self::sub(self.position, other.position);
        let v_diff = Self::sub(self.velocity, other.velocity);
        let a_diff = Self::sub(self.acceleration, other.acceleration);

        let a = Self::dot(a_diff, a_diff);
        let b = 2 * Self::dot(v_diff, a_diff) + Self::dot(a_diff, x_diff);
        let c = Self::dot(x_diff, x_diff) - Self::dot(v_diff, v_diff);

        Self::solve_quadratic(a as f64, b as f64, c as f64).and_then(|(t1, t2)| {
            if t1 >= 0.0 {
                Some(t1)
            } else if t2 >= 0.0 {
                Some(t2)
            } else {
                None
            }
        })
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
        let particles = make_particles(&input)?;

        // Act
        let intersect = particles[0].find_intersect(&particles[1]);

        // Assert
        assert!(intersect.is_some());
        let actual = intersect.unwrap();
        assert_eq!(actual, 2.0);
        assert!(matches!(intersect, Some(2.0)));
        Ok(())
    }
}
