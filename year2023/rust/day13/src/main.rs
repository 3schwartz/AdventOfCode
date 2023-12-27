use anyhow::{anyhow, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day13_data.txt")?;
    let squares = get_squares(&input);

    let mut total = 0;
    for s in squares {
        let square = Square::new(s);
        total += square.find_reflection()?;
    }

    println!("Part 1: {}", total);

    Ok(())
}

#[derive(Debug)]
struct Square {
    row_max: i32,
    column_max: i32,
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>,
}

impl Square {
    fn find_reflection(&self) -> Result<i32> {
        if let Some(column) = self.range_equals(self.column_max, &self.columns) {
            return Ok(column + 1);
        }
        if let Some(row) = self.range_equals(self.row_max, &self.rows) {
            return Ok((row + 1)* 100);
        }
        Err(anyhow!("{:?}", self))
    }

    fn range_equals(&self, max: i32, range: &Vec<Vec<char>>) -> Option<i32> {
        for i in 0..max {
            let line = i;
            let mut inc = 0;
            let mut is_match = false;
            loop {
                let left_or_up = line - inc;
                let right_or_down = line + inc + 1;

                if left_or_up < 0 || right_or_down > max {
                    is_match = true;
                    break;
                }

                let equals = range[left_or_up as usize] == range[right_or_down as usize];

                if !equals {
                    break;
                }
                inc += 1;
            }
            if is_match {
                return Some(line);
            }
        }
        None
    }

    fn new(input: &str) -> Square {
        let mut rows = Vec::new();
        let mut columns = Vec::new();

        let mut row_max: i32 = 0;
        let mut column_max: i32 = 0;
        for (x, line) in input.lines().enumerate() {
            row_max = x as i32;
            let mut row = Vec::new();
            for (y, c) in line.chars().enumerate() {
                column_max = y as i32;
                row.push(c);
                if x == 0 {
                    columns.push(Vec::from([c]))
                }
                columns[y].push(c);
            }
            rows.push(row);
        }
        Square {
            row_max,
            column_max,
            rows,
            columns,
        }
    }
}

fn get_squares(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day13_data_test.txt")?;

        // Act
        let squares = get_squares(&input);

        let mut total = 0;
        for s in squares {
            let square = Square::new(s);
            total += square.find_reflection()?;
        }
        // Assert
        assert_eq!(405, total);
        Ok(())
    }
}
