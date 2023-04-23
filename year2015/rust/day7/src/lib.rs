use core::fmt;
use std::{error::Error, collections::HashMap};

pub type AoCResult<T> = Result<T, AoCError>;

#[derive(Debug)]
pub enum AoCError {
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

pub fn find_signal<'a, T>(signals: &mut HashMap<&'a str, u16>, lines: &T, debug: bool) -> AoCResult<()> 
    where 
        T: IntoIterator<Item = &'a str> + Clone
{
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
    if debug {
        println!("{count}");
    }
    

    Ok(())
}