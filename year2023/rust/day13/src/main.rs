use anyhow::{anyhow, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day13_data.txt")?;
    let squares = get_squares(&input);

    let mut total = 0;
    for s in &squares {
        let square = Square::new(s);
        let reflection = square.find_reflection()?;
        total += reflection.0 + reflection.1;
    }

    println!("Part 1: {}", total);

    let mut total = 0;
    for s in &squares {
        let mut square = Square::new(s);
        total += square.find_smudge()?;
    }

    println!("Part 2: {}", total);

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
    fn find_smudge(&mut self) -> Result<i32> {
        let (row_own, column_own) = self.find_reflection()?;
        for row in 0..=self.row_max {
            for column in 0..=self.column_max {
                let r_s = self.rows[row as usize][column as usize];
                let c_s = self.columns[column as usize][row as usize];
                if r_s != c_s {
                    return Err(anyhow!("({},{}) not same char in {:?}", row, column, self));
                }
                let opposite = Square::get_opposite(r_s);
                self.rows[row as usize][column as usize] = opposite;
                self.columns[column as usize][row as usize] = opposite;

                if let Some(column_r) =
                    self.range_equals(self.column_max, &self.columns, column_own - 1)
                {
                    return Ok(column_r + 1);
                }
                if let Some(row_r) = self.range_equals(self.row_max, &self.rows, row_own / 100 - 1)
                {
                    return Ok((row_r + 1) * 100);
                }

                self.rows[row as usize][column as usize] = r_s;
                self.columns[column as usize][row as usize] = c_s;
            }
        }
        Err(anyhow!("No solution for square {:?}", self))
    }

    fn get_opposite(c: char) -> char {
        if c == '#' {
            return '.';
        }
        '#'
    }

    fn find_reflection(&self) -> Result<(i32, i32)> {
        if let Some(column) = self.range_equals(self.column_max, &self.columns, -1) {
            return Ok((0, column + 1));
        }
        if let Some(row) = self.range_equals(self.row_max, &self.rows, -1) {
            return Ok(((row + 1) * 100, 0));
        }
        Err(anyhow!("{:?}", self))
    }

    fn range_equals(&self, max: i32, range: &[Vec<char>], not: i32) -> Option<i32> {
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
            if is_match && line != not {
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
                } else {
                    columns[y].push(c);
                }
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
            let reflection = square.find_reflection()?;
            total += reflection.0 + reflection.1;
        }
        // Assert
        assert_eq!(405, total);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day13_data_test.txt")?;

        // Act
        let squares = get_squares(&input);

        let mut total = 0;
        for s in squares {
            let mut square = Square::new(s);
            total += square.find_smudge()?;
        }
        // Assert
        assert_eq!(400, total);
        Ok(())
    }
}
