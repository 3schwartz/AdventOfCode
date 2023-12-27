use anyhow::{anyhow, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../../data/day13_data.txt")?;
    let squares = get_squares(&input);

    // let mut total = 0;
    // for s in &squares {
    //     let square = Square::new(s, None);
    //     total += square.find_reflection()?;
    // }

    let mut total = 0;
    // total = 0;
    for s in &squares {
        let square = Square::new(s, None);
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
    input: String,
}

impl Square {
    fn find_smudge(&self) -> Result<i32> {
        let own_reflection = self.find_reflection()?;
        for row in 0..self.row_max {
            for column in 0..self.column_max {
                let s = Square::new(&self.input, Some((row as usize, column as usize)));
                if let Some(r) = s.reflection(Some(own_reflection)) {
                    return Ok(r);
                }
            }
        }
        Err(anyhow!("{:?}", self))
    }

    fn reflection(&self, avoid: Option<i32>) -> Option<i32> {
        if let Some(column) = self.range_equals(self.column_max, &self.columns) {
            let result = column + 1;
            match avoid {
                Some(a) => {
                    if a != result {
                        return Some(result);
                    };
                }
                None => return Some(result),
            }
        }
        if let Some(row) = self.range_equals(self.row_max, &self.rows) {
            let result = (row + 1) * 100;
            match avoid {
                Some(a) => {
                    if a != result {
                        return Some(result);
                    };
                }
                None => return Some(result),
            }
        }
        None
    }

    fn find_reflection(&self) -> Result<i32> {
        match self.reflection(None) {
            Some(r) => Ok(r),
            None => Err(anyhow!("{:?}", self)),
        }
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

    fn new(input: &str, substitute: Option<(usize, usize)>) -> Square {
        let mut rows = Vec::new();
        let mut columns = Vec::new();

        let mut row_max: i32 = 0;
        let mut column_max: i32 = 0;
        for (x, line) in input.lines().enumerate() {
            row_max = x as i32;
            let mut row = Vec::new();
            for (y, c) in line.chars().enumerate() {
                column_max = y as i32;
                let cc = match substitute {
                    Some((x_s, y_s)) => {
                        if x_s == x && y_s == y && c == '.' {
                            '#'
                        } else if x_s == x && y_s == y && c == '#' {
                            '.'
                        } else {
                            c
                        }
                    }
                    None => c,
                };
                row.push(cc);
                if x == 0 {
                    columns.push(Vec::from([cc]))
                }
                columns[y].push(cc);
            }
            rows.push(row);
        }
        Square {
            row_max,
            column_max,
            rows,
            columns,
            input: input.to_string(),
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
            let square = Square::new(s, None);
            total += square.find_reflection()?;
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
            let square = Square::new(s, None);
            total += square.find_smudge()?;
        }
        // Assert
        assert_eq!(400, total);
        Ok(())
    }
}
