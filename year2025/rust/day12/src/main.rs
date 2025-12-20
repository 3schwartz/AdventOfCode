use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("../data/day12_data.txt")?;

    let (shapes, regions) = parse(&input)?;

    let mut total = 0;
    for region in regions {
        total += region.can_fit(&shapes) as u32;
        println!("{}x{}", region.x_max, region.y_max)
    }

    println!("Part 1: {}", total);

    Ok(())
}

struct ShapeSet {
    shapes: [Shape; 8],
}

fn parse(input: &str) -> Result<(Vec<ShapeSet>, Vec<Region>)> {
    let mut iter = input.lines();
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    while let Some(line) = iter.next() {
        if line.contains('x') {
            regions.push(Region::new(line)?);
        } else if !line.trim().is_empty() {
            let shape = Shape::new(&mut iter);
            shapes.push(shape);
        }
    }

    Ok((
        shapes.iter().map(|s| s.create_shape_set()).collect(),
        regions,
    ))
}

#[derive(Clone)]
struct Shape {
    points: HashSet<(i32, i32)>,
    x_max: i32,
    y_max: i32,
}

impl Shape {
    fn new<'a, I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut points = HashSet::new();

        for (y, line) in lines.by_ref().enumerate() {
            if line.trim().is_empty() {
                break;
            }
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    points.insert((x as i32, y as i32));
                }
            }
        }
        let x_max = points.iter().map(|(x, _)| *x).max().unwrap_or_default();
        let y_max = points.iter().map(|(_, y)| *y).max().unwrap_or_default();
        Shape {
            points,
            x_max,
            y_max,
        }
    }

    fn create_shape_set(&self) -> ShapeSet {
        let mut s = self.clone();

        let shapes_rotated: [Shape; 4] = std::array::from_fn(|_| {
            s = s.rotate();
            s.clone()
        });
        let shapes_flipped: [Shape; 4] = std::array::from_fn(|_| {
            s = s.flip();
            s.clone()
        });
        let shapes: [Shape; 8] = std::array::from_fn(|i| {
            if i < 4 {
                shapes_rotated[i].clone()
            } else {
                shapes_flipped[i - 4].clone()
            }
        });

        ShapeSet { shapes }
    }

    /// ##
    ///  #
    ///  #
    ///
    ///   #
    /// ###
    fn rotate(&self) -> Shape {
        let mut points = HashSet::new();
        for p in &self.points {
            points.insert((self.y_max - p.1, p.0));
        }
        Self {
            points,
            x_max: self.y_max,
            y_max: self.x_max,
        }
    }
    /// ##
    ///  #
    ///  #
    ///
    /// ##
    /// #
    /// #
    fn flip(&self) -> Shape {
        let mut points = HashSet::new();
        for p in &self.points {
            points.insert((self.x_max - p.0, p.1));
        }
        Self {
            points,
            x_max: self.x_max,
            y_max: self.y_max,
        }
    }

    fn shift(&self, x: i32, y: i32) -> Shape {
        let mut points = HashSet::new();
        for p in &self.points {
            points.insert((p.0 + x, p.1 + y));
        }
        Self {
            points,
            x_max: self.x_max,
            y_max: self.y_max,
        }
    }
}

struct Region {
    x_max: i32,
    y_max: i32,
    shapes_count: Vec<i32>,
}

impl Region {
    fn new(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split(' ').collect();
        let x_y: Vec<&str> = parts[0].split('x').collect();

        let x_max = x_y[0].parse::<i32>()? - 1;
        let y_max = x_y[1]
            .strip_suffix(':')
            .ok_or_else(|| anyhow!("{}", x_y[1]))?
            .parse::<i32>()?
            - 1;

        let shapes_count: Vec<i32> = parts
            .iter()
            .skip(1)
            .map(|v| v.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(Self {
            x_max,
            y_max,
            shapes_count,
        })
    }

    fn can_fit(&self, shapes: &Vec<ShapeSet>) -> bool {
        let mut total_size = 0;
        for (i, s_c) in self.shapes_count.iter().enumerate() {
            let s = &shapes[i];
            total_size += s.shapes[0].points.len() as i32 * *s_c
        }
        if total_size > (self.x_max + 1) * (self.y_max + 1) {
            return false;
        }
        self.can_fit_dfs(HashSet::new(), 0, &self.shapes_count, shapes, (0, 0))
    }

    fn can_fit_dfs(
        &self,
        visited: HashSet<(i32, i32)>,
        idx: usize,
        shapes_count: &Vec<i32>,
        shapes_sets: &Vec<ShapeSet>,
        position: (i32, i32),
    ) -> bool {
        if idx == shapes_count.len() {
            return true;
        }

        let shape_count = shapes_count[idx];
        if shape_count == 0 {
            return self.can_fit_dfs(visited, idx + 1, shapes_count, shapes_sets, position);
        }
        let shape_set = &shapes_sets[idx];
        let mut shapes_count_updated = shapes_count.clone();
        shapes_count_updated[idx] -= 1;

        for x in position.0..=self.x_max {
            for y in 0..=self.y_max {
                if (x, y) < position {
                    continue;
                }
                for shape in &shape_set.shapes {
                    if x + shape.x_max > self.x_max {
                        break;
                    }
                    if y + shape.y_max > self.y_max {
                        break;
                    }
                    let shifted = shape.shift(x, y);

                    if !visited.is_disjoint(&shifted.points) {
                        continue;
                    }

                    let mut visited_updated = visited.clone();
                    visited_updated.extend(shifted.points);
                    let next = self.can_fit_dfs(
                        visited_updated,
                        idx,
                        &shapes_count_updated,
                        shapes_sets,
                        (x, y),
                    );
                    if next {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ##
    ///  #
    ///  #
    ///
    ///   #
    /// ###
    #[test]
    fn test_when_rotate_then_correct() -> Result<()> {
        // Arrange
        let input = r#"0;
##
 #
 #
"#;
        let s = Shape::new(&mut input.lines().skip(1));
        let points_before = HashSet::from([(0, 0), (1, 0), (1, 1), (1, 2)]);

        let points_after = HashSet::from([(2, 0), (0, 1), (1, 1), (2, 1)]);

        // Act
        let r = s.rotate();

        // Assert
        assert_eq!(s.points, points_before);
        assert_eq!(s.x_max, 1);
        assert_eq!(s.y_max, 2);

        assert_eq!(r.points, points_after);
        assert_eq!(r.x_max, 2);
        assert_eq!(r.y_max, 1);
        Ok(())
    }

    /// ##
    ///  #
    ///  #
    ///
    /// ##
    /// #
    /// #
    #[test]
    fn test_when_flip_then_correct() -> Result<()> {
        // Arrange
        let input = r#"0;
##
 #
 #
"#;
        let s = Shape::new(&mut input.lines().skip(1));
        let points_before = HashSet::from([(0, 0), (1, 0), (1, 1), (1, 2)]);

        let points_after = HashSet::from([(0, 0), (1, 0), (0, 1), (0, 2)]);

        // Act
        let r = s.flip();

        // Assert
        assert_eq!(s.points, points_before);
        assert_eq!(s.x_max, 1);
        assert_eq!(s.y_max, 2);

        assert_eq!(r.points, points_after);
        assert_eq!(r.x_max, 1);
        assert_eq!(r.y_max, 2);
        Ok(())
    }

    #[test]
    fn test_when_shape_can_fit_then_return_true() -> Result<()> {
        // Arrange
        let input = read_to_string("../../data/day12_data_test.txt")?;

        let regions = vec![
            (Region::new("4x4: 0 0 0 0 2 0")?, true),
            (Region::new("12x5: 1 0 1 0 2 2")?, true),
            (Region::new("12x5: 1 0 1 0 3 2")?, false),
        ];
        let (shapes, _) = parse(&input)?;

        for (region, expected) in regions {
            // Act
            let can_fit = region.can_fit(&shapes);

            // Assert
            assert_eq!(can_fit, expected);
        }

        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        // Arrannge
        let input = read_to_string("../../data/day12_data_test.txt")?;

        // Act
        let (shapes, regions) = parse(&input)?;

        let mut total = 0;
        for region in regions {
            total += region.can_fit(&shapes) as u32;
        }

        // Assert
        assert_eq!(total, 2);
        Ok(())
    }

    #[test]
    fn test_count() {
        let r = r#"....AAAFFE.E
.BBBAAFFFEEE
DDDBAAFFCECE
DBBB....CCC.
DDD.....C.C."#;
        let mut count = 0;
        for line in r.lines() {
            for c in line.chars() {
                if c != '.' {
                    count += 1;
                }
            }
        }

        assert_eq!(42, count);
    }
}
