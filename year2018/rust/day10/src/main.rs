use std::{collections::HashSet, fs::{self, File}, io::{Write, self}};

fn main() {
    let input = fs::read_to_string("../data/day10_data.txt")
    .expect("file");

    let mut sky = Sky::new(input);
    match sky.find_message() {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    };
}

fn generate_point(input: &str) -> (i32, i32) {
    let numbers = input.split(", ")
        .map(|n| n
                .trim()
                .parse::<i32>().expect("not able to parse"))
        .collect::<Vec<i32>>();
    return (numbers[0], numbers[1]);
}

#[derive(Debug)]
struct Record {
    cx: i32,
    cy: i32,
    vx: i32,
    vy: i32
}

impl Record {
    fn new(line : &str) -> Self {
        let sub = &line[10..(line.len()-1)]
            .split("> velocity=<")
            .collect::<Vec::<&str>>();

        let (cx,cy) = generate_point(sub[0]);
        let (vx, vy) = generate_point(sub[1]);

        return Self {cx, cy, vx, vy }
    }

    fn move_velocity(&mut self) {
        self.cx += self.vx;
        self.cy += self.vy;
    }

    fn rollback(&mut self) {
        self.cx -= self.vx;
        self.cy -= self.vy;
    }
}

struct Sky {
    records: Vec<Record>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Sky {
    /// Doesn't set min and max before first update run.
    fn new(input: String) -> Self {
        let mut records = Vec::new();

        for line in input.lines() {
            let record = Record::new(line);
            records.push(record);
        }

        return Self { records, x_min: 0, x_max: 0, y_min: 0, y_max: 0 }
    }

    fn find_message(&mut self) -> Result<(), io::Error> {
        let mut distance: i32 = 0;
        for i in 0..1_000_000 {
            self.update();
            let x_abs = (self.x_max - self.x_min).abs();
            let y_abs = (self.y_max - self.y_min).abs();
            if distance < x_abs + y_abs && i != 0 {
                self.rollback();
                self.print()?;
                println!("After: {}", i);
                break;
            }
            distance = x_abs + y_abs;
        }
        return Ok(());
    }

    fn update(&mut self) {
        self.x_max = i32::MIN;
        self.x_min = i32::MAX;
        self.y_max = i32::MIN;
        self.y_min = i32::MAX;
        for record in &mut self.records {
            record.move_velocity();
            if record.cx > self.x_max {
                self.x_max = record.cx;
            }
            if record.cx < self.x_min {
                self.x_min = record.cx;
            }
            if record.cy > self.y_max {
                self.y_max = record.cy;
            }
            if record.cy < self.y_min {
                self.y_min = record.cy;
            }
        }
    }
    fn rollback(&mut self) {
        for record in &mut self.records {
            record.rollback();
        }
    }

    fn print(&self) -> Result<(), io::Error>{
        let sky : &HashSet<Coord>= &self.records
            .iter()
            .map(|r| Coord::new(r))
            .collect();
        let mut file = File::create("result.txt")?;
        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                if sky.contains(&Coord::new_simple(x, y)) {
                    print!("#");
                    file.write(b"#")?;
                    continue;
                }
                file.write(b".")?;
                print!(".");
            }
            file.write(b"\n")?;
            println!()
        }
        println!();
        println!("---------------------------------");
        println!();
        return Ok(());
    }
}


#[derive(Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn new_simple(x: i32, y:i32) -> Self {
        return Self { x, y }
    }
    fn new(record : &Record) -> Self {
        return Self { x: record.cx, y: record.cy }
    }
}

#[cfg(test)]
mod test {
    use crate::Sky;
    use std::fs;

    #[test]
    fn test_part1(){
        let input = fs::read_to_string("../data/day10_test_data.txt")
            .expect("file");
            
        let mut sky = Sky::new(input);
        match sky.find_message() {
            Ok(_) => {},
            Err(err) => println!("{}", err),
        };
    }
}