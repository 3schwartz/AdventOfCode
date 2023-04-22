use core::fmt;
use std::{error::Error, fs, collections::HashMap, str::Lines};

#[derive(Debug)]
enum AoCError {
    LineNotCorrectLength(String),
    MissingKey(String),
    UnknownSignal(String),
    NoResult
}

impl fmt::Display for AoCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AoCError::LineNotCorrectLength(msg) => write!(f, "{}", msg),
            AoCError::MissingKey(msg) => write!(f, "{}", msg),
            AoCError::UnknownSignal(msg) => write!(f, "{}", msg),
            AoCError::NoResult => write!(f, "No result")
        }
    }
}

impl Error for AoCError{}

type AoCResult<T> = Result<T, AoCError>;


fn main() -> AoCResult<()> {
    let input = fs::read_to_string("../data/day7_data.txt").unwrap();
    let lines = input.lines();
    let mut signals: HashMap<&str, u16> = HashMap::new();

    find_signal(&mut signals, &lines)?;

    let a = signals.get("a").ok_or(AoCError::NoResult)?;
    println!("Part 1: {}", a);

    let mut signals_part2: HashMap<&str, u16> = HashMap::from([("b", *a)]);
    
    find_signal(&mut signals_part2, &lines)?;
    let a_part2 = signals_part2.get("a").ok_or(AoCError::NoResult)?;
    println!("Part 2: {}", a_part2);

    Ok(())
}


fn find_signal<'a>(signals: &mut HashMap<&'a str, u16>, lines: &'a Lines) -> AoCResult<()> {
    let mut count = 0;

    while !signals.contains_key("a") {
        
        count+=1;

        for line in lines.clone() {
            let options: Vec<&str> = line.split_whitespace().collect();
    
            let last = options.last().ok_or(AoCError::LineNotCorrectLength(line.to_string()))?;
            if signals.contains_key(last) {
                continue;
            }
    
            let signal = options[0].parse::<u16>();
    
            if options.len() == 3  && signal.is_ok() {
                signals.insert(options[2], signal.unwrap());
                continue;
            }
    
            if options.len() == 3  && signals.contains_key(options[0]) {
                match signals.get(options[0]) {
                    Some(value) => {
                        signals.insert(options[2], *value);
                    },
                    None => (),
                };
                continue;
            }
            
            if options.len() == 4 && signals.contains_key(options[1]) {
                let value = signals.get(&options[1]).ok_or(AoCError::MissingKey(line.to_string()))?;
                signals.insert(options[3], !value);
                continue;
            }
    
            let first = signals.get(options[0])
                .map(|v| *v)
                .or_else(|| options[0].parse::<u16>().ok());
    
            let second = signals.get(options[2])
                .map(|v| *v)
                .or_else(|| options[2].parse::<u16>().ok());
    
            if first.is_none() || second.is_none() {
                continue;
            }
            
            let f = first.unwrap();
            let s = second.unwrap();
    
            let new_signal = match options[1] {
                "AND" => f & s,
                "LSHIFT" => f << s,
                "RSHIFT" => f >> s,
                "OR" => f | s,
                _ => return Err(AoCError::UnknownSignal(options[1].to_string()))  
            };
            signals.insert(options[4], new_signal);
        }
    }
    println!("{count}");

    Ok(())
}